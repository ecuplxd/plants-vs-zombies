use web_sys::CanvasRenderingContext2d;

use crate::{
    artists::model::{Draw, Resource},
    data::{COL_X_COORD, ROW_Y_COORD},
};

use super::{
    model::{CollisionMargin, DrawInfo, Offset, Pos, Update},
    sprite_wrap::SpriteWrap,
};

pub struct GuidelineArtist {}

impl GuidelineArtist {
    pub fn new() -> GuidelineArtist {
        GuidelineArtist {}
    }

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
            context.line_to(*col + 0.5, 600.5);

            context.close_path();
            context.stroke();

            context.close_path();
        });
    }
}

impl Resource for GuidelineArtist {}

impl Draw for GuidelineArtist {
    fn draw(
        &self,
        context: &CanvasRenderingContext2d,
        _pos: &Pos,
        _collision_margin: &CollisionMargin,
    ) {
        context.save();

        GuidelineArtist::draw_row(context);
        GuidelineArtist::draw_col(context);

        context.restore();
    }
}

pub struct Guideline {
    pub draw_info: DrawInfo,
    artist: Box<dyn Draw>,
    collision_margin: CollisionMargin,
}

impl Guideline {
    pub fn new() -> SpriteWrap {
        let guideline = Guideline {
            draw_info: Default::default(),
            artist: Box::new(GuidelineArtist::new()),
            collision_margin: CollisionMargin::no_collision(),
        };

        return SpriteWrap::new(Box::new(guideline), vec![]);
    }
}

impl Update for Guideline {
    fn draw(&self, context: &CanvasRenderingContext2d) {
        self.artist
            .draw(context, &self.draw_info.pos, &self.collision_margin);
    }

    fn get_artist(&mut self) -> &mut dyn Draw {
        self.artist.as_mut()
    }

    fn get_draw_info(&self) -> Option<&DrawInfo> {
        Some(&self.draw_info)
    }

    fn update_draw_info(&mut self, pos: Option<Pos>, offset: Option<Offset>) {
        let pos = match pos {
            Some(pos) => pos,
            None => self.draw_info.pos,
        };
        let offset = match offset {
            Some(offset) => offset,
            None => self.draw_info.offset,
        };

        self.draw_info = DrawInfo::new(pos, offset, true, 1);
    }

    fn get_read_artist(&self) -> &dyn Draw {
        self.artist.as_ref()
    }
}
