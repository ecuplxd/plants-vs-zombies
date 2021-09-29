use std::any::Any;

use web_sys::CanvasRenderingContext2d;

use super::{CollisionMargin, Pos, SpriteCell};
use crate::artists::Draw;
use crate::behaviors::{Behavior, BehaviorType};
use crate::loc::Loc;
use crate::model::SpriteType;

pub trait BaseUpdate {
    fn id(&self) -> String {
        self.name().to_string()
    }

    fn name(&self) -> SpriteType {
        SpriteType::Unknown
    }

    fn get_loc(&self) -> Loc {
        Default::default()
    }

    fn get_order(&self) -> usize {
        0
    }

    fn get_rect(&self) -> SpriteCell {
        Default::default()
    }

    fn get_collision_margin(&self) -> CollisionMargin {
        CollisionMargin::no_collision()
    }

    fn get_ref_artist(&self) -> &dyn Draw;

    fn get_mut_artist(&mut self) -> &mut dyn Draw;

    // TODO：区分 ref 和 mut get_ref_behaviors get_mut_behaviors
    fn get_mut_behaviors(&mut self) -> &mut Vec<Box<dyn Behavior>>;

    fn get_pos(&self) -> Pos {
        Default::default()
    }

    fn update_outlines(&mut self) {}

    fn update_loc(&mut self, _loc: Loc) {}

    fn is_clicked(&self) -> bool {
        false
    }

    fn set_clicked(&mut self, _clicked: bool) {}

    fn is_visible(&self) -> bool {
        true
    }

    fn toggle(&mut self) {}

    fn add_behavior(&mut self, _behavior: Box<dyn Behavior>) {}

    fn point_in_path(&self, _mouse_pos: &Pos, _context: &CanvasRenderingContext2d) -> bool {
        false
    }

    fn has_behavior(&mut self, behavior_type: BehaviorType) -> bool {
        let behavior = self.find_behavior(behavior_type);

        behavior.is_some()
    }

    fn find_behavior(&mut self, behavior_type: BehaviorType) -> Option<&mut Box<dyn Behavior>> {
        self.get_mut_behaviors()
            .iter_mut()
            .find(|behavior| behavior.name() == behavior_type)
    }

    fn can_candidate_for_collision(&self) -> bool {
        self.is_visible() && SpriteType::is_plant(self.name())
    }

    /// 需要在同一行/当前列 - 1
    fn is_candidate_for_collision(&self, other_sprite: &dyn Update) -> bool {
        let Loc { row, col } = self.get_loc();
        let Loc {
            row: o_row,
            col: o_col,
        } = other_sprite.get_loc();
        let same_row = row == o_row;
        let same_col = col == o_col;
        let pre_col = col != 0 && o_col == col - 1;

        same_row && (same_col || pre_col)
    }

    fn did_collide(&self, other_sprite: &dyn Update) -> bool {
        let pos = self.get_pos();
        let o_pos = other_sprite.get_pos();
        let collision_margin = self.get_collision_margin();
        let collision_left = pos.left + collision_margin.left;
        let cell = other_sprite.get_ref_artist().get_current_cell();

        match cell {
            Some(cell) => collision_left >= o_pos.left && collision_left <= o_pos.left + cell.width,
            None => false,
        }
    }

    fn toggle_behavior(&mut self, behavior_type: BehaviorType, run: bool, now: f64) {
        let behavior = self.find_behavior(behavior_type);

        if let Some(behavior) = behavior {
            behavior.toggle(run, now);
        }
    }
}

pub trait Update: BaseUpdate + Draw {
    fn update(
        &mut self,
        now: f64,
        last_animation_frame_time: f64,
        mouse_pos: &Pos,
        context: &CanvasRenderingContext2d,
    ) {
        self.get_mut_behaviors()
            .iter_mut()
            .filter(|behavior| behavior.is_running())
            .for_each(|behavior| {
                behavior.execute(now, last_animation_frame_time, mouse_pos, context)
            });
    }

    fn update_pos(&mut self, _pos: Pos) {}

    fn start_all_behavior(&mut self, now: f64) {
        self.get_mut_behaviors()
            .iter_mut()
            .for_each(|behavior| behavior.start(now));
    }

    fn as_any(&mut self) -> &mut dyn Any;

    fn tirgger_switch(&self) -> (bool, usize) {
        (false, 0)
    }
}
