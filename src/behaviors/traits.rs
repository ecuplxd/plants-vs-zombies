use web_sys::CanvasRenderingContext2d;

use super::BehaviorType;
use crate::callback::ErasedFnPointer;
use crate::game::Game;
use crate::sprites::{Pos, SpritePointer, Update};

pub trait Behavior {
    fn name(&self) -> BehaviorType;

    fn start(&mut self, _now: f64);

    fn stop(&mut self, _now: f64);

    fn toggle(&mut self, run: bool, now: f64) {
        match run {
            true => self.start(now),
            false => self.stop(now),
        }
    }

    fn is_running(&self) -> bool;

    fn execute(
        &mut self,
        now: f64,
        last_animation_frame_time: f64,
        mouse_pos: &Pos,
        context: &CanvasRenderingContext2d,
    );

    fn set_cb(&mut self, _cb: ErasedFnPointer<SpritePointer>) {}

    fn set_sprite(&mut self, sprite: *mut dyn Update);

    fn set_game(&mut self, _game: *mut Game) {}
}
