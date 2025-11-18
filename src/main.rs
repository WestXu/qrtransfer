#![allow(non_snake_case)]

use dioxus::prelude::*;
use qrtransfer::receive::{start_receiving, stop_receiving, switch_camera};

use qrtransfer::send::{self, QrResPage};
use qrtransfer::utils::{log, set_panic_hook};
use qrtransfer::QR_RES;

fn app() -> Element {
    let payloads = QR_RES.read().clone();
    if payloads.is_empty() {
        rsx! {
            div { id: "outer-div",
                div { id: "middle-div",
                    div { id: "inner-div",
                        div { id: "main-page",
                            h1 { "qrtransfer" }
                            ul {
                                class: "nav nav-tabs",
                                id: "myTab",
                                role: "tablist",
                                li { class: "nav-item", role: "presentation",
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
                                li { class: "nav-item", role: "presentation",
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
                            div { class: "tab-content",
                                div {
                                    class: "tab-pane active",
                                    id: "send",
                                    "aria-labelledby": "send-tab",
                                    role: "tabpanel",
                                    input {
                                        class: "form-control form-control-lg",
                                        id: "file-selector",
                                        onchange: move |_| {
                                            spawn(async move {
                                                send::read_file_content().await;
                                            });
                                        },
                                        r#type: "file",
                                    }
                                    div { id: "progress" }
                                    div { id: "qrcode" }
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
                                        style: "cursor: pointer;",
                                        onclick: move |_| switch_camera(),
                                    }
                                    div { style: "font-size: 0.85em; color: #666; margin-top: 5px;",
                                        "Click video to switch camera"
                                    }
                                    canvas { id: "canvas", style: "display: none;" }
                                    div {
                                        id: "cam-qr-result",
                                        style: "white-space: pre;word-wrap:break-word;",
                                    }
                                    br {
                                    }
                                    button {
                                        class: "btn btn-outline-primary",
                                        id: "start-button",
                                        onclick: move |_| start_receiving(),
                                        "Start"
                                    }
                                    button {
                                        class: "btn btn-outline-danger",
                                        id: "stop-button",
                                        onclick: move |_| stop_receiving(),
                                        "Stop"
                                    }
                                }
                            }
                            a { href: "https://github.com/WestXu/qrtransfer",
                                img {
                                    src: "https://github.githubassets.com/images/modules/logos_page/GitHub-Mark.png",
                                    alt: "GitHub",
                                    height: "30",
                                    style: "opacity:0.6;margin-top:10;float:right",
                                }
                            }
                        }
                    }
                }
            }
        }
    } else {
        rsx! {
            QrResPage { payloads }
        }
    }
}

fn main() {
    set_panic_hook();

    web_sys::window()
        .unwrap()
        .document()
        .unwrap()
        .get_element_by_id("spinner")
        .unwrap()
        .set_attribute("style", "display: none;")
        .unwrap();

    launch(app);
    log("Wasm initialized.");
}
