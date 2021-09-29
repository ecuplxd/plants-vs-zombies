use std::ptr::NonNull;

use web_sys::CanvasRenderingContext2d;

use super::{Behavior, BehaviorType};
use crate::fps::Fps;
use crate::loc::Loc;
use crate::sprites::{Pos, SpritePointer, Update, Velocit};
use crate::timer::{AnimationTimer, Time};

pub struct WalkBehavior {
    name: BehaviorType,
    velocit: Velocit,
    timer: AnimationTimer,
    sprite: SpritePointer,
    distance: Option<f64>,
    walked_distance: f64,
}

impl WalkBehavior {
    pub fn new(velocit: Velocit, duration: f64, distance: Option<f64>) -> WalkBehavior {
        WalkBehavior {
            name: BehaviorType::Walk,
            velocit,
            timer: AnimationTimer::new(duration),
            sprite: None,
            distance,
            walked_distance: 0.0,
        }
    }
}

impl Behavior for WalkBehavior {
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

    fn execute(
        &mut self,
        now: f64,
        last_animation_frame_time: f64,
        _mouse_pos: &Pos,
        _context: &CanvasRenderingContext2d,
    ) {
        if let Some(mut sprite) = self.sprite {
            unsafe {
                let offset = Fps::cal_velocit_offset(&self.velocit, now, last_animation_frame_time);
                let new_pos = sprite.as_ref().get_pos() + offset;

                self.walked_distance += offset.distance();

                match self.distance {
                    Some(distance) if self.walked_distance > distance => self.stop(now),
                    _ => (),
                }

                match new_pos.out_of_bound() {
                    true => sprite.as_mut().toggle(),
                    false => {
                        let loc = Loc::get_row_col_by_pos(&new_pos);

                        sprite.as_mut().update_loc(loc);
                        sprite.as_mut().update_pos(new_pos);
                        sprite.as_mut().update_outlines();
                    }
                }
            }
        }
    }

    fn set_sprite(&mut self, sprite: *mut dyn Update) {
        self.sprite = NonNull::new(sprite);
    }
}