use crate::{
    fps::Fps,
    loc::Loc,
    model::SpriteType,
    sprites::model::{DrawInfo, Pos, Update},
    timer::animation_timer::AnimationTimer,
};

use super::model::{Behavior, BehaviorType, Direction};

pub struct WalkBehavior {
    name: BehaviorType,
    rate: f64,
    timer: AnimationTimer,
    direction: Direction,
    distance: f64,
    offset: f64,
}

impl WalkBehavior {
    pub fn new(rate: f64, duration: f64, direction: Direction, distance: f64) -> WalkBehavior {
        WalkBehavior {
            name: BehaviorType::Walk,
            rate,
            direction,
            timer: AnimationTimer::new(duration),
            distance,
            offset: 0.0,
        }
    }

    fn update_loc(&self, pos: &Pos, sprite: &mut dyn Update) {
        let cell = sprite.get_artist().get_current_cell().unwrap();
        let zombie_center_pos = Pos::new(pos.left + cell.width / 2.0, pos.top + cell.height / 2.0);
        let (row, col) = Loc::get_row_col_by_pos(&zombie_center_pos);

        sprite.update_loc(row, col);
    }

    fn get_latest_pos(&self, sprite: &mut dyn Update, frame_offset: f64) -> Pos {
        let is_vertical = self.direction == Direction::Vertical;
        let DrawInfo {
            pos: Pos { left, top },
            ..
        } = *sprite.get_draw_info().unwrap();
        let (left, top) = match is_vertical {
            true => (left, top + frame_offset),
            false => {
                let left = match left < 0.0 {
                    true => 900.0,
                    false => left - frame_offset,
                };

                (left, top)
            }
        };

        return Pos::new(left, top);
    }
}

impl Behavior for WalkBehavior {
    fn execute(
        &mut self,
        sprite: &mut dyn Update,
        now: f64,
        last_animation_frame_time: f64,
        _mouse_pos: &Pos,
        _context: &web_sys::CanvasRenderingContext2d,
    ) {
        let frame_offset = Fps::cal_pixel_frame(self.rate, now, last_animation_frame_time);
        let is_vertical = self.direction == Direction::Vertical;

        self.offset += frame_offset;

        // TODO：优化
        if is_vertical && self.offset > self.distance {
            self.stop(now);

            return;
        }

        let new_pos = self.get_latest_pos(sprite, frame_offset);

        // TODO：优化
        if new_pos.left > 900.0 || new_pos.left < 0.0 {
            self.stop(now);
            sprite.toggle();

            return;
        }

        match sprite.name() {
            SpriteType::Zombie(_) => self.update_loc(&new_pos, sprite),
            _ => (),
        }

        sprite.update_draw_info(Some(new_pos), None);
    }

    fn get_timer(&mut self) -> Option<&mut AnimationTimer> {
        Some(&mut self.timer)
    }

    fn name(&self) -> BehaviorType {
        self.name
    }
}
