#![allow(non_snake_case)]

use crate::base10;
use crate::compress::decompress;
use crate::utils::hash;
use crate::utils::log;
use base64::{prelude::BASE64_STANDARD, Engine as _};
use image::{DynamicImage, ImageBuffer, RgbaImage};
use quircs::Quirc;
use std::cmp::Ordering;
use std::collections::HashSet;
use std::mem::take;
use wasm_bindgen::prelude::*;

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
enum Msg {
    Length(usize),
    Name(String),
    Hash(String),
    Piece(String, String),
}

impl Msg {
    fn new(chunk: String) -> Self {
        let split = chunk.split(':').collect::<Vec<&str>>();
        let i = split[0];
        let data = split[1];
        match i {
            "LEN" => Msg::Length(data.to_string().parse::<usize>().unwrap()),
            "NAME" => Msg::Name(data.to_string()),
            "HASH" => Msg::Hash(data.to_string()),
            _ => Msg::Piece(i.to_string(), data.to_string()),
        }
    }
}

#[derive(Default)]
struct Initted {
    received_msgs: HashSet<Msg>,
    file_name: Option<String>,
    hash: Option<String>,
    length: Option<usize>,
}

#[derive(Default)]
struct Started {
    expected_iterations: HashSet<String>,
    received_msgs: HashSet<Msg>,
    file_name: Option<String>,
    hash: Option<String>,
    length: usize,
}

#[wasm_bindgen]
pub struct Finished {
    file_name: String,
    hash: String,
    data: Vec<u8>,
}

#[wasm_bindgen]
impl Finished {
    fn get_decompressed_data(&self) -> Vec<u8> {
        log("Decompressing...");
        decompress(self.data.clone())
    }

    fn check_integrity(&self) {
        let final_hash = hash(&self.data);

        let received_hash = &self.hash;
        if received_hash != &final_hash {
            panic!("[*] Expected: {}, got: {}", received_hash, final_hash)
        }
    }

    pub fn to_base64(&self) -> String {
        self.check_integrity();
        let decompressed_data = self.get_decompressed_data();
        BASE64_STANDARD.encode(decompressed_data)
    }

    pub fn get_name(&self) -> String {
        self.file_name.clone()
    }
}

struct Machine<S> {
    state: S,
}

impl Default for Machine<Initted> {
    fn default() -> Self {
        Machine {
            state: Initted::default(),
        }
    }
}

impl Default for Machine<Started> {
    // for take to work
    fn default() -> Self {
        Machine {
            state: Started::default(),
        }
    }
}

impl From<Machine<Initted>> for Machine<Started> {
    fn from(machine: Machine<Initted>) -> Machine<Started> {
        let length = machine.state.length.unwrap();
        log(&format!("[*] The message will come in {} parts", length));
        Machine {
            state: Started {
                expected_iterations: {
                    let mut iterations = HashSet::new();
                    iterations.insert("NAME".to_string());
                    iterations.insert("LEN".to_string());
                    iterations.insert("HASH".to_string());
                    for i in 1..=length {
                        iterations.insert(i.to_string());
                    }
                    iterations
                },
                received_msgs: machine.state.received_msgs,
                file_name: machine.state.file_name,
                hash: machine.state.hash,
                length,
            },
        }
    }
}

impl From<Machine<Started>> for Machine<Finished> {
    fn from(machine: Machine<Started>) -> Machine<Finished> {
        assert!(machine.check_finished(), "Incomplete data.");
        Machine {
            state: Finished {
                file_name: machine.state.file_name.unwrap(),
                hash: machine.state.hash.unwrap(),
                data: {
                    let mut ordered_iteration = machine
                        .state
                        .received_msgs
                        .into_iter()
                        .filter(|msg| matches!(msg, Msg::Piece(_, _)))
                        .collect::<Vec<Msg>>();
                    ordered_iteration.sort_by(|x, y| {
                        if let (Msg::Piece(xi, _), Msg::Piece(yi, _)) = (x, y) {
                            xi.parse::<usize>()
                                .unwrap()
                                .cmp(&yi.parse::<usize>().unwrap())
                        } else {
                            panic!("")
                        }
                    });
                    log(&format!("{:?}", ordered_iteration));
                    let data = ordered_iteration
                        .iter()
                        .map(|msg| {
                            if let Msg::Piece(_, data) = msg {
                                base10::decode(data)
                            } else {
                                panic!("")
                            }
                        })
                        .collect::<Vec<Vec<u8>>>()
                        .concat();
                    data
                },
            },
        }
    }
}

