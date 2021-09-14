use web_sys::CanvasRenderingContext2d;

use crate::{
    artists::model::Draw,
    sprites::model::{Pos, Update},
    timer::animation_timer::AnimationTimer,
};

use super::model::{Behavior, BehaviorType};

pub struct CycleBehavior {
    name: BehaviorType,
    pub duration: f64,
    interval: Option<f64>,
    pub last_advance: f64,
    pub timer: AnimationTimer,
}

impl CycleBehavior {
    pub fn new(duration: f64, interval: Option<f64>) -> CycleBehavior {
        CycleBehavior {
            name: BehaviorType::Cycle,
            duration,
            interval,
            last_advance: 0.0,
            timer: AnimationTimer::new(duration),
        }
    }

    fn advance(&mut self, artist: &mut dyn Draw, now: f64) {
        artist.advance();

        self.last_advance = now;
    }
}

impl Behavior for CycleBehavior {
    fn execute(
        &mut self,
        sprite: &mut dyn Update,
        now: f64,
        _last_animation_frame_time: f64,
        _mouse_pos: &Pos,
        _context: &CanvasRenderingContext2d,
    ) {
        let last_advance = self.last_advance;
        let one_frame_passed = now - last_advance > self.duration;
        let artist = sprite.get_artist();

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

    fn get_timer(&mut self) -> Option<&mut AnimationTimer> {
        Some(&mut self.timer)
    }

    fn name(&self) -> BehaviorType {
        self.name
    }
}
