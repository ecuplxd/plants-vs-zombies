use crate::{callback::ErasedFnPointer, timer::animation_timer::AnimationTimer};

use super::model::{Behavior, BehaviorType};

pub struct IntervalBehavior {
    name: BehaviorType,
    timer: AnimationTimer,
    cb: Option<ErasedFnPointer>,
}

impl IntervalBehavior {
    pub fn new(interval: f64) -> IntervalBehavior {
        IntervalBehavior {
            name: BehaviorType::Interval,
            timer: AnimationTimer::new(interval),
            cb: None,
        }
    }

    fn execute_callback(&self) {
        if let Some(cb) = &self.cb {
            cb.call();
        }
    }
}

impl Behavior for IntervalBehavior {
    fn execute(
        &mut self,
        _sprite: &mut dyn crate::sprites::model::Update,
        now: f64,
        _last_animation_frame_time: f64,
        _mouse_pos: &crate::sprites::model::Pos,
        _context: &web_sys::CanvasRenderingContext2d,
    ) {
        if self.timer.is_expired(now) {
            self.start(now);
            self.execute_callback();
        }
    }

    fn name(&self) -> BehaviorType {
        self.name
    }

    fn get_timer(&mut self) -> Option<&mut AnimationTimer> {
        Some(&mut self.timer)
    }

    fn set_cb(&mut self, cb: ErasedFnPointer) {
        self.cb = Some(cb);
    }
}
