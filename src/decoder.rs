use super::compress::decompress;
use super::utils::hash;
use super::utils::log;
use image::{DynamicImage, ImageBuffer, RgbaImage};
use quircs::Quirc;
use std::cmp::Ordering;
use std::collections::{HashMap, HashSet};
use std::mem::take;
use wasm_bindgen::prelude::*;

struct Decoder<S> {
    state: S,
}

struct Initted {
    received_iterations: HashMap<String, String>,
    file_name: Option<String>,
    hash: Option<String>,
}

struct Started {
    expected_iterations: HashSet<String>,
    received_iterations: HashMap<String, String>,
    file_name: Option<String>,
    hash: Option<String>,
    length: usize,
}

pub struct Finished {
    pub file_name: String,
    hash: String,
    data: Vec<u8>,
}

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
        base64::encode(decompressed_data)
    }
}

impl Default for Decoder<Initted> {
    fn default() -> Self {
        Decoder {
            state: Initted {
                received_iterations: HashMap::new(),
                file_name: None,
                hash: None,
            },
        }
    }
}

impl Decoder<Initted> {
    pub fn start(self, length: usize) -> Decoder<Started> {
        log(&format!("[*] The message will come in {} parts", length));
        Decoder {
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
                received_iterations: self.state.received_iterations,
                file_name: self.state.file_name,
                hash: self.state.hash,
                length,
            },
        }
    }
}

impl Default for Decoder<Started> {
    // for take to work
    fn default() -> Self {
        Decoder {
            state: Started {
                expected_iterations: HashSet::new(),
                received_iterations: HashMap::new(),
                file_name: None,
                hash: None,
                length: 0,
            },
        }
    }
}

impl Decoder<Started> {
    fn expecting(&self) -> HashSet<&String> {
        self.state
            .expected_iterations
            .iter()
            .filter(|s| !self.state.received_iterations.contains_key(*s))
            .collect()
    }

    fn finish(self) -> Decoder<Finished> {
        Decoder {
            state: Finished {
                file_name: self.state.file_name.unwrap(),
                hash: self.state.hash.unwrap(),
                data: {
                    let mut ordered_iteration = self
                        .state
                        .received_iterations
                        .iter()
                        .filter(|(k, _v)| !((k == &"NAME") | (k == &"LEN") | (k == &"HASH")))
                        .collect::<Vec<_>>();
                    ordered_iteration.sort_by(|x, y| {
                        x.0.parse::<usize>()
                            .unwrap()
                            .cmp(&y.0.parse::<usize>().unwrap())
                    });
                    log(&format!("{:?}", ordered_iteration));
                    let data = ordered_iteration
                        .iter()
                        .map(|(_k, v)| base64::decode(v).unwrap())
                        .collect::<Vec<Vec<u8>>>()
                        .concat();
                    data
                },
            },
        }
    }

    fn check_finished(&mut self) -> bool {
        self.expecting().is_empty()
    }
}

enum DecoderWrapper {
    Initted(Decoder<Initted>),
    Started(Decoder<Started>),
    Finished(Decoder<Finished>),
}

#[wasm_bindgen]
pub struct DecoderFactory {
    decoder: DecoderWrapper,
}

impl DecoderFactory {
    pub fn new() -> Self {
        DecoderFactory {
            decoder: DecoderWrapper::Initted(Decoder::default()),
        }
    }

    fn set_name(&mut self, name: String) {
        log(&format!("[*] File name: {}", name));
        match &mut self.decoder {
            DecoderWrapper::Initted(val) => {
                val.state.file_name = Some(name);
            }
            DecoderWrapper::Started(val) => {
                val.state.file_name = Some(name);
            }
            DecoderWrapper::Finished(_) => {
                panic!("Decoder is already finished.")
            }
        };
    }

    fn set_hash(&mut self, hash: String) {
        log(&format!("[*] Hash {}", hash));
        match &mut self.decoder {
            DecoderWrapper::Initted(val) => {
                val.state.hash = Some(hash);
            }
            DecoderWrapper::Started(val) => {
                val.state.hash = Some(hash);
            }
            DecoderWrapper::Finished(_) => {
                panic!("Decoder is already finished.")
            }
        };
    }

