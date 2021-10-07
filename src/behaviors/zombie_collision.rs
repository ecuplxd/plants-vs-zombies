use derives::{derive_behavior, WithCallback, WithoutTimer};
use web_sys::CanvasRenderingContext2d;

use super::{Behavior, BehaviorType};
use crate::callback::ErasedFnPointer;
use crate::game::Game;
use crate::scenes::LevelScene;
use crate::sprites::{BaseUpdate, ColCheck, Life, Pos, SpritePointer, Update, Zombie};

#[derive_behavior("default")]
#[derive(Default, WithoutTimer, WithCallback)]
pub struct ZombieCollision {
    name: BehaviorType,
    game: Option<NonNull<Game>>,
    collided: bool,
}

impl ZombieCollision {
    pub fn new() -> ZombieCollision {
        ZombieCollision {
            name: BehaviorType::ZombieCollision,
            ..Default::default()
        }
    }

    fn after_zombie_die(&mut self, zombie: &mut Zombie, game: &mut Game) {
        if !zombie.loss_head {
            zombie.loss_head = true;

            LevelScene::build_zombie_head(game, zombie.loss_head_pos);
        }

        if zombie.get_ref_artist().in_last_cell() {
            zombie.hide();
        }
    }
}

impl Behavior for ZombieCollision {
    fn name(&self) -> BehaviorType {
        self.name
    }

    fn execute(
        &mut self,
        now: f64,
        _last_animation_frame_time: f64,
        _mouse_pos: &Pos,
        _context: &CanvasRenderingContext2d,
    ) {
        if let (Some(mut sprite), Some(mut game)) = (self.sprite, self.game) {
            unsafe {
                let zombie = sprite.as_mut().as_any().downcast_mut::<Zombie>().unwrap();

                if zombie.is_die() {
                    self.after_zombie_die(zombie, game.as_mut());

                    return;
                }

                if zombie.get_pos().left < 0.0 {
                    game.as_mut().game_over();

                    return;
                }

                self.collided = false;
                let sprite_ref = sprite.as_ref();

                game.as_mut()
                    .sprites
                    .iter_mut()
                    .filter(|target| {
                        target.can_candidate_for_collision()
                            && sprite_ref
                                .is_candidate_for_collision(target.as_ref(), ColCheck::PrevCol)
                            && sprite_ref.did_collide(target.as_ref())
                    })
                    .for_each(|target| {
                        self.collided = true;

                        zombie.process_collision(target, now);
                    });

                if !self.collided {
                    zombie.change_to_walk(now);
                }
            }
        }
    }

    fn set_game(&mut self, game: *mut Game) {
        self.game = NonNull::new(game);
    }
}
