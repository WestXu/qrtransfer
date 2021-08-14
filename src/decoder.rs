use super::log;
use sha1::{Digest, Sha1};
use std::cmp::Ordering;
use std::collections::{HashMap, HashSet};
use wasm_bindgen::prelude::*;

#[derive(PartialEq, Debug)]
enum Status {
    Initted,
    Started,
    Finished,
}

#[wasm_bindgen]
pub struct Decoder {
    status: Status,
    expected_iterations: Option<HashSet<String>>,
    received_iterations: HashMap<String, String>,
    file_name: Option<String>,
    hash: Option<String>,
    length: Option<usize>,
}

#[wasm_bindgen]
impl Decoder {
    pub fn new() -> Decoder {
        Decoder {
            status: Status::Initted,
            expected_iterations: None,
            received_iterations: HashMap::new(),
            file_name: None,
            hash: None,
            length: None,
        }
    }
    fn set_name(&mut self, name: String) {
        assert_ne!(self.status, Status::Finished);
        log(&format!("[*] File name: {}", name));
        self.file_name = Some(name);
    }

    fn set_length(&mut self, length: usize) {
        assert_eq!(self.status, Status::Initted);
        log(&format!("[*] The message will come in {} parts", length));
        self.length = Some(length);
        self.expected_iterations = {
            let mut iterations = HashSet::new();
            iterations.insert("NAME".to_string());
            iterations.insert("LEN".to_string());
            iterations.insert("HASH".to_string());
            for i in 1..=length {
                iterations.insert(i.to_string());
            }
            Some(iterations)
        };
        self.status = Status::Started;
    }

    fn set_hash(&mut self, hash: String) {
        assert_ne!(self.status, Status::Finished);
        log(&format!("[*] Hash {}", hash));
        self.hash = Some(hash);
    }

    fn expecting(&self) -> HashSet<&String> {
        assert_ne!(self.status, Status::Initted);

        self.expected_iterations
            .as_ref()
            .unwrap()
            .iter()
            .filter(|s| !self.received_iterations.contains_key(*s))
            .collect()
    }

    fn check_finished(&mut self) {
        log(&format!("status {:?}", self.status));
        if self.status == Status::Initted {
            return;
        }
        if self.expecting().len() == 0 {
            self.status = Status::Finished;
        }
    }

    fn data(&self) -> Vec<u8> {
        assert_eq!(self.status, Status::Finished);
        let mut ordered_iteration = self
            .received_iterations
            .iter()
            .filter(|(k, v)| !((k == &"NAME") | (k == &"LEN") | (k == &"HASH")))
            .collect::<Vec<_>>();

        ordered_iteration.sort_by(|x, y| {
            x.0.parse::<usize>()
                .unwrap()
                .cmp(&y.0.parse::<usize>().unwrap())
        });
        log(&format!("{:?}", ordered_iteration));
        ordered_iteration
            .iter()
            .map(|(k, v)| base64::decode(v).unwrap())
            .collect::<Vec<Vec<u8>>>()
            .concat()
    }

    fn check_integrity(&self) {
        assert_eq!(self.status, Status::Finished);
        let final_hash = format!("{:x}", {
            let mut hasher = Sha1::new();
            hasher.update(&self.data());
            hasher.finalize()
        });

        let received_hash = self.hash.as_ref().unwrap();
        if received_hash != &final_hash {
            panic!("[*] Expected: {}, got: {}", received_hash, final_hash)
        }
    }

    pub fn process_chunk(&mut self, chunk: String) -> bool {
        if self.status == Status::Finished {
            return false;
        }
        let split = chunk.split(":").collect::<Vec<&str>>();
        let i = split[0];
        let data = split[1];
        if self.received_iterations.contains_key(i) {
            return false;
        }

        self.received_iterations
            .insert(i.to_string(), data.to_string());

        match i {
            "NAME" => self.set_name(String::from_utf8(base64::decode(data).unwrap()).unwrap()),
            "LEN" => self.set_length(data.to_string().parse::<usize>().unwrap()),
            "HASH" => self.set_hash(data.to_string()),
            _ => (),
        }

        log(&format!("processed {}", chunk));
        self.check_finished();
        true
    }

    pub fn to_base64(&self) -> String {
        self.check_integrity();
        base64::encode(self.data())
    }

    pub fn get_name(&self) -> String {
        self.file_name.as_ref().unwrap().to_string()
    }

    pub fn is_finished(&self) -> bool {
        self.status == Status::Finished
    }

    pub fn get_progress(&self) -> String {
        match self.status {
            Status::Initted => "No LEN yet.".to_string(),
            Status::Finished => "Finished.".to_string(),
            Status::Started => {
                let mut expecting = self.expecting().into_iter().collect::<Vec<&String>>();
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
}
#[test]
fn test_decoder() {
    // FIXME: test won't work due to env of wasm
    let mut decoder = Decoder::new();

    decoder.process_chunk("NAME:YmluRmlsZS50eHQ=".to_string());
    decoder.process_chunk("LEN:2".to_string());
    decoder.process_chunk("HASH:23cbdb7dc9c34166abd505f2518152a6c05978d5".to_string());
    decoder.process_chunk("1:77u/VHJhbnNmZXIgeW91ciBmaWxlIGZyb20gYW4gYWlyIGdhcHBlZCBjb21wdXRlciB0byBpT1MvaVBob25lL2lQYWQgdXNpbmcgb25seSBxcmNvZGUsIG5vIHdpZmkvdXNiLw==".to_string());
    decoder.process_chunk("2:Ymx1ZXRvb3RoIG5lZWRlZC4=".to_string());

    let decoded_data = base64::decode(decoder.to_base64()).unwrap();
    let decoded_data = String::from_utf8(decoded_data).unwrap();
    println!("{}", decoded_data);
    assert_eq!(decoded_data, "Transfer your file from an air gapped computer to iOS/iPhone/iPad using only qrcode, no wifi/usb/bluetooth needed.");

    // one more
    decoder.process_chunk("2:Ymx1ZXRvb3RoIG5lZWRlZC4=".to_string());
}
