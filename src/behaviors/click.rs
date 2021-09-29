use derives::{derive_behavior, WithCallback, WithoutTimer};
use web_sys::CanvasRenderingContext2d;

use super::{Behavior, BehaviorType};
use crate::callback::ErasedFnPointer;
use crate::sprites::{Pos, SpritePointer, Update};
use crate::util::{has_sprite_clicked, set_sprite_clicked};

#[derive_behavior("default")]
#[derive(Default, WithoutTimer, WithCallback)]
pub struct ClickBehavior {
    name: BehaviorType,
}

impl ClickBehavior {
    pub fn new() -> ClickBehavior {
        ClickBehavior {
            name: BehaviorType::Click,
            ..Default::default()
        }
    }
}

impl Behavior for ClickBehavior {
    fn name(&self) -> BehaviorType {
        self.name
    }

    fn execute(
        &mut self,
        now: f64,
        _last_animation_frame_time: f64,
        mouse_pos: &Pos,
        context: &CanvasRenderingContext2d,
    ) {
        if let Some(mut sprite) = self.sprite {
            unsafe {
                self.stop(now);

                if has_sprite_clicked() {
                    sprite.as_mut().set_clicked(false);

                    return;
                }

                let clicked = sprite.as_ref().point_in_path(mouse_pos, context);

                sprite.as_mut().set_clicked(clicked);

                if clicked {
                    set_sprite_clicked("clicked");
                    self.execute_callback();
                }
            }
        }
    }
}
