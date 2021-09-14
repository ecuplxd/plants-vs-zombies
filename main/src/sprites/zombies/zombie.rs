use std::{cell::Cell, rc::Rc};

use web_sys::CanvasRenderingContext2d;

use crate::{
    artists::model::Draw,
    behavior::collision::CollisionBehavior,
    model::SpriteType,
    sprites::{
        model::{DrawInfo, Offset, Pos, Update},
        sprite::Sprite,
    },
};

use super::model::ZombieState;

pub struct ZombieSprite {
    life: f64,
    attack: f64,
    sprite: Sprite,
    state: Rc<Cell<ZombieState>>,
    collisioned: Rc<Cell<bool>>,
}

impl ZombieSprite {
    pub fn new(sprite: Sprite) -> ZombieSprite {
        ZombieSprite {
            life: 100.0,
            attack: 1.0,
            sprite,
            collisioned: Rc::new(Cell::new(false)),
            state: Rc::new(Cell::new(ZombieState::new())),
        }
    }

    pub fn swtich_callback(&mut self) {}

    pub fn collision_callback(&mut self) {
        self.state.set(ZombieState {
            switch_index: 1,
            waiting: false,
            walking: false,
            attacking: true,
            dieing: false,
            died: false,
        });
    }
}

impl Update for ZombieSprite {
    fn draw(&self, context: &CanvasRenderingContext2d) {
        self.sprite.draw(context);
    }

    fn get_artist(&mut self) -> &mut dyn Draw {
        self.sprite.get_artist()
    }

    fn get_draw_info(&self) -> Option<&DrawInfo> {
        Some(&self.sprite.draw_info)
    }

    fn name(&self) -> SpriteType {
        self.sprite.name
    }

    fn update_draw_info(&mut self, pos: Option<Pos>, offset: Option<Offset>) {
        self.sprite.update_draw_info(pos, offset);
    }

    fn is_visible(&self) -> bool {
        self.sprite.draw_info.visible
    }

    fn toggle(&mut self) {
        self.sprite.draw_info.visible = !self.sprite.draw_info.visible;
    }

    fn get_order(&self) -> usize {
        self.sprite.draw_info.order
    }

    fn tirgger_switch(&mut self) -> (bool, usize) {
        let ZombieState {
            walking,
            attacking,
            dieing,
            died,
            switch_index,
            ..
        } = self.state.get();

        return (walking || attacking || dieing || died, switch_index);
    }

    fn check_collision(&self, sprites: &Vec<&Box<dyn Update>>) -> bool {
        for sprite in sprites {
            if CollisionBehavior::is_candidate_for_collision(&self.sprite, sprite)
                && CollisionBehavior::did_collide(&self.sprite, sprite)
            {
                self.collisioned.set(true);

                return true;
            }
        }

        self.collisioned.set(false);
        self.state.set(ZombieState {
            switch_index: 0,
            waiting: false,
            walking: true,
            attacking: false,
            dieing: false,
            died: false,
        });

        return false;
    }

    fn is_collision(&self) -> bool {
        self.collisioned.get()
    }

    fn update_loc(&mut self, row: usize, col: usize) {
        self.sprite.update_loc(row, col);
    }

    fn get_loc(&self) -> (usize, usize) {
        self.sprite.get_loc()
    }

    fn get_read_artist(&self) -> &dyn Draw {
        self.sprite.get_read_artist()
    }

    fn is_clicked(&self) -> bool {
        self.sprite.clicked
    }

    fn set_clicked(&mut self, clicked: bool) {
        self.sprite.set_clicked(clicked);
    }
}
