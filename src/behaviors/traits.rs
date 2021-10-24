use web_sys::CanvasRenderingContext2d;

use super::BehaviorType;
use crate::callback::ErasedFnPointer;
use crate::game::Game;
use crate::sprites::{Pos, SpritePointer, Update};

pub trait BehaviorState {
    fn start(&mut self, _now: f64);

    fn stop(&mut self, _now: f64);

    fn is_running(&self) -> bool;

    fn toggle(&mut self, run: bool, now: f64) {
        match run {
            true => self.start(now),
            false => self.stop(now),
        }
    }
}

pub trait BehaviorCallback {
    fn set_sprite(&mut self, sprite: *mut dyn Update);

    fn add_callback(&mut self, _callback: ErasedFnPointer<SpritePointer>) {}

    fn execute_callback(&self) {}
}

pub trait Behavior: BehaviorState + BehaviorCallback {
    fn name(&self) -> BehaviorType;

    fn execute(
        &mut self,
        now: f64,
        last_animation_frame_time: f64,
        mouse_pos: &Pos,
        context: &CanvasRenderingContext2d,
    );

    fn set_game(&mut self, _game: *mut Game) {}

    fn set_infinite(&mut self, _infinite: bool) {}
}
