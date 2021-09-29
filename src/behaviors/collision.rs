use std::ptr::NonNull;

use web_sys::CanvasRenderingContext2d;

use super::{Behavior, BehaviorType};
use crate::callback::ErasedFnPointer;
use crate::game::Game;
use crate::log;
use crate::model::SpriteType;
use crate::sprites::{Pos, SpritePointer, Update, ZombieSprite};

pub struct CollisionBehavior {
    name: BehaviorType,
    running: bool,
    sprite: SpritePointer,
    cb: Option<ErasedFnPointer<SpritePointer>>,
    game: Option<NonNull<Game>>,
    collided: bool,
}

impl CollisionBehavior {
    pub fn new() -> CollisionBehavior {
        CollisionBehavior {
            name: BehaviorType::Collision,
            running: false,
            sprite: None,
            cb: None,
            game: None,
            collided: false,
        }
    }

    fn execute_callback(&self) {
        match self.cb {
            Some(cb) => cb.call(self.sprite),
            _ => (),
        }
    }
}

impl Behavior for CollisionBehavior {
    fn name(&self) -> BehaviorType {
        self.name
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

    fn execute(
        &mut self,
        now: f64,
        _last_animation_frame_time: f64,
        _mouse_pos: &Pos,
        _context: &CanvasRenderingContext2d,
    ) {
        match (self.sprite, self.game) {
            (Some(mut sprite), Some(mut game)) => unsafe {
                self.collided = false;

                let sprite_ref = sprite.as_ref();
                let zombie = sprite
                    .as_mut()
                    .as_any()
                    .downcast_mut::<ZombieSprite>()
                    .unwrap();

                game.as_mut()
                    .sprites
                    .iter_mut()
                    .filter(|target| {
                        target.can_candidate_for_collision()
                            && sprite_ref.is_candidate_for_collision(target.as_ref())
                            && sprite_ref.did_collide(target.as_ref())
                    })
                    .for_each(|target| {
                        log!("{:?} 和 {:?} 发生碰撞", sprite_ref.id(), target.id());

                        self.collided = true;

                        let is_bullet = SpriteType::is_bullet(target.name());
                        let is_lawn_cleaner = SpriteType::is_lawn_cleaner(target.name());

                        if is_bullet {
                            zombie.process_bullet_collision(target, now);
                        } else if is_lawn_cleaner {
                            zombie.process_lawn_cleaner_collision(target, now);
                        } else if !zombie.attacking {
                            zombie.change_to_attack(now);
                        }
                    });

                if !self.collided {
                    zombie.change_to_walk(now);
                }
            },
            _ => (),
        }
    }

    fn set_sprite(&mut self, sprite: *mut dyn Update) {
        self.sprite = NonNull::new(sprite);
    }

    fn set_game(&mut self, game: *mut Game) {
        self.game = NonNull::new(game);
    }

    fn set_cb(&mut self, cb: ErasedFnPointer<SpritePointer>) {
        self.cb = Some(cb);
    }
}
