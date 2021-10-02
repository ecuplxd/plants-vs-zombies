use derives::{BaseUpdate, Draw, Life};

use crate::behaviors::{Behavior, BehaviorType, IntervalBehavior};
use crate::callback::ErasedFnPointer;
use crate::loc::Loc;
use crate::sprites::{PlantCallback, PlantSprite, Pos, Sprite, SpritePointer, Update, ZombieState};
use crate::timer::Timer;
use crate::util::get_random_int_inclusive;

#[derive(Life, BaseUpdate, Draw)]
pub struct ZombieSprite {
    pub sprite: Sprite,
    pub life: f64,
    pub hurt: f64,
    pub switch_index: u8,
    state: u8,
    stage: u8,
    pub loss_head: bool,
}

impl ZombieSprite {
    pub fn new(life: f64, hurt: f64, sprite: Sprite) -> ZombieSprite {
        ZombieSprite {
            sprite,
            life,
            hurt,
            switch_index: ZombieState::Wait as u8,
            state: 1 << ZombieState::Wait as u8,
            stage: (life / 100.0) as u8,
            loss_head: false,
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

    pub fn change_state(&mut self, state: ZombieState, switch: bool) {
        self.state = 1 << state.clone() as u8;

        if switch {
            self.switch_index = state as u8 - 1;
        }
    }

    pub fn in_state(&self, state: ZombieState) -> bool {
        self.state == (1 << state as u8)
    }

    pub fn change_to_walk(&mut self, now: f64) {
        if self.in_walking() {
            return;
        }

        match self.life < 100.0 {
            true => self.change_state(ZombieState::LostArmorWalk, true),
            false => self.change_state(ZombieState::Walk, true),
        }

        self.toggle_behavior(BehaviorType::Walk, true, now);
    }

    pub fn change_to_attack(&mut self, now: f64) {
        match self.life < 100.0 {
            true => self.change_state(ZombieState::LostArmorAttack, true),
            false => self.change_state(ZombieState::Attack, true),
        }

        self.toggle_behavior(BehaviorType::Walk, false, now);
    }

    pub fn change_to_die(&mut self, now: f64) {
        self.change_state(ZombieState::Die, true);
        self.toggle_behavior(BehaviorType::Walk, false, now);
        self.toggle_behavior(BehaviorType::Collision, false, now);
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

        if SpriteType::is_screen_door(self.name()) && self.in_state(ZombieState::Attack) {
            new_pos.top += 18.0;
        }

        self.sprite.update_pos(new_pos);
    }

    pub fn hide_bullet(&mut self, plant: SpritePointer) {
        self.sprite.global_alpha = 1.0;

        if let Some(mut bullet) = plant {
            unsafe {
                let now = Timer::get_current_time();
                let bullet = bullet
                    .as_mut()
                    .as_any()
                    .downcast_mut::<PlantSprite>()
                    .unwrap();

                bullet.hide();
                bullet.stop_all_behavior(now);

                self.process_attacked(bullet, now);
                self.toggle_behavior(BehaviorType::Collision, true, now);
            }
        }
    }

    fn attack_plant(&mut self, plant: SpritePointer) {
        if let Some(mut plant) = plant {
            unsafe {
                let now = Timer::get_current_time();
                let plant = plant
                    .as_mut()
                    .as_any()
                    .downcast_mut::<PlantSprite>()
                    .unwrap();
                let died = plant.process_attacked(self, now);

                if died {
                    self.change_to_walk(now);
                    self.toggle_behavior(BehaviorType::Collision, true, now);
                }
            }
        }
    }

    fn in_walking(&mut self) -> bool {
        self.in_state(ZombieState::Walk) || self.in_state(ZombieState::LostArmorWalk)
    }

    pub fn in_attacking(&mut self) -> bool {
        self.in_state(ZombieState::Attack) || self.in_state(ZombieState::LostArmorAttack)
    }

    fn process_life_stage(&mut self, now: f64) {
        if self.life < 100.0 && self.stage == 2 {
            if self.in_walking() {
                self.change_state(ZombieState::LostArmorWalk, true);
            } else if self.in_state(ZombieState::Attack) {
                self.change_to_attack(now);
            }

            self.stage = 1;
        }
    }

    fn process_attacked(&mut self, attack: &impl Attack, now: f64) {
        self.being_attacked(attack);
        self.process_life_stage(now);

        if self.is_die() {
            self.change_to_die(now);
        }
    }

    fn get_callback(&mut self, callback: PlantCallback) -> ErasedFnPointer<SpritePointer> {
        match callback {
            PlantCallback::Switch => {
                ErasedFnPointer::from_associated(self, ZombieSprite::hide_bullet)
            }
            PlantCallback::Interval => {
                ErasedFnPointer::from_associated(self, ZombieSprite::attack_plant)
            }
        }
    }

    pub fn register_callback(&mut self, behavior: &mut Box<dyn Behavior>, callback: PlantCallback) {
        let pointer = self.get_callback(callback);

        behavior.add_callback(pointer);
    }

    fn before_process_collision(&mut self, now: f64) {
        self.toggle_behavior(BehaviorType::Collision, false, now);
    }

    pub fn process_bullet_collision(&mut self, bullet: &mut Box<dyn Update>, now: f64) {
        self.sprite.global_alpha = 0.5;
        self.before_process_collision(now);
        bullet.toggle_behavior(BehaviorType::Walk, false, now);

        let switch = bullet.find_behavior(BehaviorType::Switch).unwrap();

        self.register_callback(switch, PlantCallback::Switch);

        let bullet = bullet.as_any().downcast_mut::<PlantSprite>().unwrap();

        bullet.switch = true;
    }

    pub fn process_lawn_cleaner_collision(&mut self, lawn_cleaner: &mut Box<dyn Update>, now: f64) {
        let lawn_cleaner = lawn_cleaner
            .as_mut()
            .as_any()
            .downcast_mut::<PlantSprite>()
            .unwrap();

        self.before_process_collision(now);
        self.process_attacked(lawn_cleaner, now);

        lawn_cleaner.toggle_behavior(BehaviorType::Walk, true, now);
    }

    pub fn process_plant_collision(&mut self, plant: &mut Box<dyn Update>, now: f64) {
        let mut interval: Box<dyn Behavior> = Box::new(IntervalBehavior::new(2000.0));

        self.register_callback(&mut interval, PlantCallback::Interval);

        interval.set_sprite(plant.as_mut());
        interval.start(now);
        plant.add_behavior(interval);

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

    fn tirgger_switch(&self) -> (bool, u8) {
        (self.state != 0, self.switch_index)
    }
}
