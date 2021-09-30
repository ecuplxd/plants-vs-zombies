use derives::{derive_behavior, WithCallback, WithTimer};
use web_sys::CanvasRenderingContext2d;

use super::{Behavior, BehaviorType};
use crate::callback::ErasedFnPointer;
use crate::sprites::{Pos, SpritePointer, Update};
use crate::timer::{AnimationTimer, Time};

#[derive_behavior("with_callback")]
#[derive(Default, WithTimer, WithCallback)]
pub struct IntervalBehavior {
    name: BehaviorType,
    timer: AnimationTimer,
}

impl IntervalBehavior {
    pub fn new(interval: f64) -> IntervalBehavior {
        IntervalBehavior {
            name: BehaviorType::Interval,
            timer: AnimationTimer::new(interval),
            ..Default::default()
        }
    }
}

impl Behavior for IntervalBehavior {
    fn name(&self) -> BehaviorType {
        self.name
    }

    fn execute(
        &mut self,
        now: f64,
        _last_animation_frame_time: f64,
        _mouse_pos: &Pos,
        _context: &CanvasRenderingContext2d,
    ) {
        if self.sprite.is_some() && self.timer.is_expired(now) {
            self.start(now);
            self.execute_callback();
        }
    }
}
