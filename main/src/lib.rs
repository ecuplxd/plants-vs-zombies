use wasm_bindgen::prelude::*;

use engine::Engine;
use util::init;

pub mod artists;
mod behavior;
mod builder;
mod callback;
mod data;
mod engine;
mod fps;
mod game;
mod loader;
mod loc;
mod marching_squares;
mod model;
mod scene;
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
