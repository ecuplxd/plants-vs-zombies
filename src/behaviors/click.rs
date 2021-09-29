use std::ptr::NonNull;

use web_sys::CanvasRenderingContext2d;

use super::{Behavior, BehaviorType};
use crate::callback::ErasedFnPointer;
use crate::sprites::{Pos, SpritePointer, Update};
use crate::util::{has_sprite_clicked, set_sprite_clicked};

pub struct ClickBehavior {
    running: bool,
    name: BehaviorType,
    sprite: SpritePointer,
    cb: Option<ErasedFnPointer<SpritePointer>>,
}

impl ClickBehavior {
    pub fn new() -> ClickBehavior {
        ClickBehavior {
            running: false,
            name: BehaviorType::Click,
            sprite: None,
            cb: None,
        }
    }

    fn execute_callback(&self) {
        match self.cb {
            Some(cb) => cb.call(self.sprite),
            _ => (),
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

    fn set_cb(&mut self, cb: ErasedFnPointer<SpritePointer>) {
        self.cb = Some(cb);
    }
}
