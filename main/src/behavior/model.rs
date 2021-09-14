use serde::Deserialize;
use web_sys::CanvasRenderingContext2d;

use crate::{
    callback::ErasedFnPointer,
    sprites::model::{Pos, Update},
    timer::{animation_timer::AnimationTimer, model::Time},
};

/* *************** trait *************** */

pub trait Behavior {
    fn execute(
        &mut self,
        sprite: &mut dyn Update,
        now: f64,
        last_animation_frame_time: f64,
        mouse_pos: &Pos,
        context: &CanvasRenderingContext2d,
    );

    fn get_timer(&mut self) -> Option<&mut AnimationTimer> {
        None
    }

    fn start(&mut self, now: f64) {
        if let Some(timer) = self.get_timer() {
            timer.start(now)
        }
    }

    fn stop(&mut self, now: f64) {
        if let Some(timer) = self.get_timer() {
            timer.stop(now)
        }
    }

    fn is_working(&mut self) -> bool {
        return match self.get_timer() {
            Some(timer) => timer.is_working(),
            None => true,
        };
    }

    fn set_cb(&mut self, _cb: ErasedFnPointer) {}

    fn name(&self) -> BehaviorType;
}

#[derive(Debug, Clone, Copy, Deserialize, PartialEq)]
pub enum BehaviorType {
    Hover,
    Cycle,
    Walk,
    Switch,
    Frequency,
    Click,
    Scroll,
    Collision,
    Drag,
    Interval,
}

#[derive(Debug, Clone, Copy, Deserialize, PartialEq)]
pub enum Direction {
    Horizontal,
    Vertical,
}

impl Default for Direction {
    fn default() -> Self {
        Direction::Horizontal
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct BehaviorData {
    pub name: BehaviorType,
    #[serde(default)]
    pub duration: f64,
    #[serde(default)]
    pub interval: Option<f64>,
    #[serde(default)]
    pub rate: f64,
    #[serde(default)]
    pub distance: f64,
    #[serde(default = "default_normal_shape")]
    pub normal_shape: bool,
    #[serde(default = "default_infinite")]
    pub infinite: bool,
    #[serde(default)]
    pub switch_cells: Vec<String>,
    #[serde(default)]
    pub direction: Direction,
}

fn default_normal_shape() -> bool {
    true
}

fn default_infinite() -> bool {
    true
}
