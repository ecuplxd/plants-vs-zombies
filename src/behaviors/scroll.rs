use derives::{derive_behavior, WithCallback, WithoutTimer};
use web_sys::CanvasRenderingContext2d;

use super::{Behavior, BehaviorType};
use crate::callback::ErasedFnPointer;
use crate::fps::Fps;
use crate::sprites::{Pos, Sprite, SpritePointer, Update};

#[derive_behavior("default")]
#[derive(Default, WithoutTimer, WithCallback)]
pub struct ScrollBehavior {
    name: BehaviorType,
    rate: f64,
    distance: f64,
    offset: f64,
}

impl ScrollBehavior {
    pub fn new(distance: f64, rate: f64) -> ScrollBehavior {
        ScrollBehavior {
            name: BehaviorType::Scroll,
            rate,
            distance,
            ..Default::default()
        }
    }

    fn turn_left(&mut self, now: f64) {
        self.rate = -self.rate;
        self.stop(now);
        self.execute_callback();
    }

    fn turn_right(&mut self, now: f64) {
        self.rate = self.rate.abs();
        self.stop(now);
        self.execute_callback();
    }
}

impl Behavior for ScrollBehavior {
    fn name(&self) -> super::BehaviorType {
        self.name
    }

    fn execute(
        &mut self,
        now: f64,
        last_animation_frame_time: f64,
        _mouse_pos: &Pos,
        _context: &CanvasRenderingContext2d,
    ) {
        if let Some(mut sprite) = self.sprite {
            unsafe {
                let sprite = sprite.as_mut().as_any().downcast_mut::<Sprite>().unwrap();
                let frame_offset = Fps::cal_pixel_frame(self.rate, now, last_animation_frame_time);

                self.offset += frame_offset;

                match self.offset >= self.distance {
                    true => self.turn_left(now),
                    false if self.offset <= 0.0 => self.turn_right(now),
                    false => (),
                }

                let new_offset = sprite.offset + frame_offset;

                sprite.update_offset(new_offset);
            }
        }
    }
}
