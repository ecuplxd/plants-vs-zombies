use web_sys::CanvasRenderingContext2d;

use crate::{
    callback::ErasedFnPointer,
    sprites::{
        model::{Pos, Update},
        sprite::Sprite,
    },
};

use super::model::{Behavior, BehaviorType};

pub struct CollisionBehavior {
    name: BehaviorType,
    working: bool,
    cb: Option<ErasedFnPointer>,
}

impl CollisionBehavior {
    pub fn new() -> CollisionBehavior {
        CollisionBehavior {
            name: BehaviorType::Collision,
            working: false,
            cb: None,
        }
    }

    pub fn is_candidate_for_collision(sprite: &Sprite, other_sprite: &Box<dyn Update>) -> bool {
        let (row, col) = (sprite.row, sprite.col);
        let (o_row, o_col) = other_sprite.get_loc();

        row == o_row && col != 0 && (o_col == col - 1)
    }

    pub fn did_collide(sprite: &Sprite, other_sprite: &Box<dyn Update>) -> bool {
        let pos = sprite.draw_info.pos;
        let collision_margin = sprite.collision_margin;
        let collision_left = pos.left + collision_margin.left;
        let cell = other_sprite.get_read_artist().get_current_cell();
        let draw_info = other_sprite.get_draw_info();

        match (cell, draw_info) {
            (Some(cell), Some(draw_info)) => {
                let o_pos = draw_info.pos;

                collision_left >= o_pos.left && collision_left <= o_pos.left + cell.width
            }
            _ => false,
        }
    }

    fn execute_callback(&self) {
        if let Some(cb) = &self.cb {
            cb.call();
        }
    }
}

impl Behavior for CollisionBehavior {
    fn execute(
        &mut self,
        sprite: &mut dyn Update,
        now: f64,
        _last_animation_frame_time: f64,
        _mouse_pos: &Pos,
        _context: &CanvasRenderingContext2d,
    ) {
        if sprite.is_collision() {
            self.execute_callback();
            self.stop(now);
        }
    }

    fn name(&self) -> BehaviorType {
        self.name
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
}
