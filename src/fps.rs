use crate::log;
use crate::sprites::{Pos, Velocit};

#[derive(Debug, Default)]
pub struct Fps {
    current_time: f64,
    value: f64,
    display: u16,
    pub last_animation_frame_time: f64,
    pub last_fps_update_time: f64,
}

impl Fps {
    pub fn new() -> Fps {
        Fps {
            value: 60.0,
            display: 60,
            ..Default::default()
        }
    }

    pub fn format(value: f64) -> u16 {
        return value as u16;
    }

    pub fn cal_pixel_frame(rate: f64, now: f64, last_animation_frame_time: f64) -> f64 {
        return rate * ((now - last_animation_frame_time) / 1000.0);
    }

    pub fn cal_velocit_offset(velocit: &Velocit, now: f64, last_animation_frame_time: f64) -> Pos {
        let x_offset = Fps::cal_pixel_frame(velocit.x, now, last_animation_frame_time);
        let y_ofset = Fps::cal_pixel_frame(velocit.y, now, last_animation_frame_time);

        Pos::new(x_offset, y_ofset)
    }

    pub fn calc(&mut self, now: f64, rate: f64) {
        self.current_time = now;
        self.value = (1.0 / (now - self.last_animation_frame_time)) * 1000.0 * rate;

        if now - self.last_fps_update_time > 1000.0 {
            self.last_fps_update_time = now;
            self.display = Fps::format(self.value);

            unsafe { log!("{} fps", &self.display) }
        }
    }

    pub fn _one_frame_passed(&self, last_time: f64, animation_rate: f64) -> bool {
        return self.current_time - last_time > animation_rate;
    }

    pub fn _increase_update(&mut self, delta: f64) {
        self.last_animation_frame_time += delta;
    }

    pub fn update(&mut self, now: f64) {
        self.last_animation_frame_time = now;
    }
}
