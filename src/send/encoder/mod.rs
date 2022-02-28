use crate::utils::{hash, log};
mod qr;
use crate::compress;
use base64::encode;
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

    fn get_headers(&self, length: usize) -> IndexMap<String, String> {
        let mut headers = IndexMap::new();
        headers.insert(
            "NAME".to_string(),
            format!("NAME:{}", encode(&self.file_name)),
        );
        headers.insert("LEN".to_string(), format!("LEN:{}", length));
        headers.insert("HASH".to_string(), format!("HASH:{}", hash(&self.data)));
        headers
    }

    pub fn payloads(self) -> IndexMap<String, String> {
        let chunks = self.get_chunks();

        let mut data_payloads = IndexMap::new();
        for (counter, data) in chunks.iter().enumerate() {
            data_payloads.insert(
                format!("{}", counter + 1),
                format!("{}:{}", counter + 1, encode(data)),
            );
        }

        let mut payloads = self.get_headers(chunks.len());
        payloads.extend(data_payloads);
        payloads
    }

    fn one_to_html(name: String, payload: String) -> String {
        "<table style=\"float:left;\">".to_string()
            + &format!("<tr><td class=\"qr\">{}</td></tr>", qr(&payload))
            + &format!("<tr><td align=\"center\">{}</td></tr></table>", name)
    }

    pub fn to_html(self) -> String {
        let mut html = String::new();
        for (name, payload) in self.payloads() {
            log(&payload);
            html += &Encoder::one_to_html(name, payload);
        }
        html
    }
}

#[test]
fn test_encoder() {
    println!(
        "{}",
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
        .to_html()
    )
}
