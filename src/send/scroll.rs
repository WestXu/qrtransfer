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

fn start_scroll() -> i32 {
    let scroll_cb = Closure::wrap(Box::new(scroll) as Box<dyn Fn()>);
    let scroll_id = web_sys::window()
        .unwrap()
        .set_interval_with_callback_and_timeout_and_arguments_0(
            scroll_cb.as_ref().unchecked_ref(),
            1000,
        )
        .unwrap();
    scroll_cb.forget();
    scroll_id
}

fn stop_scroll(scroll_id: i32) {
    web_sys::window()
        .unwrap()
        .clear_interval_with_handle(scroll_id)
}

pub fn toggle_scroll(scroll_id: i32, set_scroll_id: &dioxus::prelude::UseState<i32>) {
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
        true => set_scroll_id(start_scroll()),
        false => stop_scroll(scroll_id),
    }
}
