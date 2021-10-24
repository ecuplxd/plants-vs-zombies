use derives::{derive_behavior, WithCallback, WithoutTimer};
use web_sys::CanvasRenderingContext2d;

use super::{Behavior, BehaviorType};
use crate::callback::ErasedFnPointer;
use crate::sprites::{Pos, SpriteCell, SpritePointer, Update};

#[derive_behavior("default")]
#[derive(Default, WithoutTimer, WithCallback)]
pub struct Switch {
    name: BehaviorType,
    switched: bool,
    switch_index: u8,
    cells: Vec<Vec<SpriteCell>>,
    infinite: bool,
    last_finished_time: f64,
    duration: f64,
}

impl Switch {
    pub fn new(cells: Vec<Vec<SpriteCell>>, duration: f64, infinite: bool) -> Switch {
        Switch {
            name: BehaviorType::Switch,
            switch_index: 99,
            infinite,
            cells,
            duration,
            ..Default::default()
        }
    }

    fn switch(&mut self, switch_index: u8, now: f64) {
        unsafe {
            let sprite = self.sprite.unwrap().as_mut();
            let artist = sprite.get_mut_artist();

            artist.switch(&self.cells[switch_index as usize]);

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

    fn update(&mut self, switch_index: u8, switched: bool, now: f64) {
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
}

impl Behavior for Switch {
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

    fn set_infinite(&mut self, infinite: bool) {
        self.infinite = infinite;
    }
}
