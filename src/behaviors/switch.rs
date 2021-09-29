use std::ptr::NonNull;

use web_sys::CanvasRenderingContext2d;

use super::{Behavior, BehaviorType};
use crate::callback::ErasedFnPointer;
use crate::sprites::{Pos, SpriteCell, SpritePointer, Update};

pub struct SwitchBehavior {
    name: BehaviorType,
    switched: bool,
    switch_index: usize,
    cells: Vec<Vec<SpriteCell>>,
    infinite: bool,
    sprite: SpritePointer,
    last_finished_time: f64,
    duration: f64,
    running: bool,
    cb: Option<ErasedFnPointer<SpritePointer>>,
}

impl SwitchBehavior {
    pub fn new(cells: Vec<Vec<SpriteCell>>, duration: f64, infinite: bool) -> SwitchBehavior {
        SwitchBehavior {
            name: BehaviorType::Switch,
            switch_index: 99,
            infinite,
            cells,
            switched: false,
            sprite: None,
            duration,
            last_finished_time: 0.0,
            running: false,
            cb: None,
        }
    }

    fn switch(&mut self, switch_index: usize, now: f64) {
        unsafe {
            let sprite = self.sprite.unwrap().as_mut();
            let artist = sprite.get_mut_artist();

            artist.switch(&self.cells[switch_index]);

            self.update(switch_index, false, now);
        }
    }

    fn revert(&mut self, now: f64) {
        unsafe {
            let sprite = self.sprite.unwrap().as_mut();
            let artist = sprite.get_mut_artist();

            artist.revert();

            self.update(99, false, now);
            self.execute_callback();
        }
    }

    fn update(&mut self, switch_index: usize, switched: bool, now: f64) {
        unsafe {
            let sprite = self.sprite.unwrap().as_mut();

            // switch 后导致前后 cell 宽高可能不一致
            sprite.update_pos(sprite.get_pos());
        }

        self.switch_index = switch_index;
        self.switched = switched;
        self.last_finished_time = now;
    }

    fn is_finished(&self, now: f64) -> bool {
        now - self.last_finished_time > self.duration
    }

    fn execute_callback(&self) {
        match self.cb {
            Some(cb) => cb.call(self.sprite),
            _ => (),
        }
    }
}

impl Behavior for SwitchBehavior {
    fn name(&self) -> BehaviorType {
        self.name
    }

    fn start(&mut self, _now: f64) {
        self.running = true;
    }

    fn stop(&mut self, _now: f64) {
        self.running = false;
    }

    fn is_running(&self) -> bool {
        self.running
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
                let (trigger, index) = sprite.as_mut().tirgger_switch();

                self.switched = index == self.switch_index;

                match trigger {
                    true if !self.switched => self.switch(index, now),
                    true if !self.infinite && self.is_finished(now) => self.revert(now),
                    _ => (),
                }
            }
        }
    }

    fn set_sprite(&mut self, sprite: *mut dyn Update) {
        self.sprite = NonNull::new(sprite);
    }

    fn set_cb(&mut self, cb: ErasedFnPointer<SpritePointer>) {
        self.cb = Some(cb);
    }
}
