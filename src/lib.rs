mod decoder;
mod encoder;
mod scan;
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
        log(&format!("{:?}", int_array));

        encoder::Encoder::new(file_name.to_string(), int_array).to_html()
    };

    let window = web_sys::window().expect("no global `window` exists");
    let document = window.document().expect("should have a document on window");

    let qrcode_div = document
        .get_element_by_id("qrcode")
        .expect("should have a qrcode element");

    qrcode_div.set_inner_html(&html);

    Ok(())
}

#[wasm_bindgen]
pub fn scan_img(width: u32, height: u32, data: &JsValue) -> Result<(), JsValue> {
    let int_array: Vec<u8> = data.into_serde().unwrap();
    // log(&format!("data:{:?}", int_array));
    let decoded_msg = scan::scan(width, height, int_array);
    decoded_msg
        .iter()
        .map(|msg| log(&format!("{}", msg)))
        .collect::<()>();

    Ok(())
}
