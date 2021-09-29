use std::ptr::NonNull;

use web_sys::CanvasRenderingContext2d;

use super::{Behavior, BehaviorType};
use crate::callback::ErasedFnPointer;
use crate::sprites::{Pos, SpritePointer, Update};
use crate::timer::{AnimationTimer, Time};

pub struct IntervalBehavior {
    name: BehaviorType,
    timer: AnimationTimer,
    sprite: SpritePointer,
    cb: Option<ErasedFnPointer<SpritePointer>>,
}

impl IntervalBehavior {
    pub fn new(interval: f64) -> IntervalBehavior {
        IntervalBehavior {
            name: BehaviorType::Interval,
            timer: AnimationTimer::new(interval),
            sprite: None,
            cb: None,
        }
    }

    fn execute_callback(&self) {
        match self.cb {
            Some(cb) => cb.call(self.sprite),
            _ => (),
        }
    }
}

impl Behavior for IntervalBehavior {
    fn name(&self) -> BehaviorType {
        self.name
    }

    fn start(&mut self, now: f64) {
        self.timer.start(now);
    }

    fn stop(&mut self, now: f64) {
        self.timer.stop(now);
    }

    fn is_running(&self) -> bool {
        self.timer.is_running()
    }

    fn set_sprite(&mut self, sprite: *mut dyn Update) {
        self.sprite = NonNull::new(sprite);
    }

    fn set_cb(&mut self, cb: ErasedFnPointer<SpritePointer>) {
        self.cb = Some(cb);
    }

    fn execute(
        &mut self,
        now: f64,
        _last_animation_frame_time: f64,
        _mouse_pos: &Pos,
        _context: &CanvasRenderingContext2d,
    ) {
        if let Some(_) = self.sprite {
            if self.timer.is_expired(now) {
                self.start(now);
                self.execute_callback();
            }
        }
    }
}
