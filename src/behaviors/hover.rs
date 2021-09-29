use std::ptr::NonNull;

use web_sys::CanvasRenderingContext2d;

use super::{Behavior, BehaviorType};
use crate::sprites::{Pos, Sprite, SpritePointer, Update};

pub struct HoverBehavior {
    running: bool,
    name: BehaviorType,
    moving: bool,
    sprite: SpritePointer,
}

impl HoverBehavior {
    pub fn new() -> HoverBehavior {
        HoverBehavior {
            running: false,
            name: BehaviorType::Hover,
            moving: false,
            sprite: None,
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
                let sprite = sprite.as_mut().as_any().downcast_mut::<Sprite>().unwrap();

                match inpath {
                    true if !self.moving => {
                        self.moving = true;

                        sprite.artist.goto(1);
                    }
                    true => (),
                    false => {
                        self.moving = false;

                        sprite.artist.goto(0);
                    }
                }
            }
        }
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

    fn set_sprite(&mut self, sprite: *mut dyn Update) {
        self.sprite = NonNull::new(sprite);
    }
}
