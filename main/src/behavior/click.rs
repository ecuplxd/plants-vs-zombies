use web_sys::CanvasRenderingContext2d;

use crate::{
    artists::model::Stroke,
    callback::ErasedFnPointer,
    model::{Interface, SpriteType},
    sprites::model::{DrawInfo, Pos, SpriteCell, Update},
};

use super::model::{Behavior, BehaviorType};

pub struct ClickBehavior {
    name: BehaviorType,
    working: bool,
    clicked: bool,
    points: Vec<Pos>,
    cb: Option<ErasedFnPointer>,
}

impl ClickBehavior {
    pub fn new(points: Vec<Pos>) -> ClickBehavior {
        ClickBehavior {
            name: BehaviorType::Click,
            points,
            clicked: false,
            working: false,
            cb: None,
        }
    }

    pub fn execute_callback(&self) {
        match self.cb {
            Some(cb) if self.clicked => cb.call(),
            _ => (),
        }
    }

    pub fn update_points(&mut self, sprite: &dyn Update) {
        match sprite.name() {
            SpriteType::Interface(Interface::Sun) => {
                let DrawInfo {
                    pos: Pos { left, top },
                    ..
                } = sprite.get_draw_info().unwrap();
                let SpriteCell { width, height, .. } =
                    sprite.get_read_artist().get_current_cell().unwrap();
                let new_points: Vec<Pos> = vec![
                    Pos::new(*left, *top),
                    Pos::new(left + width, *top),
                    Pos::new(left + width, top + height),
                    Pos::new(*left, top + height),
                ];

                self.points = new_points;
            }
            _ => (),
        }
    }
}

impl Behavior for ClickBehavior {
    fn execute(
        &mut self,
        sprite: &mut dyn Update,
        now: f64,
        _last_animation_frame_time: f64,
        mouse_pos: &Pos,
        context: &CanvasRenderingContext2d,
    ) {
        self.update_points(sprite);

        let Pos { left, top } = mouse_pos;

        self.outline(context, &self.points);
        self.clicked = context.is_point_in_path_with_f64(*left, *top);

        sprite.set_clicked(self.clicked);

        self.stop(now);
        self.execute_callback();
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

    fn set_cb(&mut self, cb: ErasedFnPointer) {
        self.cb = Some(cb);
    }
}

impl Stroke for ClickBehavior {}
