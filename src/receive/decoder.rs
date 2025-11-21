#![allow(non_snake_case)]

use crate::compress::decompress;
use crate::protocol::Message;
use crate::protocol::Metadata;
use crate::utils::hash;
use crate::utils::log;
use base64::{prelude::BASE64_STANDARD, Engine as _};
use image::{DynamicImage, ImageBuffer, RgbaImage};
use quircs::Quirc;
use std::cmp::Ordering;
use std::collections::HashSet;
use std::mem::take;
use std::str::FromStr;
use wasm_bindgen::prelude::*;

#[derive(Default)]
struct Initted {
    received_msgs: HashSet<Message>,
    metadata: Option<Metadata>,
}

#[derive(Default)]
struct Started {
    expected_iterations: HashSet<String>,
    received_msgs: HashSet<Message>,
    metadata: Metadata,
}

#[wasm_bindgen]
pub struct Finished {
    metadata: Metadata,
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

        let received_hash = &self.metadata.hash;
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
        let decoded_bytes = BASE64_STANDARD.decode(&self.metadata.name).unwrap();
        String::from_utf8(decoded_bytes).unwrap_or_else(|_| self.metadata.name.clone())
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
        let length = machine.state.metadata.as_ref().unwrap().length;
        log(&format!("[*] The message will come in {} parts", length));
        Machine {
            state: Started {
                expected_iterations: {
                    let mut iterations = HashSet::new();
                    iterations.insert("METADATA".to_string());
                    for i in 1..=length {
                        iterations.insert(i.to_string());
                    }
                    iterations
                },
                received_msgs: machine.state.received_msgs,
                metadata: machine.state.metadata.unwrap(),
            },
        }
    }
}

