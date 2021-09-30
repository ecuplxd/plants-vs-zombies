use derives::{WithTimer, WithoutCallback};
use web_sys::CanvasRenderingContext2d;

use super::{Behavior, BehaviorType};
use crate::artists::Draw;
use crate::sprites::{Pos, SpritePointer, Update};
use crate::timer::{AnimationTimer, Time};

#[derive(WithTimer, WithoutCallback)]
pub struct CycleBehavior {
    name: BehaviorType,
    interval: Option<f64>,
    pub sprite: SpritePointer,
    pub timer: AnimationTimer,
    pub last_advance: f64,
}

impl CycleBehavior {
    pub fn new(duration: f64, interval: Option<f64>) -> CycleBehavior {
        CycleBehavior {
            name: BehaviorType::Cycle,
            interval,
            sprite: None,
            timer: AnimationTimer::new(duration),
            last_advance: 0.0,
        }
    }

    fn advance(&mut self, artist: &mut dyn Draw, now: f64) {
        artist.advance();

        self.last_advance = now;
    }
}

impl Behavior for CycleBehavior {
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
        if let Some(mut sprite) = self.sprite {
            unsafe {
                let last_advance = self.last_advance;
                let one_frame_passed = now - last_advance > self.timer.duration;
                let artist = sprite.as_mut().get_mut_artist();

                match self.interval {
                    Some(interval) => match artist.in_last_cell() {
                        true if now - last_advance > interval => self.advance(artist, now),
                        false if one_frame_passed => self.advance(artist, now),
                        _ => (),
                    },
                    None if one_frame_passed => self.advance(artist, now),
                    _ => (),
                }
            }
        }
    }
}
