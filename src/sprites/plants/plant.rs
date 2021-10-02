use derives::{BaseUpdate, Draw, Life};

use crate::behaviors::Behavior;
use crate::loc::Loc;
use crate::sprites::{Pos, Sprite, Update};

#[derive(Life, BaseUpdate, Draw)]
pub struct PlantSprite {
    pub sprite: Sprite,
    pub life: f64,
    pub hurt: f64,
    pub switch_index: u8,
    pub switch: bool,
}

impl PlantSprite {
    pub fn new(life: f64, hurt: f64, sprite: Sprite) -> PlantSprite {
        PlantSprite {
            sprite,
            life,
            hurt,
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

    // TODO：放到 Attack trait
    pub fn process_attacked(&mut self, attack: &impl Attack, now: f64) -> bool {
        self.being_attacked(attack);

        let die = self.is_die();

        if die {
            self.hide();
            self.stop_all_behavior(now);
        }

        die
    }
}

impl Update for PlantSprite {
    fn update_pos(&mut self, pos: Pos) {
        self.sprite.update_pos(pos);
    }

    fn tirgger_switch(&self) -> (bool, u8) {
        (self.switch, self.switch_index)
    }
}