impl From<Machine<Started>> for Machine<Finished> {
    fn from(machine: Machine<Started>) -> Machine<Finished> {
        assert!(machine.check_finished(), "Incomplete data.");
        Machine {
            state: Finished {
                metadata: machine.state.metadata,
                data: {
                    let mut ordered_iteration = machine
                        .state
                        .received_msgs
                        .into_iter()
                        .filter(|msg| matches!(msg, Message::Piece { .. }))
                        .collect::<Vec<Message>>();
                    ordered_iteration.sort_by(|x, y| {
                        if let (
                            Message::Piece { index: xi, .. },
                            Message::Piece { index: yi, .. },
                        ) = (x, y)
                        {
                            xi.cmp(yi)
                        } else {
                            panic!("")
                        }
                    });
                    log(&format!("{:?}", ordered_iteration));
                    let data = ordered_iteration
                        .iter()
                        .map(|msg| {
                            if let Message::Piece { data, .. } = msg {
                                BASE64_STANDARD.decode(data).unwrap()
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
                Message::Metadata(_) => "METADATA".to_string(),
                Message::Piece { index, .. } => index.to_string(),
            })
            .collect::<HashSet<String>>();

        self.state
            .expected_iterations
            .iter()
            .filter(|s| !received_iterations.contains(*s))
            .collect()
    }

    fn check_finished(&self) -> bool {
        self.expecting().is_empty()
    }
}

trait Receive {
    fn get_mut_metadata(&mut self) -> &mut Option<Metadata>;
    fn set_metadata(&mut self, metadata: Metadata) {
        log(&format!("[*] Metadata: {}", metadata));
        *(self.get_mut_metadata()) = Some(metadata)
    }
    fn get_mut_received_msgs(&mut self) -> &mut HashSet<Message>;
    fn update(&mut self, msg: Message) -> bool {
        let received_msgs = self.get_mut_received_msgs();

        if received_msgs.contains(&msg) {
            return false;
        }

        received_msgs.insert(msg.clone());

        if let Message::Metadata(metadata) = msg.clone() {
            self.set_metadata(metadata);
        }

        true
    }
}

impl Receive for Machine<Initted> {
    fn get_mut_metadata(&mut self) -> &mut Option<Metadata> {
        &mut self.state.metadata
    }
    fn get_mut_received_msgs(&mut self) -> &mut HashSet<Message> {
        &mut self.state.received_msgs
    }
}
impl Receive for Machine<Started> {
    fn get_mut_metadata(&mut self) -> &mut Option<Metadata> {
        panic!("Started state already has metadata.")
    }
    fn get_mut_received_msgs(&mut self) -> &mut HashSet<Message> {
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
            MachineWrapper::Initted(_) => "No METADATA yet.".to_string(),
            MachineWrapper::Finished(_) => "Finished.".to_string(),
            MachineWrapper::Started(machine) => {
                let mut expecting = machine.expecting().into_iter().collect::<Vec<&String>>();
                expecting.sort_by(|x, y| {
                    if x == &"METADATA" {
                        Ordering::Less
                    } else if y == &"METADATA" {
                        Ordering::Greater
                    } else {
                        x.parse::<usize>()
                            .unwrap()
                            .cmp(&y.parse::<usize>().unwrap())
                    }
                });
                format!(
                    "{}/{}, expecting: {}.",
                    machine.state.metadata.length + 1 - expecting.len(),
                    machine.state.metadata.length + 1,
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
                if decoder.state.metadata.is_some() {
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
        let msg = match Message::from_str(&chunk) {
            Ok(msg) => msg,
            Err(e) => {
                log(&format!("Failed to parse message: {}", e));
                return false;
            }
        };

        let updated = match &mut self.decoder {
            MachineWrapper::Initted(decoder) => {
                let updated = decoder.update(msg.clone());
                if updated {
                    if let Message::Metadata(ref metadata) = msg {
                        decoder.state.metadata = Some(metadata.clone());
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

    decoder.process_chunk(
        "METADATA:dGVzdF9xcnRyYW5zZmVyLnR4dA==,2,bf0c337e1d303f70a099465a726ef627ef91c4db"
            .to_string(),
    );
    println!("{}", decoder.get_progress());
    decoder.process_chunk("1:G7YA4MVyW6oXCn6KbhrMx0C9wiM8U0+WhRrPCKomVFU2OVunN7y5HhGHtMnB5hPiEp8t9bCBGnjYey3YRlLaTxOWCBIsfQ5bSXyDSXg2x69btma2UFu4x4svyoIGUQyUNPFGXw==".to_string());
    println!("{}", decoder.get_progress());
    decoder.process_chunk("2:3fsUxrFm4KoZKOUb".to_string());
    println!("{}", decoder.get_progress());

    let res = decoder.get_finished();
    let decoded_data = BASE64_STANDARD.decode(res.to_base64()).unwrap();
    let decoded_data = String::from_utf8(decoded_data).unwrap();
    println!("{}", decoded_data);
    assert_eq!(decoded_data, "Transfer your file from an air gapped computer to iOS/iPhone/iPad using only qrcode, no wifi/usb/bluetooth needed. This is a proof-of-concept project, implemented in Rust WebAssembly.");
}

#[test]
fn test_when_metadata_came_at_last() {
    let mut decoder = Decoder::new();
    decoder.process_chunk("1:G7YA4MVyW6oXCn6KbhrMx0C9wiM8U0+WhRrPCKomVFU2OVunN7y5HhGHtMnB5hPiEp8t9bCBGnjYey3YRlLaTxOWCBIsfQ5bSXyDSXg2x69btma2UFu4x4svyoIGUQyUNPFGXw==".to_string());
    decoder.process_chunk("2:3fsUxrFm4KoZKOUb".to_string());
    decoder.process_chunk(
        "METADATA:dGVzdF9xcnRyYW5zZmVyLnR4dA==,2,bf0c337e1d303f70a099465a726ef627ef91c4db"
            .to_string(),
    );

    let res = decoder.get_finished();
    let decoded_data = BASE64_STANDARD.decode(res.to_base64()).unwrap();
    let decoded_data = String::from_utf8(decoded_data).unwrap();
    assert_eq!(decoded_data, "Transfer your file from an air gapped computer to iOS/iPhone/iPad using only qrcode, no wifi/usb/bluetooth needed. This is a proof-of-concept project, implemented in Rust WebAssembly.");
}
