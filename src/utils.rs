pub fn set_panic_hook() {
    console_error_panic_hook::set_once();
}

pub fn log(msg: &str) {
    web_sys::console::log_1(&wasm_bindgen::JsValue::from_str(msg));
}
