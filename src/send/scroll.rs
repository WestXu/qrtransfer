use js_sys::Reflect;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;

fn scroll() {
    web_sys::window().unwrap().scroll_by_with_scroll_to_options(
        web_sys::ScrollToOptions::new()
            .behavior(web_sys::ScrollBehavior::Instant)
            .top(200.0),
    );
}

fn start_scroll() {
    let scroll_cb = Closure::wrap(Box::new(scroll) as Box<dyn Fn()>);
    let scroll_id = web_sys::window()
        .unwrap()
        .set_interval_with_callback_and_timeout_and_arguments_0(
            scroll_cb.as_ref().unchecked_ref(),
            1000,
        )
        .unwrap();
    scroll_cb.forget();
    Reflect::set(
        &web_sys::window().unwrap(),
        &JsValue::from("scrollIntervalId"),
        &JsValue::from(scroll_id),
    )
    .unwrap();
}

fn stop_scroll() {
    web_sys::window().unwrap().clear_interval_with_handle(
        Reflect::get(
            &web_sys::window().unwrap(),
            &JsValue::from("scrollIntervalId"),
        )
        .unwrap()
        .as_f64()
        .unwrap() as i32,
    )
}

pub fn toggle_scroll() {
    let v = Reflect::get(
        &web_sys::window()
            .unwrap()
            .document()
            .unwrap()
            .get_element_by_id("scroll-check")
            .unwrap(),
        &JsValue::from("checked"),
    )
    .unwrap()
    .as_bool()
    .unwrap();
    match v {
        true => start_scroll(),
        false => stop_scroll(),
    }
}
