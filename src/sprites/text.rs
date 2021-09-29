use std::any::Any;

use web_sys::{CanvasRenderingContext2d, TextMetrics};

use super::{BaseUpdate, Pos, Size, SpriteCell, Update};
use crate::artists::Draw;
use crate::behaviors::Behavior;
use crate::game::Game;
use crate::loc::Loc;
use crate::model::SpriteType;

pub struct TextArtist;

impl TextArtist {
    fn get_pos(text: &str, size: f64, rect: &SpriteCell) -> Pos {
        let offscreen_canvas = Game::create_canvas(200, 200);
        let offscreen_context = Game::get_canvas_context(&offscreen_canvas);

        TextArtist::set_text_style(&offscreen_context, size);

        let text_metrics: TextMetrics = offscreen_context.measure_text(text).unwrap();
        let text_size: Size = text_metrics.into();

        Loc::put_center(rect, &text_size)
    }

    fn set_text_style(context: &CanvasRenderingContext2d, size: f64) {
        let font_size = format!("{}px 黑体", size);

        context.set_font(&font_size);
        context.set_text_baseline("top");
    }
}

impl Draw for TextArtist {
    fn draw_text(&self, context: &CanvasRenderingContext2d, text: &str, size: f64, pos: &Pos) {
        context.save();

        TextArtist::set_text_style(context, size);

        context.fill_text(text, pos.left, pos.top).unwrap();

        context.restore();
    }
}

pub struct TextSprite {
    name: SpriteType,
    text: String,
    size: f64,
    pub pos: Pos,
    artist: TextArtist,
    behaviors: Vec<Box<dyn Behavior>>,
}

impl TextSprite {
    pub fn new(name: SpriteType, text: &str, size: f64, cell: &SpriteCell) -> TextSprite {
        TextSprite {
            name,
            pos: TextArtist::get_pos(text, size, cell),
            text: String::from(text),
            size,
            artist: TextArtist,
            behaviors: vec![],
        }
    }
}

impl Draw for TextSprite {
    fn draw(&self, context: &CanvasRenderingContext2d) {
        let TextSprite {
            text, size, pos, ..
        } = self;

        self.artist.draw_text(context, text, *size, pos);
    }
}

impl BaseUpdate for TextSprite {
    fn name(&self) -> SpriteType {
        self.name
    }

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

impl Update for TextSprite {
    fn as_any(&mut self) -> &mut dyn Any {
        self
    }
}
