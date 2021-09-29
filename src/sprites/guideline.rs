use std::any::Any;

use web_sys::CanvasRenderingContext2d;

use super::{BaseUpdate, Update};
use crate::artists::Draw;
use crate::behaviors::Behavior;
use crate::model::{CANVAS_HEIGHT_F64, COL_X_COORD, ROW_Y_COORD};

pub struct GuidelineArtist;

impl GuidelineArtist {
    fn draw_row(context: &CanvasRenderingContext2d) {
        ROW_Y_COORD.iter().for_each(|row| {
            context.begin_path();

            context.move_to(0.5, *row + 0.5);
            context.line_to(1400.5, *row + 0.5);

            context.close_path();
            context.stroke();

            context.close_path();
        });
    }

    fn draw_col(context: &CanvasRenderingContext2d) {
        COL_X_COORD.iter().for_each(|col| {
            context.begin_path();

            context.move_to(*col + 0.5, 0.5);
            context.line_to(*col + 0.5, CANVAS_HEIGHT_F64 + 0.5);

            context.close_path();
            context.stroke();

            context.close_path();
        });
    }
}

impl Draw for GuidelineArtist {
    fn draw(&self, context: &CanvasRenderingContext2d) {
        context.save();

        GuidelineArtist::draw_row(context);
        GuidelineArtist::draw_col(context);

        context.restore();
    }
}

pub struct Guideline {
    artist: GuidelineArtist,
    behaviors: Vec<Box<dyn Behavior>>,
}

impl Guideline {
    pub fn new() -> Guideline {
        Guideline {
            artist: GuidelineArtist,
            behaviors: vec![],
        }
    }
}
impl BaseUpdate for Guideline {
    fn get_ref_artist(&self) -> &dyn Draw {
        &self.artist
    }

    fn get_mut_artist(&mut self) -> &mut dyn Draw {
        &mut self.artist
    }

    fn get_order(&self) -> usize {
        3
    }

    fn get_mut_behaviors(&mut self) -> &mut Vec<Box<dyn Behavior>> {
        &mut self.behaviors
    }
}

impl Draw for Guideline {
    fn draw(&self, context: &CanvasRenderingContext2d) {
        self.artist.draw(context);
    }
}

impl Update for Guideline {
    fn as_any(&mut self) -> &mut dyn Any {
        self
    }
}