    pub fn get_progress(&self) -> String {
        match &self.decoder {
            DecoderWrapper::Initted(_) => "No LEN yet.".to_string(),
            DecoderWrapper::Finished(_) => "Finished.".to_string(),
            DecoderWrapper::Started(val) => {
                let mut expecting = val.expecting().into_iter().collect::<Vec<&String>>();
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
                format!("Expecting: {:?}", expecting)
            }
        }
    }

    pub fn process_chunk(&mut self, chunk: String) -> bool {
        let split = chunk.split(':').collect::<Vec<&str>>();
        let i = split[0];
        let data = split[1];

        match &mut self.decoder {
            DecoderWrapper::Finished(_) => {
                return false;
            }
            DecoderWrapper::Initted(val) => {
                if val.state.received_iterations.contains_key(i) {
                    return false;
                }
                val.state
                    .received_iterations
                    .insert(i.to_string(), data.to_string());
            }
            DecoderWrapper::Started(val) => {
                if val.state.received_iterations.contains_key(i) {
                    return false;
                }
                val.state
                    .received_iterations
                    .insert(i.to_string(), data.to_string());
            }
        }
        match i {
            "NAME" => self.set_name(String::from_utf8(base64::decode(data).unwrap()).unwrap()),
            "HASH" => self.set_hash(data.to_string()),
            _ => (),
        }

        if let DecoderWrapper::Initted(val) = &mut self.decoder {
            if i == "LEN" {
                self.decoder = DecoderWrapper::Started(
                    take(val).start(data.to_string().parse::<usize>().unwrap()),
                )
            }
        }

        if let DecoderWrapper::Started(val) = &mut self.decoder {
            if val.check_finished() {
                self.decoder = DecoderWrapper::Finished(take(val).finish())
            }
        }

        log(&format!("processed {}", chunk));
        true
    }

    pub fn scan(&mut self, width: u32, height: u32, data: Vec<u8>) -> usize {
        let img: RgbaImage = ImageBuffer::from_raw(width, height, data).unwrap();
        let img_gray = DynamicImage::ImageRgba8(img).into_luma8();
        let mut decoder = Quirc::default();
        let codes = decoder.identify(
            img_gray.width() as usize,
            img_gray.height() as usize,
            &img_gray,
        );

        let mut counter = 0;
        for code in codes {
            {
                if let Ok(code) = code {
                    if let Ok(decoded) = code.decode() {
                        if let Ok(msg) = String::from_utf8(decoded.payload) {
                            if self.process_chunk(msg) {
                                counter += 1;
                            }
                        }
                    }
                }
            }
        }
        counter
    }

    pub fn get_finished(self) -> Finished {
        if let DecoderWrapper::Finished(val) = self.decoder {
            val.state
        } else {
            panic!("Should be finished by now.")
        }
    }
}

#[test]
fn test_decoder() {
    let mut decoder = DecoderFactory::new();

    decoder.process_chunk("NAME:dGVzdF9xcnRyYW5zZmVyLnR4dA==".to_string());
    decoder.process_chunk("LEN:2".to_string());
    decoder.process_chunk("HASH:bf0c337e1d303f70a099465a726ef627ef91c4db".to_string());
    decoder.process_chunk("1:G7YA4MVyW6oXCn6KbhrMx0C9wiM8U0+WhRrPCKomVFU2OVunN7y5HhGHtMnB5hPiEp8t9bCBGnjYey3YRlLaTxOWCBIsfQ5bSXyDSXg2x69btma2UFu4x4svyoIGUQyUNPFGXw==".to_string());
    decoder.process_chunk("2:3fsUxrFm4KoZKOUb".to_string());

    let res = decoder.get_finished();
    let decoded_data = base64::decode(res.to_base64()).unwrap();
    let decoded_data = String::from_utf8(decoded_data).unwrap();
    println!("{}", decoded_data);
    assert_eq!(decoded_data, "Transfer your file from an air gapped computer to iOS/iPhone/iPad using only qrcode, no wifi/usb/bluetooth needed. This is a proof-of-concept project, implemented in Rust WebAssembly.");
}
