use web_sys::CanvasRenderingContext2d;

use crate::{
    callback::ErasedFnPointer,
    fps::Fps,
    sprites::model::{DrawInfo, Offset, Pos, Update},
};

use super::model::{Behavior, BehaviorType};

pub struct ScrollBehavior {
    name: BehaviorType,
    rate: f64,
    distance: f64,
    offset: f64,
    working: bool,
    cb: Option<ErasedFnPointer>,
}

impl ScrollBehavior {
    pub fn new(distance: f64, rate: f64) -> ScrollBehavior {
        ScrollBehavior {
            name: BehaviorType::Scroll,
            rate,
            distance,
            offset: 0.0,
            working: false,
            cb: None,
        }
    }

    fn turn_right(&mut self, now: f64) {
        self.rate = self.rate.abs();
        self.stop(now);
        self.execute_callback();
    }

    fn turn_left(&mut self, now: f64) {
        self.rate = -self.rate;
        self.stop(now);
        self.execute_callback();
    }

    fn execute_callback(&self) {
        if let Some(cb) = &self.cb {
            cb.call();
        }
    }
}

impl Behavior for ScrollBehavior {
    fn execute(
        &mut self,
        sprite: &mut dyn Update,
        now: f64,
        last_animation_frame_time: f64,
        _mouse_pos: &Pos,
        _context: &CanvasRenderingContext2d,
    ) {
        let frame_offset = Fps::cal_pixel_frame(self.rate, now, last_animation_frame_time);

        self.offset += frame_offset;

        match self.offset >= self.distance {
            true => self.turn_left(now),
            false if self.offset <= 0.0 => self.turn_right(now),
            false => (),
        }

        let DrawInfo { offset, .. } = *sprite.get_draw_info().unwrap();
        let new_offset = Offset::new(offset.x + frame_offset, offset.y);

        sprite.update_draw_info(None, Some(new_offset));
    }

    fn is_working(&mut self) -> bool {
        self.working
    }

    fn start(&mut self, _now: f64) {
        self.working = true;
    }

    fn stop(&mut self, _now: f64) {
        self.working = false;
    }

    fn set_cb(&mut self, cb: ErasedFnPointer) {
        self.cb = Some(cb);
    }

    fn name(&self) -> BehaviorType {
        self.name
    }
}
