use super::compress::decompress;
use super::utils::hash;
use super::utils::log;
use image::{DynamicImage, ImageBuffer, RgbaImage};
use quircs::Quirc;
use std::cmp::Ordering;
use std::collections::HashSet;
use std::mem::take;
use wasm_bindgen::prelude::*;

enum State {
    Initted {
        received_msgs: HashSet<Msg>,
        file_name: Option<String>,
        hash: Option<String>,
    },
    Started {
        expected_iterations: HashSet<String>,
        received_msgs: HashSet<Msg>,
        file_name: Option<String>,
        hash: Option<String>,
        length: usize,
    },
    Finished {
        file_name: String,
        hash: String,
        data: Vec<u8>,
    },
}

#[derive(Debug, PartialEq, Eq, Hash)]
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

fn assemble_data(msgs: HashSet<Msg>) -> Vec<u8> {
    let mut ordered_iteration = msgs
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
    ordered_iteration
        .iter()
        .map(|msg| {
            if let Msg::Piece(_, data) = msg {
                base64::decode(data).unwrap()
            } else {
                panic!("")
            }
        })
        .collect::<Vec<Vec<u8>>>()
        .concat()
}

impl Default for State {
    fn default() -> Self {
        State::Initted {
            received_msgs: HashSet::new(),
            file_name: None,
            hash: None,
        }
    }
}

impl State {
    fn expecting(&self) -> Option<HashSet<&String>> {
        if let State::Started {
            expected_iterations,
            received_msgs,
            ..
        } = self
        {
            let received_iterations = received_msgs
                .iter()
                .map(|it| match it {
                    Msg::Hash(_) => "HASH",
                    Msg::Name(_) => "NAME",
                    Msg::Length(_) => "LEN",
                    Msg::Piece(i, _) => i,
                })
                .collect::<HashSet<&str>>();
            Some(
                expected_iterations
                    .iter()
                    .filter(|s| !received_iterations.contains(s as &str)) // what the hell is this... why can't just s.
                    .collect(),
            )
        } else {
            None
        }
    }
    fn check_finished(&self) -> bool {
        if let Some(its) = self.expecting() {
            its.is_empty()
        } else {
            false
        }
    }
    fn insert_msg(received_msgs: &mut HashSet<Msg>, msg: Msg) -> bool {
        if received_msgs.contains(&msg) {
            return false;
        }
        received_msgs.insert(msg);
        true
    }
    fn update(&mut self, msg: Msg) -> bool {
        use self::Msg::*;
        use self::State::*;

        match (self, msg) {
            (Finished { .. }, _) => false,
            (
                Initted {
                    received_msgs,
                    file_name,
                    ..
                }
                | Started {
                    received_msgs,
                    file_name,
                    ..
                },
                Name(n),
            ) => {
                let inserted = State::insert_msg(received_msgs, Name(n.clone()));
                if inserted {
                    log(&format!("[*] File name: {}", n));
                    *file_name = Some(String::from_utf8(base64::decode(n).unwrap()).unwrap());
                }
                inserted
            }
            (
                Initted {
                    received_msgs,
                    file_name: _,
                    hash,
                }
                | Started {
                    received_msgs,
                    file_name: _,
                    hash,
                    ..
                },
                Hash(h),
            ) => {
                let inserted = State::insert_msg(received_msgs, Hash(h.clone()));
                if inserted {
                    log(&format!("[*] Hash {}", h));
                    *hash = Some(h);
                }
                inserted
            }
            (Initted { received_msgs, .. } | Started { received_msgs, .. }, msg) => {
                State::insert_msg(received_msgs, msg)
            }
        }
    }
    pub fn next(self, msg: Msg) -> (State, bool) {
        use self::Msg::*;
        use self::State::*;

        match (self, msg) {
            (
                Initted {
                    received_msgs,
                    file_name,
                    hash,
                },
                Length(length),
            ) => {
                let mut state = Started {
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
                    received_msgs,
                    file_name,
                    hash,
                    length,
                };
                let updated = state.update(Length(length));
                (state, updated)
            }
            (
                Initted {
                    received_msgs,
                    file_name,
                    hash,
                },
                msg,
            ) => {
                let mut state = Initted {
                    received_msgs,
                    file_name,
                    hash,
                };
                let updated = state.update(msg);
                (state, updated)
            }
            (
                Started {
                    expected_iterations,
                    received_msgs,
                    file_name,
                    hash,
                    length,
                },
                msg,
            ) => {
                // the following is insanely wordy...
                let mut state = Started {
                    expected_iterations,
                    received_msgs,
                    file_name,
                    hash,
                    length,
                };
                if state.update(msg) & (state.check_finished()) {
                    if let Started {
                        expected_iterations: _,
                        received_msgs,
                        file_name,
                        hash,
                        length: _,
                    } = state
                    {
                        (
                            Finished {
                                file_name: file_name.unwrap(),
                                hash: hash.unwrap(),
                                data: assemble_data(received_msgs),
                            },
                            true,
                        )
                    } else {
                        panic!()
                    }
                } else if let Started {
                    expected_iterations,
                    received_msgs,
                    file_name,
                    hash,
                    length,
                } = state
                {
                    (
                        Started {
                            expected_iterations,
                            received_msgs,
                            file_name,
                            hash,
                            length,
                        },
                        false,
                    )
                } else {
                    panic!()
                }
            }
            (val @ Finished { .. }, _) => (val, false),
        }
    }
}

