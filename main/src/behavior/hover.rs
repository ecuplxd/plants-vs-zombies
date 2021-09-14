use web_sys::CanvasRenderingContext2d;

use crate::{
    artists::model::Stroke,
    sprites::model::{Pos, Update},
};

use super::model::{Behavior, BehaviorType};

#[derive(Debug)]
pub struct HoverBehavior {
    name: BehaviorType,
    working: bool,
    enter: bool,
    moving: bool,
    points: Vec<Pos>,
}

impl HoverBehavior {
    pub fn new(points: Vec<Pos>) -> HoverBehavior {
        HoverBehavior {
            name: BehaviorType::Hover,
            enter: false,
            moving: false,
            working: false,
            points,
        }
    }
}

impl Behavior for HoverBehavior {
    fn execute(
        &mut self,
        sprite: &mut dyn Update,
        _now: f64,
        _last_animation_frame_time: f64,
        mouse_pos: &Pos,
        context: &CanvasRenderingContext2d,
    ) {
        let Pos { left, top } = mouse_pos;

        self.outline(context, &self.points);

        let inpath = context.is_point_in_path_with_f64(*left, *top);
        let artist = sprite.get_artist();

        self.enter = inpath;

        match inpath {
            true if !self.moving => {
                self.moving = true;
                artist.goto(1);
            }
            true => (),
            false => {
                self.moving = false;
                artist.goto(0);
            }
        }
    }

    fn name(&self) -> BehaviorType {
        self.name
    }

    fn is_working(&mut self) -> bool {
        self.working
    }

    fn start(&mut self, _now: f64) {
        self.working = true;
    }

    fn stop(&mut self, _now: f64) {
        self.working = false;
    }
}

impl Stroke for HoverBehavior {}
