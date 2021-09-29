#![feature(macro_attributes_in_derive_output)]

use engine::Engine;
use util::init;
use wasm_bindgen::prelude::*;

mod artists;
mod behaviors;
mod callback;
mod engine;
mod fps;
mod game;
mod loader;
mod loc;
mod marching_squares;
mod model;
mod scenes;
mod sprites;
mod time_system;
mod timer;
mod util;

#[wasm_bindgen(start)]
pub fn run() -> Result<(), JsValue> {
    #[cfg(debug_assertions)]
    init();

    Engine::launch();

    Ok(())
}
