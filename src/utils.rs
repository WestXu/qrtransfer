use sha1::{Digest, Sha1};

pub fn set_panic_hook() {
    console_error_panic_hook::set_once();
}

pub fn log(msg: &str) {
    web_sys::console::log_1(&wasm_bindgen::JsValue::from_str(msg));
}

pub fn hash(data: &[u8]) -> String {
    format!("{:x}", {
        let mut hasher = Sha1::new();
        hasher.update(data);
        hasher.finalize()
    })
}
