mod decoder;

use std::collections::HashMap;
use std::sync::Arc;
use std::sync::Mutex;
use wasm_bindgen::closure::Closure;
use wasm_bindgen::JsCast;
use wasm_bindgen::JsValue;
use wasm_bindgen_futures::spawn_local;
use wasm_bindgen_futures::JsFuture;
use web_sys::{AudioContext, OscillatorType};
use web_sys::{
    CanvasRenderingContext2d, HtmlCanvasElement, HtmlVideoElement, MediaStream,
    MediaStreamConstraints,
};

pub use decoder::Decoder;

fn beep(audio_context: &AudioContext, freq: f32, duration: f64, vol: f32) {
    let oscillator = audio_context.create_oscillator().unwrap();
    let gain = audio_context.create_gain().unwrap();

    oscillator.connect_with_audio_node(&gain).unwrap();
    gain.connect_with_audio_node(&audio_context.destination())
        .unwrap();

    oscillator.frequency().set_value(freq);
    oscillator.set_type(OscillatorType::Square);

    gain.gain().set_value(vol * 0.01);

    oscillator
        .start_with_when(audio_context.current_time())
        .unwrap();
    oscillator
        .stop_with_when(audio_context.current_time() + duration * 0.001)
        .unwrap();
}

async fn sleep(ms: i32) {
    let promise = js_sys::Promise::new(&mut |resolve, _| {
        let window = web_sys::window().unwrap();
        window
            .set_timeout_with_callback_and_timeout_and_arguments_0(&resolve, ms)
            .unwrap();
    });

    JsFuture::from(promise).await.unwrap();
}

async fn beep_n(n: i32) {
    let audio_context = AudioContext::new().unwrap();
    for _ in 0..n {
        beep(&audio_context, 1500.0, 30.0, 100.0);
        sleep(50).await;
    }
}

fn add_download(base64_data: &str, file_name: &str) {
    let document = web_sys::window().unwrap().document().unwrap();
    let a = document.create_element("a").unwrap();
    a.set_attribute("href", &format!("data:;base64,{}", base64_data))
        .unwrap();
    a.set_attribute("download", file_name).unwrap();
    a.set_inner_html("Download");
    document
        .get_element_by_id("receive")
        .unwrap()
        .append_child(&a)
        .unwrap();
    let a: web_sys::HtmlElement = a.dyn_into().unwrap();
    a.click();
}

pub async fn start_receiving() {
    let window = web_sys::window().unwrap();
    let navigator = window.navigator();
    let media_devices = navigator.media_devices().unwrap();
    let stream_promise = media_devices
        .get_user_media_with_constraints(&{
            let constraints = MediaStreamConstraints::new();
            constraints.set_video(
                &serde_wasm_bindgen::to_value(&HashMap::from([(
                    "facingMode",
                    "environment".to_string(),
                )]))
                .unwrap(),
            );
            constraints
        })
        .unwrap();

    let stream = JsFuture::from(stream_promise)
        .await
        .unwrap()
        .dyn_into::<MediaStream>()
        .unwrap();

    js_sys::Reflect::set(&window, &JsValue::from("stream"), &JsValue::from(&stream)).unwrap();

    let document = web_sys::window().unwrap().document().unwrap();
    let video = document
        .get_element_by_id("scan-video")
        .unwrap()
        .dyn_into::<HtmlVideoElement>()
        .unwrap();
    let canvas = document
        .get_element_by_id("canvas")
        .unwrap()
        .dyn_into::<HtmlCanvasElement>()
        .unwrap();
    let cam_qr_result = document.get_element_by_id("cam-qr-result").unwrap();
    let ctx = canvas
        .get_context("2d")
        .unwrap()
        .unwrap()
        .dyn_into::<CanvasRenderingContext2d>()
        .unwrap();

    let decoder = Arc::new(Mutex::new(Some(Decoder::new())));

    video.set_src_object(Some(&stream));
    let video_clone = video.clone();
    let interval_closure = Closure::wrap(Box::new(move || {
        canvas.set_width(video_clone.video_width());
        canvas.set_height(video_clone.video_height());
        ctx.draw_image_with_html_video_element(&video_clone, 0.0, 0.0)
            .unwrap();
        let Ok(my_image_data) =
            ctx.get_image_data(0.0, 0.0, canvas.width() as f64, canvas.height() as f64)
        else {
            return;
        };

        let counter = decoder.lock().unwrap().as_mut().unwrap().scan(
            canvas.width() as u32,
            canvas.height() as u32,
            my_image_data.data().to_vec(),
        );
        if counter > 0 {
            cam_qr_result.set_text_content(Some(
                &decoder.lock().unwrap().as_mut().unwrap().get_progress(),
            ));
            let beep_n_closure = beep_n(counter as i32);
            spawn_local(beep_n_closure);

            if decoder.lock().unwrap().as_mut().unwrap().is_finished() {
                stop_receiving();
                let finished = decoder.lock().unwrap().take().unwrap().get_finished();
                add_download(&finished.to_base64(), &finished.get_name());
            }
        }
    }) as Box<dyn FnMut()>);

    let interval_id = web_sys::window()
        .unwrap()
        .set_interval_with_callback_and_timeout_and_arguments_0(
            interval_closure.as_ref().unchecked_ref(),
            40,
        )
        .unwrap();
    js_sys::Reflect::set(
        &window,
        &JsValue::from("intervalId"),
        &JsValue::from(interval_id),
    )
    .unwrap();

    interval_closure.forget();
}

pub fn stop_receiving() {
    let window = web_sys::window().unwrap();
    let stream = js_sys::Reflect::get(&window, &"stream".into())
        .unwrap()
        .dyn_into::<web_sys::MediaStream>()
        .unwrap();
    for track in stream.get_tracks().to_vec() {
        track
            .dyn_into::<web_sys::MediaStreamTrack>()
            .unwrap()
            .stop();
    }
    let interval_id = js_sys::Reflect::get(&window, &"intervalId".into())
        .unwrap()
        .as_f64()
        .unwrap() as i32;
    window.clear_interval_with_handle(interval_id);
    let document = window.document().unwrap();
    let canvas = document
        .get_element_by_id("canvas")
        .unwrap()
        .dyn_into::<HtmlCanvasElement>()
        .unwrap();
    let ctx = canvas
        .get_context("2d")
        .unwrap()
        .unwrap()
        .dyn_into::<CanvasRenderingContext2d>()
        .unwrap();
    ctx.clear_rect(0.0, 0.0, canvas.width() as f64, canvas.height() as f64);
}
