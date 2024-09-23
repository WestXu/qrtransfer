use dioxus::signals::GlobalSignal;
use dioxus::signals::Readable;
use dioxus::signals::Signal;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;

use crate::utils::log;

pub static SCROLLING_ID: GlobalSignal<Option<i32>> = Signal::global(|| None);

fn scroll() {
    let window = web_sys::window().unwrap();
    window.scroll_by_with_scroll_to_options(&{
        let options = web_sys::ScrollToOptions::new();
        options.set_behavior(web_sys::ScrollBehavior::Instant);
        options.set_top(200.0);
        options
    });

    // ((window.innerHeight + window.scrollY) >= document.body.scrollHeight)
    if (window.inner_height().unwrap().as_f64().unwrap() + window.scroll_y().unwrap())
        >= window.document().unwrap().body().unwrap().scroll_height() as f64
    {
        window.scroll_to_with_x_and_y(0.0, 0.0);
    }
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
    log(&format!("start scroll_id: {}", scroll_id));
    scroll_id
}

fn stop_scroll(id: i32) {
    web_sys::window().unwrap().clear_interval_with_handle(id);
    log(&format!("stop scroll_id: {}", id));
}

pub fn toggle_scroll() {
    let previous = *SCROLLING_ID.read();

    *SCROLLING_ID.write() = match previous {
        None => Some(start_scroll()),
        Some(id) => {
            stop_scroll(id);
            None
        }
    }
}
