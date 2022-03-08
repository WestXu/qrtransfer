use dioxus::prelude::*;
use js_sys::Reflect;
use wasm_bindgen::prelude::*;

use qrtransfer::utils::{log, set_panic_hook};
use qrtransfer::{decoder, send};

#[wasm_bindgen]
pub struct QrTransfer {}

#[wasm_bindgen]
impl QrTransfer {
    #[wasm_bindgen]
    pub fn new_decoder(&self) -> Result<decoder::Decoder, JsValue> {
        Ok(decoder::Decoder::new())
    }
}

fn app(cx: Scope) -> Element {
    let (scroll_id, set_scroll_id) = use_state(&cx, || 0);
    let (scrolling, set_scrolling) = use_state(&cx, || false);

    cx.render(rsx! {
        div {
            id: "outer-div",
            div {
                class: "form-check form-switch float",
                id: "scroll-check-div",
                style: "display: none;",
                input {
                    class: "form-check-input",
                    id: "scroll-check",
                    onclick: move |_| send::toggle_scroll(
                        scrolling.to_owned(), 
                        set_scrolling, 
                        scroll_id.to_owned(), 
                        set_scroll_id
                    ),
                    r#type: "checkbox",
                }
                label {
                    class: "form-check-label",
                    r#for: "scroll-check",
                    "Scroll"
                }
            }
            div {
                id: "middle-div",
                div {
                    id: "inner-div",
                    div {
                        id: "main-page",
                        h1 {
                            "qrtransfer"
                        }
                        ul {
                            class: "nav nav-tabs",
                            id: "myTab",
                            role: "tablist",
                            li {
                                class: "nav-item",
                                role: "presentation",
                                button {
                                    class: "nav-link active",
                                    id: "send-tab",
                                    "aria-controls": "send",
                                    "aria-selected": "true",
                                    "data-bs-target": "#send",
                                    "data-bs-toggle": "tab",
                                    role: "tab",
                                    r#type: "button",
                                    "Send"
                                }
                            }
                            li {
                                class: "nav-item",
                                role: "presentation",
                                button {
                                    class: "nav-link",
                                    id: "receive-tab",
                                    "aria-controls": "receive",
                                    "aria-selected": "false",
                                    "data-bs-target": "#receive",
                                    "data-bs-toggle": "tab",
                                    role: "tab",
                                    r#type: "button",
                                    "Receive"
                                }
                            }
                        }
                        div {
                            class: "tab-content",
                            div {
                                class: "tab-pane active",
                                id: "send",
                                "aria-labelledby": "send-tab",
                                role: "tabpanel",
                                input {
                                    class: "form-control form-control-lg",
                                    id: "file-selector",
                                    onchange: move |_| send::read_file_content(),
                                    r#type: "file",
                                }
                                div {
                                    id: "progress",
                                }
                                div {
                                    id: "qrcode",
                                }
                            }
                            div {
                                class: "tab-pane",
                                id: "receive",
                                "aria-labelledby": "receiv-tab",
                                role: "tabpanel",
                                video {
                                    id: "scan-video",
                                    playsinline: "true",
                                    autoplay: "true",
                                }
                                canvas {
                                    id: "canvas",
                                    style: "display: none;",
                                }
                                div {
                                    id: "cam-qr-result",
                                    style: "white-space: pre;word-wrap:break-word;",
                                }
                                br {
                                }
                                button {
                                    class: "btn btn-outline-primary",
                                    id: "start-button",
                                    "onclick": "start_receiving()",
                                    "Start"
                                }
                                button {
                                    class: "btn btn-outline-danger",
                                    id: "stop-button",
                                    "onclick": "stop_receiving()",
                                    "Stop"
                                }
                            }
                        }
                    }
                }
            }
        }
    })
}

fn main() {
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

    dioxus::web::launch(app);
    log("Wasm initialized.");
}
