use std::any::Any;

use derives::{BaseUpdate, Draw};

use crate::behaviors::Behavior;
use crate::loc::Loc;
use crate::sprites::{BaseUpdate, Pos, Sprite, Update};

#[derive(BaseUpdate, Draw)]
pub struct PlantSprite {
    pub sprite: Sprite,
    pub life: f64,
    pub attack: f64,
    pub switch_index: usize,
}

impl PlantSprite {
    pub fn new(sprite: Sprite) -> PlantSprite {
        PlantSprite {
            sprite,
            life: 100.0,
            attack: 1.0,
            switch_index: 0,
        }
    }

    pub fn get_bullet_pos(&self) -> Pos {
        let pos = self.sprite.pos;

        pos + self.sprite.size.width / 1.5
    }

    pub fn get_sun_pos(&self) -> Pos {
        let pos = self.sprite.pos;
        let delta = Pos::new(73.0, -37.0);

        pos + delta
    }
}

impl Update for PlantSprite {
    fn update_pos(&mut self, pos: Pos) {
        self.sprite.update_pos(pos);
    }

    fn as_any(&mut self) -> &mut dyn Any {
        self
    }

    fn tirgger_switch(&self) -> (bool, usize) {
        (false, self.switch_index)
    }
}
