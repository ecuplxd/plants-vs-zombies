use derives::{BaseUpdate, Draw, Life};

use crate::behaviors::Behavior;
use crate::loc::Loc;
use crate::sprites::{Pos, Sprite, Update};

#[derive(Life, BaseUpdate, Draw)]
pub struct PlantSprite {
    pub sprite: Sprite,
    pub life: f64,
    pub hurt: f64,
    pub switch_index: usize,
    pub switch: bool,
}

impl PlantSprite {
    pub fn new(sprite: Sprite) -> PlantSprite {
        PlantSprite {
            sprite,
            life: 100.0,
            hurt: 1.0,
            switch_index: 0,
            switch: false,
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

    fn tirgger_switch(&self) -> (bool, usize) {
        (self.switch, self.switch_index)
    }
}
