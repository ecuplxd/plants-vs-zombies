use derives::{WithCallback, WithTimer};

use super::{Behavior, BehaviorType, Cycle};
use crate::artists::Draw;
use crate::callback::ErasedFnPointer;
use crate::sprites::{SpritePointer, Update};
use crate::timer::Elapsed;

#[derive(WithTimer, WithCallback)]
#[behavior(cycle)]
pub struct Frequency {
    name: BehaviorType,
    total: usize,
    cycle: Cycle,
    count: usize,
    delay_start: f64,
    delay_execute_callback: f64,
    callbacks: Vec<ErasedFnPointer<SpritePointer>>,
}

impl Frequency {
    pub fn new(duration: f64, delay_start: f64) -> Frequency {
        Frequency {
            count: 0,
            total: 2,
            name: BehaviorType::Frequency,
            cycle: Cycle::new(duration, None),
            delay_execute_callback: 1000.0,
            delay_start,
            callbacks: vec![],
        }
    }

    fn finished(&self) -> bool {
        self.count == self.total - 1
    }

    fn should_execute_callback(&self, now: f64) -> bool {
        now - self.cycle.last_advance > self.delay_execute_callback
    }

    fn cycle_finished(&self, now: f64, artist: &dyn Draw) -> bool {
        let one_frame_passed = now - self.cycle.last_advance > self.cycle.timer.duration;

        artist.in_last_cell() && one_frame_passed
    }
}

impl Behavior for Frequency {
    fn name(&self) -> BehaviorType {
        self.name
    }

    fn execute(
        &mut self,
        now: f64,
        last_animation_frame_time: f64,
        mouse_pos: &crate::sprites::Pos,
        context: &web_sys::CanvasRenderingContext2d,
    ) {
        if self.cycle.timer.get_elapsed_time(now) < self.delay_start {
            return;
        }

        if let Some(mut sprite) = self.cycle.sprite {
            unsafe {
                match self.finished() {
                    true if self.should_execute_callback(now) => {
                        sprite.as_mut().hide();

                        self.stop(now);
                        self.execute_callback();
                    }
                    true => (),
                    false => {
                        let artist = sprite.as_ref().get_ref_artist();

                        if self.cycle_finished(now, artist) {
                            self.count += 1;
                        }

                        if !self.finished() {
                            self.cycle
                                .execute(now, last_animation_frame_time, mouse_pos, context);
                        }
                    }
                }
            }
        }
    }
}