pub struct Output {
    pub file_name: String,
    pub hash: String,
    pub data: Vec<u8>,
}

impl From<State> for Output {
    fn from(state: State) -> Self {
        if let State::Finished {
            file_name,
            hash,
            data,
        } = state
        {
            Output {
                file_name,
                hash,
                data,
            }
        } else {
            panic!("Unfinished.")
        }
    }
}

impl Output {
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

#[wasm_bindgen]
#[derive(Default)]
pub struct Decoder {
    state: State,
}
impl Decoder {
    pub fn new() -> Self {
        Self::default()
    }
    pub fn process_chunk(&mut self, chunk: String) -> bool {
        let (state, updated) = take(&mut self.state).next(Msg::new(chunk));
        self.state = state;
        updated
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
    pub fn get_progress(&self) -> String {
        use self::State::*;

        match self.state {
            Initted { .. } => "No LEN yet.".to_string(),
            Finished { .. } => "Finished.".to_string(),
            Started { length, .. } => {
                let mut expecting = self
                    .state
                    .expecting()
                    .unwrap()
                    .into_iter()
                    .collect::<Vec<&String>>();
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
                    length + 3 - expecting.len(),
                    length + 3,
                    expecting
                        .into_iter()
                        .map(|s| s.to_string())
                        .collect::<Vec<String>>()
                        .join(", ")
                )
            }
        }
    }
    pub fn get_finished(self) -> Output {
        self.state.into()
    }
    pub fn is_finished(self) -> bool {
        matches!(self.state, State::Finished { .. })
    }
}

#[test]
fn test_decoder() {
    let mut decoder = Decoder::new();

    decoder.process_chunk("NAME:dGVzdF9xcnRyYW5zZmVyLnR4dA==".to_string());
    println!("{}", decoder.get_progress());
    decoder.process_chunk("LEN:2".to_string());
    println!("{}", decoder.get_progress());
    decoder.process_chunk("HASH:bf0c337e1d303f70a099465a726ef627ef91c4db".to_string());
    println!("{}", decoder.get_progress());
    decoder.process_chunk("1:G7YA4MVyW6oXCn6KbhrMx0C9wiM8U0+WhRrPCKomVFU2OVunN7y5HhGHtMnB5hPiEp8t9bCBGnjYey3YRlLaTxOWCBIsfQ5bSXyDSXg2x69btma2UFu4x4svyoIGUQyUNPFGXw==".to_string());
    println!("{}", decoder.get_progress());
    decoder.process_chunk("2:3fsUxrFm4KoZKOUb".to_string());
    println!("{}", decoder.get_progress());

    let res = decoder.get_finished();
    let decoded_data = base64::decode(res.to_base64()).unwrap();
    let decoded_data = String::from_utf8(decoded_data).unwrap();
    println!("{}", decoded_data);
    assert_eq!(decoded_data, "Transfer your file from an air gapped computer to iOS/iPhone/iPad using only qrcode, no wifi/usb/bluetooth needed. This is a proof-of-concept project, implemented in Rust WebAssembly.");
}
