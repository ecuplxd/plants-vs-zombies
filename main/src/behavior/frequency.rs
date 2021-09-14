use web_sys::CanvasRenderingContext2d;

use crate::{
    callback::ErasedFnPointer,
    sprites::model::{Pos, Update},
    timer::animation_timer::AnimationTimer,
};

use super::{
    cycle::CycleBehavior,
    model::{Behavior, BehaviorType},
};

pub struct FrequencyBehavior {
    name: BehaviorType,
    total: usize,
    cycle: CycleBehavior,
    count: usize,
    delay_execute_callback: f64,
    cb: Option<ErasedFnPointer>,
}

impl FrequencyBehavior {
    pub fn new(duration: f64) -> FrequencyBehavior {
        FrequencyBehavior {
            count: 0,
            total: 2,
            name: BehaviorType::Frequency,
            cycle: CycleBehavior::new(duration, None),
            delay_execute_callback: 1000.0,
            cb: None,
        }
    }

    fn finished(&self) -> bool {
        self.count == self.total - 1
    }

    fn should_execute_callback(&self, now: f64) -> bool {
        now - self.cycle.last_advance > self.delay_execute_callback
    }

    fn execute_callback(&self) {
        if let Some(cb) = &self.cb {
            cb.call();
        }
    }
}

impl Behavior for FrequencyBehavior {
    fn execute(
        &mut self,
        sprite: &mut dyn Update,
        now: f64,
        last_animation_frame_time: f64,
        mouse_pos: &Pos,
        context: &CanvasRenderingContext2d,
    ) {
        let artist = sprite.get_artist();

        match self.finished() {
            true if self.should_execute_callback(now) => {
                sprite.toggle();

                self.execute_callback();
                self.cycle.stop(now);
            }
            true => (),
            false => {
                let one_frame_passed = now - self.cycle.last_advance > self.cycle.duration;

                if artist.in_last_cell() && one_frame_passed {
                    self.count += 1;
                }

                if !self.finished() {
                    self.cycle
                        .execute(sprite, now, last_animation_frame_time, mouse_pos, context);
                }
            }
        }
    }

    fn get_timer(&mut self) -> Option<&mut AnimationTimer> {
        self.cycle.get_timer()
    }

    fn set_cb(&mut self, cb: ErasedFnPointer) {
        self.cb = Some(cb);
    }

    fn name(&self) -> BehaviorType {
        self.name
    }
}
