use derives::{derive_behavior, WithoutCallback, WithoutTimer};
use web_sys::CanvasRenderingContext2d;

use super::{Behavior, BehaviorType};
use crate::sprites::{Pos, SpritePointer, Update};

#[derive_behavior("without_callback")]
#[derive(Default, WithoutTimer, WithoutCallback)]
pub struct HoverBehavior {
    name: BehaviorType,
    moving: bool,
}

impl HoverBehavior {
    pub fn new() -> HoverBehavior {
        HoverBehavior {
            name: BehaviorType::Hover,
            ..Default::default()
        }
    }
}

impl Behavior for HoverBehavior {
    fn name(&self) -> BehaviorType {
        self.name
    }

    fn execute(
        &mut self,
        _now: f64,
        _last_animation_frame_time: f64,
        mouse_pos: &Pos,
        context: &CanvasRenderingContext2d,
    ) {
        if let Some(mut sprite) = self.sprite {
            unsafe {
                let inpath = sprite.as_ref().point_in_path(mouse_pos, context);
                let artist = sprite.as_mut().get_mut_artist();

                match inpath {
                    true if !self.moving => {
                        self.moving = true;

                        artist.goto(1);
                    }
                    true => (),
                    false => {
                        self.moving = false;

                        artist.goto(0);
                    }
                }
            }
        }
    }
}
