pub mod encoder;
mod scroll;

use crate::utils::log;

use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;

use dioxus::prelude::*;
use indexmap::IndexMap;

pub use scroll::toggle_scroll;

// Remember: owned props must implement PartialEq!
#[derive(PartialEq, Props, Default)]
pub struct QrRes {
    pub payloads: IndexMap<String, String>,
}

fn send(qrres: dioxus::hooks::UseState<QrRes>, file_name: String, int_array: Vec<u8>) {
    qrres.set(QrRes {
        payloads: { encoder::Encoder::new(file_name, int_array).to_qr() },
    });
}

pub fn read_file_content(qrres: dioxus::hooks::UseState<QrRes>) {
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
    let onloadend_cb = Closure::wrap(Box::new(move |_e: web_sys::ProgressEvent| {
        let array = js_sys::Uint8Array::new(&fr_c.result().unwrap());
        send(qrres.clone(), file_name.clone(), array.to_vec());
    }) as Box<dyn Fn(web_sys::ProgressEvent)>);

    file_reader.set_onloadend(Some(onloadend_cb.as_ref().unchecked_ref()));
    file_reader
        .read_as_array_buffer(&file)
        .expect("blob not readable");
    onloadend_cb.forget();
}
