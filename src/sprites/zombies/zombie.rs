use std::any::Any;

use derives::{BaseUpdate, Draw};

use crate::behaviors::{Behavior, BehaviorType};
use crate::callback::ErasedFnPointer;
use crate::loc::Loc;
use crate::model::Zombie;
use crate::sprites::{BaseUpdate, PlantCallback, PlantSprite, Pos, Sprite, SpritePointer, Update};
use crate::timer::Timer;
use crate::util::get_random_int_inclusive;

#[derive(BaseUpdate, Draw)]
pub struct ZombieSprite {
    pub sprite: Sprite,
    pub waiting: bool,
    pub walking: bool,
    pub attacking: bool,
    pub dieing: bool,
    pub died: bool,
    pub life: f64,
    pub attack: f64,
    pub switch_index: usize,
}

impl ZombieSprite {
    pub fn new(sprite: Sprite) -> ZombieSprite {
        ZombieSprite {
            sprite,
            waiting: true,
            walking: false,
            attacking: false,
            dieing: false,
            died: false,
            life: 100.0,
            attack: 1.0,
            switch_index: 0,
        }
    }

    pub fn get_random_pos(index: usize, size: &Size) -> (Loc, Pos) {
        let loc = Loc::new(index % 5, 9);
        let mut pos = Loc::put_on_cell_bottom(&loc, size);
        let random_offset = get_random_int_inclusive(0.0, 40.0);

        pos.left -= random_offset;

        (loc, pos)
    }

    pub fn update_offset(&mut self, offset: Pos) {
        self.sprite.update_offset(offset);
    }

    pub fn init_pos(&mut self, index: usize) {
        let (loc, pos) = ZombieSprite::get_random_pos(index, &self.sprite.size);

        self.sprite.update_loc(loc);
        self.sprite.update_pos(pos);
    }

    // TODO：优化 使用位运算
    pub fn change_to_walk(&mut self, now: f64) {
        self.waiting = false;
        self.walking = true;
        self.attacking = false;
        self.dieing = false;
        self.died = false;
        self.switch_index = 0;

        self.toggle_behavior(BehaviorType::Walk, true, now);
    }

    pub fn change_to_attack(&mut self, now: f64) {
        self.waiting = false;
        self.walking = false;
        self.attacking = true;
        self.dieing = false;
        self.died = false;
        self.switch_index = 1;

        self.toggle_behavior(BehaviorType::Walk, false, now);
    }

    pub fn change_to_dieing(&mut self, now: f64) {
        self.waiting = false;
        self.walking = false;
        self.attacking = false;
        self.dieing = true;
        self.died = false;

        self.toggle_behavior(BehaviorType::Walk, false, now);
    }

    pub fn change_to_died(&mut self, now: f64) {
        self.waiting = false;
        self.walking = false;
        self.attacking = false;
        self.dieing = false;
        self.died = true;

        self.toggle_behavior(BehaviorType::Walk, false, now);
    }

    fn align_plant_pos(&mut self, pos: Option<Pos>, size: Option<Size>) {
        let size = match size {
            Some(size) => size,
            None => self.sprite.size,
        };
        let pos = match pos {
            Some(pos) => pos,
            None => self.sprite.pos,
        };
        let zombie_center_pos = pos + Pos::new(size.width / 2.0, size.height / 2.0);
        let loc = Loc::get_row_col_by_pos(&zombie_center_pos);
        let offset_pos = Loc::put_on_cell_bottom(&loc, &size);
        // switch 后导致前后 cell 宽高可能不一致
        let mut new_pos = Pos::new(pos.left, offset_pos.top);

        self.sprite.size = size;
        self.sprite.update_loc(loc);

        if self.name() == SpriteType::Zombie(Zombie::ScreenDoor) && self.attacking {
            new_pos.top += 18.0;
        }

        self.sprite.update_pos(new_pos);
    }

    pub fn toggle_bullet(&mut self, plant: SpritePointer) {
        if let Some(mut bullet) = plant {
            unsafe {
                bullet.as_mut().toggle();

                self.sprite.global_alpha = 1.0;
                self.toggle_behavior(BehaviorType::Collision, true, Timer::get_current_time());
            }
        }
    }

    fn get_callback(&mut self, callback: PlantCallback) -> ErasedFnPointer<SpritePointer> {
        match callback {
            PlantCallback::Switch => {
                ErasedFnPointer::from_associated(self, ZombieSprite::toggle_bullet)
            }
        }
    }

    pub fn register_callback(&mut self, behavior: &mut Box<dyn Behavior>, callback: PlantCallback) {
        let pointer = self.get_callback(callback);

        behavior.set_cb(pointer);
    }

    fn before_process_collision(&mut self, now: f64) {
        self.toggle_behavior(BehaviorType::Collision, false, now);
    }

    pub fn process_bullet_collision(&mut self, bullet: &mut Box<dyn Update>, now: f64) {
        self.before_process_collision(now);

        let switch = bullet.find_behavior(BehaviorType::Switch).unwrap();

        self.sprite.global_alpha = 0.5;
        self.register_callback(switch, PlantCallback::Switch);

        let bullet = bullet.as_any().downcast_mut::<PlantSprite>().unwrap();

        bullet.switch = true;
    }

    pub fn process_lawn_cleaner_collision(&mut self, lawn_cleaner: &mut Box<dyn Update>, now: f64) {
        self.before_process_collision(now);

        self.change_to_dieing(now);

        lawn_cleaner.toggle_behavior(BehaviorType::Walk, true, now);
    }

    pub fn process_plant_collision(&mut self, _plant: &mut Box<dyn Update>, now: f64) {
        self.change_to_attack(now);
    }
}

// TODO：优化
impl Update for ZombieSprite {
    fn update_pos(&mut self, pos: Pos) {
        let cell = self.sprite.artist.get_current_cell().unwrap();
        let size = cell.into();

        self.align_plant_pos(Some(pos), Some(size));
    }

    fn as_any(&mut self) -> &mut dyn Any {
        self
    }

    fn tirgger_switch(&self) -> (bool, usize) {
        let flag = self.walking || self.attacking || self.dieing || self.died;

        (flag, self.switch_index)
    }
}
