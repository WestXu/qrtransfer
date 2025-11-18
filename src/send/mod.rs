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

    let total = props.payloads.len();

    use_effect(move || {
        if total > 0 {
            spawn(async move {
                loop {
                    gloo_timers::future::TimeoutFuture::new(500).await;
                    let current = *QR_INDEX.read();
                    *QR_INDEX.write() = (current + 1) % total;
                }
            });
        }
    });

    if props.payloads.is_empty() {
        return rsx! { div {} };
    }

    let current_index = *qr_index.read() % total;
    let (_name, svg) = props.payloads.get_index(current_index).unwrap();

    rsx! {
        div {
            style: "display: flex; flex-direction: column; align-items: center; justify-content: center; height: 100vh;",
            div {
                class: "qr",
                dangerous_inner_html: "{svg}"
            }
            div {
                style: "margin-top: 20px; font-size: 24px;",
                "{current_index + 1}/{total}"
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
