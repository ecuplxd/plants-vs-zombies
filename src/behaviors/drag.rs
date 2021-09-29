use derives::{derive_behavior, WithoutCallback};
use web_sys::CanvasRenderingContext2d;

use super::{Behavior, BehaviorState, BehaviorType};
use crate::sprites::{Pos, SpritePointer, Update};

#[derive_behavior("no_callback")]
#[derive(Default, WithoutCallback)]
pub struct DragBehavior {
    name: BehaviorType,
    last_pos: Option<Pos>,
}

impl DragBehavior {
    pub fn new() -> DragBehavior {
        DragBehavior {
            name: BehaviorType::Drag,
            ..Default::default()
        }
    }

    fn cal_pos_offset(&mut self, orignal_pos: &Pos, mouse_pos: Pos) -> Pos {
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

impl BehaviorState for DragBehavior {
    fn start(&mut self, _now: f64) {
        self.last_pos = None;
        self.running = true;
    }

    fn stop(&mut self, _now: f64) {
        self.last_pos = None;
        self.running = false;
    }

    fn is_running(&self) -> bool {
        self.running
    }
}

impl Behavior for DragBehavior {
    fn name(&self) -> BehaviorType {
        self.name
    }

    fn execute(
        &mut self,
        _now: f64,
        _last_animation_frame_time: f64,
        mouse_pos: &Pos,
        _context: &CanvasRenderingContext2d,
    ) {
        if let Some(mut sprite) = self.sprite {
            unsafe {
                if sprite.as_ref().is_clicked() {
                    let old_pos = sprite.as_ref().get_pos();
                    let new_pos = self.cal_pos_offset(&old_pos, *mouse_pos);

                    sprite.as_mut().update_pos(new_pos);
                }
            }
        }
    }
}
