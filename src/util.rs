use js_sys::Math;
use wasm_bindgen::prelude::{Closure, *};
use wasm_bindgen::JsCast;

pub fn window() -> web_sys::Window {
    web_sys::window().expect("no global `window` exists")
}

pub fn request_animation_frame(f: &Closure<dyn FnMut()>) {
    window()
        .request_animation_frame(f.as_ref().unchecked_ref())
        .expect("should register `requestAnimationFrame` OK");
}

pub fn has_sprite_clicked() -> bool {
    !window().name().unwrap().is_empty()
}

pub fn set_sprite_clicked(clicked: &str) {
    window().set_name(clicked).unwrap();
}

#[wasm_bindgen]
pub fn init() {
    console_error_panic_hook::set_once();
}

#[inline]
pub fn get_random_int_inclusive(min: f64, max: f64) -> f64 {
    unsafe {
        let min = Math::ceil(min);
        let max = Math::floor(max);

        Math::floor(Math::random() * (max - min + 1.0)) + min
    }
}

#[inline]
pub fn get_random_string(prefix: String) -> String {
    format!("{}_{}", prefix, &Math::random().to_string()[2..12])
}

#[macro_export]
macro_rules! log {
    ($($t:tt)*) => {
        web_sys::console::log_1(&format!($($t)*).into())
    }
}

#[macro_export]
macro_rules! asset_json {
    ($name:tt) => {
        concat!("assets/jsons/", $name, ".json")
    };
}

#[macro_export]
macro_rules! asset_image {
    ($name:tt) => {
        (concat!("assets/images/", $name), ".png")
    };
    ($name:tt, $ext:tt) => {
        (concat!("assets/images/", $name), concat!(".", $ext))
    };
}
