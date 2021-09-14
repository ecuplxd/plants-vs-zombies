use js_sys::Math;
use wasm_bindgen::prelude::*;
use wasm_bindgen::{prelude::Closure, JsCast};

pub fn window() -> web_sys::Window {
    web_sys::window().expect("no global `window` exists")
}

pub fn request_animation_frame(f: &Closure<dyn FnMut()>) {
    window()
        .request_animation_frame(f.as_ref().unchecked_ref())
        .expect("should register `requestAnimationFrame` OK");
}

#[wasm_bindgen]
pub fn init() {
    console_error_panic_hook::set_once();
}

pub fn get_random_int(min: f64, max: f64) -> f64 {
    let min = Math::ceil(min);
    let max = Math::floor(max);

    return Math::floor(Math::random() * (max - min)) + min;
}

#[macro_export]
macro_rules! log {
    ($($t:tt)*) => {
        web_sys::console::log_1(&format!($($t)*).into())
    }
}

#[macro_export]
macro_rules! asset_json {
    ($name: tt) => {
        concat!("main/assets/jsons/", $name, ".json")
    };
}

#[macro_export]
macro_rules! asset_image {
    ($name: tt) => {
        (concat!("main/assets/images/", $name), ".png")
    };
    ($name: tt, $ext: tt) => {
        (concat!("main/assets/images/", $name), concat!(".", $ext))
    };
}
