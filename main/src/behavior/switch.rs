use web_sys::CanvasRenderingContext2d;

use crate::{
    callback::ErasedFnPointer,
    sprites::model::{Pos, SpriteCell, Update},
    timer::animation_timer::AnimationTimer,
};

use super::model::{Behavior, BehaviorType};

pub struct SwitchBehavior {
    name: BehaviorType,
    switched: bool,
    timer: AnimationTimer,
    switch_index: usize,
    cells: Vec<Vec<SpriteCell>>,
    infinite: bool,
    cb: Option<ErasedFnPointer>,
}

impl SwitchBehavior {
    pub fn new(cells: Vec<Vec<SpriteCell>>, duration: f64, infinite: bool) -> SwitchBehavior {
        SwitchBehavior {
            name: BehaviorType::Switch,
            switch_index: 99,
            cells,
            timer: AnimationTimer::new(duration),
            infinite,
            switched: false,
            cb: None,
        }
    }

    fn update_pos(&mut self, sprite: &mut dyn Update, revert: bool) {
        let pos = sprite.get_draw_info().unwrap().pos;
        let artist = sprite.get_artist();
        let cur_cell = artist.get_current_cell().unwrap();
        let cur_height = cur_cell.height;
        let height = self.cells[self.switch_index][0].height;
        let delta_height = cur_height - height;
        let dir = match revert {
            true => -1.0,
            false => 1.0,
        };
        let new_pos = Pos::new(pos.left, pos.top + delta_height * dir);

        sprite.update_draw_info(Some(new_pos), None);
    }

    fn switch(&mut self, sprite: &mut dyn Update, switch_index: usize, now: f64) {
        let artist = sprite.get_artist();

        artist.switch(&self.cells[switch_index]);

        // self.update_pos(sprite, false);
        self.update(switch_index, false, now);
    }

    fn revert(&mut self, sprite: &mut dyn Update, now: f64) {
        let artist = sprite.get_artist();

        artist.revert();

        // self.update_pos(sprite, true);
        self.update(99, false, now);
        self.execute_callback();
    }

    fn update(&mut self, switch_index: usize, switched: bool, now: f64) {
        self.switch_index = switch_index;
        self.switched = switched;
        self.timer.finisehd(now);
    }

    fn execute_callback(&self) {
        if let Some(cb) = &self.cb {
            cb.call();
        }
    }
}

impl Behavior for SwitchBehavior {
    fn execute(
        &mut self,
        sprite: &mut dyn Update,
        now: f64,
        _last_animation_frame_time: f64,
        _mouse_pos: &Pos,
        _context: &CanvasRenderingContext2d,
    ) {
        let (trigger, index) = sprite.tirgger_switch();

        self.switched = index == self.switch_index;

        match trigger {
            true if !self.switched => self.switch(sprite, index, now),
            true if !self.infinite && self.timer.is_finished(now) => self.revert(sprite, now),
            _ => (),
        }
    }

    fn get_timer(&mut self) -> Option<&mut AnimationTimer> {
        Some(&mut self.timer)
    }

    fn set_cb(&mut self, cb: ErasedFnPointer) {
        self.cb = Some(cb);
    }

    fn name(&self) -> BehaviorType {
        self.name
    }
}
