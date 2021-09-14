use web_sys::CanvasRenderingContext2d;

use crate::{
    behavior::model::{Behavior, BehaviorType},
    callback::ErasedFnPointer,
    model::{Interface, SpriteType},
};

use super::model::{DrawInfo, Offset, Pos, Update};

pub struct SpriteWrap {
    pub sprite: Box<dyn Update>,
    pub behaviors: Vec<Box<dyn Behavior>>,
}

impl SpriteWrap {
    pub fn new(sprite: Box<dyn Update>, behaviors: Vec<Box<dyn Behavior>>) -> SpriteWrap {
        SpriteWrap { behaviors, sprite }
    }

    pub fn find_sprite_behaviors(
        sprites: &mut Vec<SpriteWrap>,
        sprite_type: SpriteType,
        behavior_type: BehaviorType,
    ) -> Vec<&mut Box<dyn Behavior>> {
        let mut behaviors: Vec<&mut Box<dyn Behavior>> = vec![];

        for sprite in sprites {
            if sprite.name() == sprite_type {
                for behavior in &mut sprite.behaviors {
                    if behavior.name() == behavior_type {
                        behaviors.push(behavior);
                    }
                }
            }
        }

        return behaviors;
    }

    pub fn find_sprite_behavior(
        sprites: &mut Vec<SpriteWrap>,
        sprite_type: SpriteType,
        behavior_type: BehaviorType,
    ) -> Option<&mut Box<dyn Behavior>> {
        let sprite = sprites
            .iter_mut()
            .find(|sprite| sprite.name() == sprite_type);

        return match sprite {
            Some(sprite) => sprite
                .behaviors
                .iter_mut()
                .find(|behavior| behavior.name() == behavior_type),
            None => None,
        };
    }

    pub fn has_behavior(&self, behavior_type: BehaviorType) -> bool {
        return self
            .behaviors
            .iter()
            .find(|behavior| behavior.name() == behavior_type)
            .is_some();
    }

    pub fn before_update(&mut self, now: f64) {
        match self.sprite.name() {
            SpriteType::Zombie(_) => {
                self.toggle_behaviors(&vec![BehaviorType::Walk], !self.is_collision(), now);
            }
            _ => (),
        }
    }

    pub fn update(
        &mut self,
        now: f64,
        last_animation_frame_time: f64,
        pos: &Pos,
        context: &CanvasRenderingContext2d,
    ) {
        self.before_update(now);

        for behavior in &mut self.behaviors {
            if behavior.is_working() {
                behavior.execute(
                    self.sprite.as_mut(),
                    now,
                    last_animation_frame_time,
                    pos,
                    context,
                );
            }
        }
    }

    pub fn toggle_behaviors(&mut self, behavior_types: &Vec<BehaviorType>, run: bool, now: f64) {
        for behavior in &mut self.behaviors {
            match behavior_types.contains(&behavior.name()) {
                true => match run {
                    true => behavior.start(now),
                    false => behavior.stop(now),
                },
                _ => (),
            }
        }
    }

    pub fn draw(&self, context: &CanvasRenderingContext2d) {
        if let Some(draw_info) = self.get_draw_info() {
            let DrawInfo { offset, .. } = draw_info;

            context.translate(-offset.x, -offset.y).unwrap();

            self.sprite.draw(context);

            context.translate(offset.x, offset.y).unwrap();
        } else {
            self.sprite.draw(context);
        }
    }

    pub fn update_draw_info(&mut self, pos: Option<Pos>, offset: Option<Offset>) {
        self.sprite.update_draw_info(pos, offset);
    }

    pub fn get_draw_info(&self) -> Option<&DrawInfo> {
        return self.sprite.get_draw_info();
    }

    pub fn is_visible(&self) -> bool {
        return self.sprite.is_visible();
    }

    pub fn name(&self) -> SpriteType {
        return self.sprite.name();
    }

    pub fn register_callback(&mut self, behavior_type: BehaviorType, pointer: ErasedFnPointer) {
        let behavior = self
            .behaviors
            .iter_mut()
            .find(|behavior| behavior.name() == behavior_type);

        if let Some(behavior) = behavior {
            behavior.set_cb(pointer);
        }
    }

    pub fn update_scale(&mut self, scale: f64) {
        self.sprite.get_artist().update_scale(scale);
    }

    pub fn get_order(&self) -> usize {
        return self.sprite.get_order();
    }

    pub fn get_loc(&self) -> (usize, usize) {
        return self.sprite.get_loc();
    }

    pub fn is_collision(&self) -> bool {
        self.sprite.is_collision()
    }

    pub fn is_clicked(&self) -> bool {
        self.sprite.is_clicked()
    }

    pub fn is_plant(&self) -> bool {
        match self.sprite.name() {
            SpriteType::Interface(Interface::LawnCleaner) => true,
            SpriteType::Plant(_) => true,
            _ => false,
        }
    }

    pub fn can_check_collision(&self) -> bool {
        self.is_visible() && self.has_behavior(BehaviorType::Collision)
    }

    pub fn can_candidate_for_collision(&self) -> bool {
        self.is_visible() && self.is_plant()
    }
}
