use js_sys::Reflect;
use qrtransfer::utils::{log, set_panic_hook};
use qrtransfer::{compress, decoder, encoder};
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub struct QrTransfer {}

#[wasm_bindgen]
impl QrTransfer {
    #[wasm_bindgen]
    pub fn send(&self, file_name: &str, int_array: &JsValue) -> Result<(), JsValue> {
        let html = {
            log(file_name);
            let int_array: Vec<u8> = int_array.into_serde().unwrap();
            log("Compressing...");
            let int_array = compress::compress(int_array);
            encoder::Encoder::new(file_name.to_string(), int_array).to_html()
        };
        let window = web_sys::window().expect("no global `window` exists");
        let document = window.document().expect("should have a document on window");
        let middle_div = document
            .get_element_by_id("middle-div")
            .expect("should have a middle-div element");
        middle_div.set_inner_html(&html);
        Ok(())
    }
    #[wasm_bindgen]
    pub fn new_decoder(&self) -> Result<decoder::Decoder, JsValue> {
        Ok(decoder::Decoder::new())
    }
}

fn main() {
    log("Initializing wasm...");
    set_panic_hook();
    let window = web_sys::window().expect("no global `window` exists");
    Reflect::set(
        &window,
        &JsValue::from("qrtransfer"),
        &JsValue::from(QrTransfer {}),
    )
    .unwrap();

    let document = window.document().expect("should have a document on window");
    document
        .get_element_by_id("spinner")
        .unwrap()
        .set_attribute("style", "display: none;")
        .unwrap();
    document
        .get_element_by_id("main-page")
        .unwrap()
        .set_attribute("style", "display: block;")
        .unwrap();

    log("Wasm initialized.");
}
