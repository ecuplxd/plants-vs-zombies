use derives::{derive_behavior, WithCallback, WithoutTimer};
use web_sys::CanvasRenderingContext2d;

use super::{Behavior, BehaviorType};
use crate::callback::ErasedFnPointer;
use crate::game::Game;
use crate::log;
use crate::model::SpriteType;
use crate::sprites::{ColCheck, PlantSprite, Pos, SpritePointer, Update};

#[derive_behavior("default")]
#[derive(Default, WithoutTimer, WithCallback)]
pub struct PlantCollision {
    name: BehaviorType,
    game: Option<NonNull<Game>>,
}

impl PlantCollision {
    pub fn new() -> PlantCollision {
        PlantCollision {
            name: BehaviorType::PlantCollision,
            ..Default::default()
        }
    }

    pub fn did_collide(sprite: &dyn Update, other_sprite: &dyn Update) -> bool {
        let pos = sprite.get_pos();
        let collision_margin = sprite.get_collision_margin();
        let collision_left = pos.left + collision_margin.left;
        let o_pos = other_sprite.get_pos();
        let o_collision_margin = other_sprite.get_collision_margin();
        let o_collision_left = o_pos.left + o_collision_margin.left;

        // TODO：只适用于子弹和火炬树碰撞，后续如有其它情况则需要优化
        collision_left >= o_collision_left
    }
}

impl Behavior for PlantCollision {
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
        if let (Some(mut sprite), Some(game)) = (self.sprite, self.game) {
            unsafe {
                let bullet = sprite
                    .as_mut()
                    .as_any()
                    .downcast_mut::<PlantSprite>()
                    .unwrap();
                let sprite_ref = sprite.as_ref();
                let torchwood = game.as_ref().sprites.iter().find(|target| {
                    target.can_candidate_for_collision()
                        && SpriteType::is_torchwood(target.name())
                        && sprite_ref.is_candidate_for_collision(target.as_ref(), ColCheck::NextCol)
                        && PlantCollision::did_collide(sprite_ref, target.as_ref())
                });

                if let Some(torchwood) = torchwood {
                    log!("{:?} 和 {:?} 发生碰撞", sprite_ref.id(), torchwood.id());

                    bullet.change_to_fire_bullet();

                    self.stop(now);
                }
            }
        }
    }

    fn set_game(&mut self, game: *mut Game) {
        self.game = NonNull::new(game);
    }
}
