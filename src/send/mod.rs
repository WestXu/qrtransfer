#![allow(non_snake_case)]

pub mod encoder;

use crate::utils::log;
use crate::{QR_INDEX, QR_RES};

use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;

use dioxus::prelude::*;
use indexmap::IndexMap;

#[derive(PartialEq, Clone, Props, Default)]
pub struct QrRes {
    pub payloads: IndexMap<String, String>,
}

pub fn QrResPage(props: QrRes) -> Element {
    let qr_index = QR_INDEX.signal();
    let mut is_playing = use_signal(|| false);
    let mut playback_speed = use_signal(|| 1.0);
    let should_loop = use_signal(|| true);

    let total = props.payloads.len();

    use_hook(|| {
        if total > 0 {
            spawn(async move {
                loop {
                    let speed = *playback_speed.read();
                    let interval_ms = (100.0 / speed) as u32;
                    gloo_timers::future::TimeoutFuture::new(interval_ms).await;

                    if *is_playing.read() {
                        let current = *QR_INDEX.read();
                        let next = current + 1;
                        if next >= total {
                            if *should_loop.read() {
                                *QR_INDEX.write() = 0;
                            } else {
                                *is_playing.write() = false;
                            }
                        } else {
                            *QR_INDEX.write() = next;
                        }
                    }
                }
            });
        }
    });

    if props.payloads.is_empty() {
        return rsx! {
            div {}
        };
    }

    let current_index = *qr_index.read() % total;
    let (_name, svg) = props.payloads.get_index(current_index).unwrap();

    let play_pause_text = if *is_playing.read() { "⏸" } else { "▶" };
    let speed_text = format!("{}x", *playback_speed.read());

    let title = if current_index == 0 {
        "* Scan this METADATA before playing".to_string()
    } else {
        format!("{} / {}", current_index, total - 1)
    };

    rsx! {
        div { style: "display: flex; flex-direction: column; align-items: center; justify-content: center; height: 100vh;",
            div { style: "margin-bottom: 20px; display: flex; justify-content: left;",
                button {
                    style: "font-size: 18px; padding: 5px 15px; cursor: pointer;",
                    onclick: move |_| {
                        *QR_RES.write() = IndexMap::new();
                        *QR_INDEX.write() = 0;
                    },
                    "← Back"
                }
            }
            div { class: "qr", dangerous_inner_html: "{svg}" }
            div { style: "margin-top: 10px; font-size: 16px;", "{title}" }

            div { style: "margin-top: 20px; display: flex; align-items: center; gap: 10px;",
                button {
                    style: "font-size: 24px; padding: 5px 15px; cursor: pointer;",
                    onclick: move |_| {
                        let mut idx = *QR_INDEX.read();
                        if idx > 0 {
                            idx -= 1;
                        } else {
                            idx = total - 1;
                        }
                        *QR_INDEX.write() = idx;
                    },
                    "◄"
                }
                button {
                    style: "font-size: 24px; padding: 5px 15px; cursor: pointer;",
                    onclick: move |_| {
                        let current = *is_playing.read();
                        *is_playing.write() = !current;
                    },
                    "{play_pause_text}"
                }
                button {
                    style: "font-size: 24px; padding: 5px 15px; cursor: pointer;",
                    onclick: move |_| {
                        let idx = (*QR_INDEX.read() + 1) % total;
                        *QR_INDEX.write() = idx;
                    },
                    "►"
                }

                input {
                    r#type: "range",
                    style: "width: 300px;",
                    min: "0",
                    max: "{total - 1}",
                    value: "{current_index}",
                    oninput: move |evt| {
                        if let Ok(val) = evt.value().parse::<usize>() {
                            *QR_INDEX.write() = val;
                        }
                    },
                }

                select {
                    style: "font-size: 16px; padding: 5px;",
                    value: "{speed_text}",
                    onchange: move |evt| {
                        let speed_str = evt.value();
                        if let Ok(speed) = speed_str.trim_end_matches('x').parse::<f32>() {
                            *playback_speed.write() = speed;
                        }
                    },
                    option { value: "0.5x", "0.5x" }
                    option { value: "1x", "1x" }
                    option { value: "2x", "2x" }
                    option { value: "5x", "5x" }
                    option { value: "10x", "10x" }
                }
            }
        }
    }
}

fn send(file_name: String, data: Vec<u8>) {
    log(&format!("Sending file: {}", file_name));
    let qr = encoder::Encoder::new(file_name, data).to_qr();
    log("setting QR_RES");

    *QR_INDEX.write() = 0;
    *QR_RES.write() = qr;
    log("QR_RES set");
}

pub async fn read_file_content() {
    let window = web_sys::window().expect("no global `window` exists");
    let document = window.document().expect("should have a document on window");

    let progress_div = document
        .get_element_by_id("progress")
        .expect("should have a progress_div element");
    progress_div.set_inner_html("Processing...");

    let filelist = document
        .get_element_by_id("file-selector")
        .expect("should have a file-selector element")
        .dyn_into::<web_sys::HtmlInputElement>()
        .unwrap()
        .files()
        .expect("Failed to get filelist from File Input!");

    let file = filelist.get(0).expect("Failed to get File from filelist!");
    let file_name = file.name();
    log(&file_name);

    let file_reader = web_sys::FileReader::new().unwrap();

    let fr_c = file_reader.clone();

    let (rx, tx) = futures::channel::oneshot::channel();
    let onloadend_cb: Closure<dyn FnMut()> = Closure::new({
        let mut rx = Some(rx);
        move || {
            let array = js_sys::Uint8Array::new(&fr_c.result().unwrap());
            let _ = rx
                .take()
                .expect("multiple files read without refreshing the channel")
                .send(array.to_vec());
        }
    });

    file_reader.set_onloadend(Some(onloadend_cb.as_ref().unchecked_ref()));
    onloadend_cb.forget();
    file_reader
        .read_as_array_buffer(&file)
        .expect("blob not readable");

    let array = tx.await.unwrap();
    send(file_name.clone(), array);
}
