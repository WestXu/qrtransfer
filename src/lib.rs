mod decoder;
mod encoder;
mod utils;
use utils::{log, set_panic_hook};

use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn my_set_panic_hook() -> Result<(), JsValue> {
    set_panic_hook();
    Ok(())
}

#[wasm_bindgen]
pub fn send(file_name: &str, int_array: &JsValue) -> Result<(), JsValue> {
    let html = {
        log(file_name);
        let int_array: Vec<u8> = int_array.into_serde().unwrap();

        encoder::Encoder::new(file_name.to_string(), int_array).to_html()
    };

    let window = web_sys::window().expect("no global `window` exists");
    let document = window.document().expect("should have a document on window");
    let body = document.body().expect("should have a body on document");

    body.set_inner_html(&html);
    Ok(())
}
