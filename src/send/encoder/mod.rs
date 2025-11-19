use crate::protocol::{Message, Metadata, Payload};
use crate::utils::{hash, log};
mod qr;
use crate::base10;
use crate::compress;
use indexmap::IndexMap;
use qr::qr;

pub struct Encoder {
    file_name: String,
    data: Vec<u8>,
}

impl Encoder {
    pub fn new(file_name: String, data: Vec<u8>) -> Encoder {
        Encoder {
            file_name,
            data: compress::compress(data),
        }
    }

    fn get_chunks(&self) -> Vec<&[u8]> {
        const SIZE: usize = 100;
        let mut chunks = Vec::new();
        let length = self.data.len();
        for i in (0..length).step_by(SIZE) {
            chunks.push(&self.data[i..(i + SIZE).min(length)])
        }
        chunks
    }

    fn get_metadata(&self, length: usize) -> Metadata {
        Metadata::new(
            base10::encode(self.file_name.as_bytes()),
            length,
            hash(&self.data),
        )
    }

    pub fn get_payload(&self) -> Payload {
        let chunks = self.get_chunks();
        let metadata = self.get_metadata(chunks.len());
        let pieces = chunks
            .iter()
            .enumerate()
            .map(|(counter, data)| (counter + 1, base10::encode(data)))
            .collect();

        Payload::new(metadata, pieces)
    }

    pub fn payloads(self) -> IndexMap<String, String> {
        let payload = self.get_payload();

        let mut payloads = IndexMap::new();

        let metadata_msg = Message::Metadata(payload.metadata);
        payloads.insert("METADATA".to_string(), metadata_msg.to_string());

        for (index, data) in payload.pieces {
            let piece_msg = Message::Piece { index, data };
            payloads.insert(format!("{}", index), piece_msg.to_string());
        }

        payloads
    }

    pub fn to_qr(self) -> IndexMap<String, String> {
        self.payloads()
            .iter()
            .map(|(name, payload)| {
                log(payload);
                (name.to_string(), qr(payload))
            })
            .collect()
    }
}

#[test]
fn test_encoder() {
    println!(
        "{:?}",
        Encoder::new(
            "test_file".to_string(),
            vec!(
                119, 115, 108, 32, 47, 104, 111, 109, 101, 47, 108, 120, 117, 119, 115, 108, 47,
                109, 105, 110, 105, 99, 111, 110, 100, 97, 51, 47, 98, 105, 110, 47, 105, 112, 121,
                116, 104, 111, 110, 32, 45, 45, 112, 100, 98, 32, 45, 99, 32, 34, 115, 104, 111,
                119, 95, 100, 102, 40, 112, 100, 46, 114, 101, 97, 100, 95, 112, 105, 99, 107, 108,
                101, 40, 80, 97, 116, 104, 40, 114, 39, 37, 49, 39, 41, 41, 41, 34, 40, 80, 97,
                116, 104, 40, 114, 39, 37, 49, 39, 41, 41, 41, 34, 40, 80, 97, 116, 104, 40, 114,
                39, 37, 49, 39, 41, 41, 41, 34, 40, 80, 97, 116, 104, 40, 114, 39, 37, 49, 39, 41,
                41, 41, 34
            )
        )
        .to_qr()
    )
}
