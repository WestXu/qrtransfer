use super::log;
use sha1::{Digest, Sha1};
use std::collections::{HashMap, HashSet};

#[derive(PartialEq)]
enum Status {
    Initted,
    Started,
}

pub struct Decoder {
    status: Status,
    expected_iterations: Option<HashSet<String>>,
    received_iterations: HashMap<String, String>,
    file_name: Option<String>,
    hash: Option<String>,
    length: Option<usize>,
}

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
        log(&format!("[*] File name: {}", name));
        self.file_name = Some(name);
    }

    fn set_length(&mut self, length: usize) {
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
        log(&format!("[*] Hash {}", hash));
        self.hash = Some(hash);
    }

    fn expecting(&self) -> HashSet<&String> {
        assert!(self.status == Status::Started);

        self.expected_iterations
            .as_ref()
            .unwrap()
            .iter()
            .filter(|s| !self.received_iterations.contains_key(*s))
            .collect()
    }

    fn is_finished(&self) -> bool {
        if self.status != Status::Started {
            return false;
        };
        self.expecting().len() == 0
    }

    fn data(&self) -> Vec<u8> {
        let mut ordered_iteration = self
            .received_iterations
            .iter()
            .filter(|(k, v)| !((k == &"NAME") | (k == &"LEN") | (k == &"HASH")))
            .collect::<Vec<_>>();

        ordered_iteration.sort_by(|x, y| x.0.cmp(y.0));
        log(&format!("{:?}", ordered_iteration));
        ordered_iteration
            .iter()
            .map(|(k, v)| base64::decode(v).unwrap())
            .collect::<Vec<Vec<u8>>>()
            .concat()
    }

    fn check_integrity(&self) {
        assert!(self.is_finished());
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
        let split = chunk.split(":").collect::<Vec<&str>>();
        let i = split[0];
        let data = split[1];
        if self.received_iterations.contains_key(i) {
            return false;
        }

        self.received_iterations
            .insert(i.to_string(), data.to_string());

        log(&format!("received {}", chunk));
        match i {
            "NAME" => self.set_name(String::from_utf8(base64::decode(data).unwrap()).unwrap()),
            "LEN" => self.set_length(data.to_string().parse::<usize>().unwrap()),
            "HASH" => self.set_hash(data.to_string()),
            _ => return true,
        }
        true
    }

    pub fn to_base64(&self) -> String {
        self.check_integrity();
        base64::encode(self.data())
    }
}