impl Machine<Started> {
    fn expecting(&self) -> HashSet<&String> {
        let received_iterations = self
            .state
            .received_msgs
            .iter()
            .map(|it| match it {
                Msg::Hash(_) => "HASH",
                Msg::Name(_) => "NAME",
                Msg::Length(_) => "LEN",
                Msg::Piece(i, _) => i,
            })
            .collect::<HashSet<&str>>();

        self.state
            .expected_iterations
            .iter()
            .filter(|s| !received_iterations.contains(s as &str)) // what the hell is this... why can't just s.
            .collect()
    }

    fn check_finished(&self) -> bool {
        self.expecting().is_empty()
    }
}

trait Receive {
    fn get_mut_name(&mut self) -> &mut Option<String>;
    fn set_name(&mut self, name: String) {
        log(&format!("[*] File name: {}", name));
        *(self.get_mut_name()) = Some(name)
    }
    fn get_mut_hash(&mut self) -> &mut Option<String>;
    fn set_hash(&mut self, hash: String) {
        log(&format!("[*] Hash: {}", hash));
        *(self.get_mut_hash()) = Some(hash)
    }
    fn get_mut_received_msgs(&mut self) -> &mut HashSet<Msg>;
    fn update(&mut self, msg: Msg) -> bool {
        let received_msgs = self.get_mut_received_msgs();

        if received_msgs.contains(&msg) {
            return false;
        }

        received_msgs.insert(msg.clone());

        match msg.clone() {
            Msg::Name(name) => {
                self.set_name(String::from_utf8(base10::decode(&name)).unwrap())
            }
            Msg::Hash(hash) => self.set_hash(hash.to_string()),
            _ => (),
        }

        log(&format!("processed {:?}", msg));
        true
    }
}

impl Receive for Machine<Initted> {
    fn get_mut_name(&mut self) -> &mut Option<String> {
        &mut self.state.file_name
    }
    fn get_mut_hash(&mut self) -> &mut Option<String> {
        &mut self.state.hash
    }
    fn get_mut_received_msgs(&mut self) -> &mut HashSet<Msg> {
        &mut self.state.received_msgs
    }
}
impl Receive for Machine<Started> {
    fn get_mut_name(&mut self) -> &mut Option<String> {
        &mut self.state.file_name
    }
    fn get_mut_hash(&mut self) -> &mut Option<String> {
        &mut self.state.hash
    }
    fn get_mut_received_msgs(&mut self) -> &mut HashSet<Msg> {
        &mut self.state.received_msgs
    }
}

enum MachineWrapper {
    Initted(Machine<Initted>),
    Started(Machine<Started>),
    Finished(Machine<Finished>),
}

#[wasm_bindgen]
pub struct Decoder {
    scanner: Quirc,
    decoder: MachineWrapper,
}

#[wasm_bindgen]
impl Decoder {
    #[allow(clippy::new_without_default)]
    pub fn new() -> Self {
        Decoder {
            scanner: Quirc::default(),
            decoder: MachineWrapper::Initted(Machine::default()),
        }
    }

    pub fn get_progress(&self) -> String {
        match &self.decoder {
            MachineWrapper::Initted(_) => "No LEN yet.".to_string(),
            MachineWrapper::Finished(_) => "Finished.".to_string(),
            MachineWrapper::Started(machine) => {
                let mut expecting = machine.expecting().into_iter().collect::<Vec<&String>>();
                expecting.sort_by(|x, y| {
                    if (x == &"NAME") | (x == &"LEN") | (x == &"HASH") {
                        Ordering::Less
                    } else if (y == &"NAME") | (y == &"LEN") | (y == &"HASH") {
                        Ordering::Greater
                    } else {
                        x.parse::<usize>()
                            .unwrap()
                            .cmp(&y.parse::<usize>().unwrap())
                    }
                });
                format!(
                    "{}/{}, expecting: {}.",
                    machine.state.length + 3 - expecting.len(),
                    machine.state.length + 3,
                    expecting
                        .into_iter()
                        .map(|s| s.to_string())
                        .collect::<Vec<String>>()
                        .join(", ")
                )
            }
        }
    }

    pub fn is_finished(&self) -> bool {
        matches!(&self.decoder, MachineWrapper::Finished(_))
    }

