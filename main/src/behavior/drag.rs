use web_sys::CanvasRenderingContext2d;

use crate::sprites::model::{Pos, Update};

use super::model::{Behavior, BehaviorType};

pub struct DragBehavior {
    name: BehaviorType,
    working: bool,
    last_pos: Option<Pos>,
}

impl DragBehavior {
    pub fn new() -> DragBehavior {
        DragBehavior {
            name: BehaviorType::Drag,
            working: false,
            last_pos: None,
        }
    }

    pub fn cal_pos_offset(&mut self, orignal_pos: &Pos, mouse_pos: Pos) -> Pos {
        let last_pos = match self.last_pos {
            Some(last_pos) => last_pos,
            None => mouse_pos,
        };
        let offset_left = mouse_pos.left - last_pos.left;
        let offset_top = mouse_pos.top - last_pos.top;

        self.last_pos = Some(mouse_pos);

        Pos {
            left: orignal_pos.left + offset_left,
            top: orignal_pos.top + offset_top,
        }
    }
}

impl Behavior for DragBehavior {
    fn execute(
        &mut self,
        sprite: &mut dyn Update,
        _now: f64,
        _last_animation_frame_time: f64,
        mouse_pos: &Pos,
        _context: &CanvasRenderingContext2d,
    ) {
        if sprite.is_clicked() {
            let draw_info = sprite.get_draw_info().unwrap();
            let new_pos = self.cal_pos_offset(&draw_info.pos, mouse_pos.clone());

            sprite.update_draw_info(Some(new_pos), None);
        }
    }

    fn name(&self) -> BehaviorType {
        self.name
    }

    fn is_working(&mut self) -> bool {
        self.working
    }

    fn start(&mut self, _now: f64) {
        self.last_pos = None;
        self.working = true;
    }

    fn stop(&mut self, _now: f64) {
        self.last_pos = None;
        self.working = false;
    }
}