    fn try_evolve(&mut self) {
        match &mut self.decoder {
            MachineWrapper::Initted(decoder) => {
                if decoder.state.length.is_some() {
                    self.decoder = MachineWrapper::Started(take(decoder).into());
                    self.try_evolve(); // may evolve agian
                }
            }
            MachineWrapper::Started(decoder) => {
                if decoder.check_finished() {
                    self.decoder = MachineWrapper::Finished(take(decoder).into());
                }
            }
            _ => {}
        }
    }

    pub fn process_chunk(&mut self, chunk: String) -> bool {
        let msg = Msg::new(chunk);

        let updated = match &mut self.decoder {
            MachineWrapper::Initted(decoder) => {
                let updated = decoder.update(msg.clone());
                if updated {
                    if let Msg::Length(length) = msg {
                        decoder.state.length = Some(length.to_string().parse::<usize>().unwrap());
                    }
                }
                updated
            }
            MachineWrapper::Started(decoder) => decoder.update(msg.clone()),
            MachineWrapper::Finished(_) => false,
        };
        if updated {
            self.try_evolve()
        };
        updated
    }

    pub fn scan(&mut self, width: u32, height: u32, data: Vec<u8>) -> usize {
        let img: RgbaImage = ImageBuffer::from_raw(width, height, data).unwrap();
        let img_gray = DynamicImage::ImageRgba8(img).into_luma8();
        let codes: Vec<_> = self
            .scanner
            .identify(
                img_gray.width() as usize,
                img_gray.height() as usize,
                &img_gray,
            )
            .flatten()
            .collect();

        let mut counter = 0;
        for code in codes {
            {
                if let Ok(decoded) = code.decode() {
                    if let Ok(msg) = String::from_utf8(decoded.payload) {
                        if self.process_chunk(msg) {
                            counter += 1;
                        }
                    }
                }
            }
        }
        counter
    }

    pub fn get_finished(self) -> Finished {
        if let MachineWrapper::Finished(machine) = self.decoder {
            machine.state
        } else {
            panic!("Should be finished by now.")
        }
    }
}

#[test]
fn test_decoder() {
    let mut decoder = Decoder::new();

    decoder.process_chunk("NAME:2597379451834392631223363866405679089128269172".to_string());
    println!("{}", decoder.get_progress());
    decoder.process_chunk("LEN:2".to_string());
    println!("{}", decoder.get_progress());
    decoder.process_chunk("HASH:bf0c337e1d303f70a099465a726ef627ef91c4db".to_string());
    println!("{}", decoder.get_progress());
    decoder.process_chunk("1:2481676554535304448989663285024985913136368361479567388366436794976484106841552224124172984975668570544709175647171746092269804540550994669301355164043499806044980564484156500177098332432694272017792190515369801561555600289680265659814032923".to_string());
    println!("{}", decoder.get_progress());
    decoder.process_chunk("2:8633128646008937860073585629".to_string());
    println!("{}", decoder.get_progress());

    let res = decoder.get_finished();
    let decoded_data = BASE64_STANDARD.decode(res.to_base64()).unwrap();
    let decoded_data = String::from_utf8(decoded_data).unwrap();
    println!("{}", decoded_data);
    assert_eq!(decoded_data, "Transfer your file from an air gapped computer to iOS/iPhone/iPad using only qrcode, no wifi/usb/bluetooth needed. This is a proof-of-concept project, implemented in Rust WebAssembly.");
}

#[test]
fn test_when_len_came_at_last() {
    let mut decoder = Decoder::new();
    decoder.process_chunk("NAME:2597379451834392631223363866405679089128269172".to_string());
    decoder.process_chunk("HASH:bf0c337e1d303f70a099465a726ef627ef91c4db".to_string());
    decoder.process_chunk("1:2481676554535304448989663285024985913136368361479567388366436794976484106841552224124172984975668570544709175647171746092269804540550994669301355164043499806044980564484156500177098332432694272017792190515369801561555600289680265659814032923".to_string());
    decoder.process_chunk("2:8633128646008937860073585629".to_string());
    decoder.process_chunk("LEN:2".to_string());

    let res = decoder.get_finished();
    let decoded_data = BASE64_STANDARD.decode(res.to_base64()).unwrap();
    let decoded_data = String::from_utf8(decoded_data).unwrap();
    assert_eq!(decoded_data, "Transfer your file from an air gapped computer to iOS/iPhone/iPad using only qrcode, no wifi/usb/bluetooth needed. This is a proof-of-concept project, implemented in Rust WebAssembly.");
}
