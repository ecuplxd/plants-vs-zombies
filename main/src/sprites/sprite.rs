use web_sys::CanvasRenderingContext2d;

use crate::{artists::model::Draw, model::SpriteType};

use super::model::{CollisionMargin, DrawInfo, Offset, Pos, Update, Velocit};

pub struct Sprite {
    pub name: SpriteType,

    pub width: f64,
    pub height: f64,

    pub velocit: Velocit,

    pub draw_info: DrawInfo,

    pub collision_margin: CollisionMargin,

    pub artist: Box<dyn Draw>,

    pub row: usize,

    pub col: usize,

    pub clicked: bool,
}

impl Sprite {
    pub fn new(
        name: SpriteType,
        artist: Box<dyn Draw>,
        draw_info: DrawInfo,
        collision_margin: CollisionMargin,
    ) -> Sprite {
        let cell = artist.as_ref().get_current_cell();
        let (width, height) = match cell {
            Some(cell) => (cell.width, cell.height),
            None => (0.0, 0.0),
        };

        Sprite {
            name,
            width,
            height,

            velocit: Default::default(),

            collision_margin,

            draw_info,

            artist,

            row: 0,
            col: 0,

            clicked: false,
        }
    }
}

impl Update for Sprite {
    fn draw(&self, context: &CanvasRenderingContext2d) {
        let DrawInfo { pos, .. } = &self.draw_info;

        self.artist.draw(context, pos, &self.collision_margin);
    }

    fn get_artist(&mut self) -> &mut dyn Draw {
        return self.artist.as_mut();
    }

    fn get_draw_info(&self) -> Option<&DrawInfo> {
        return Some(&self.draw_info);
    }

    fn update_draw_info(&mut self, pos: Option<Pos>, offset: Option<Offset>) {
        let DrawInfo { visible, order, .. } = self.draw_info;
        let pos = match pos {
            Some(pos) => pos,
            None => self.draw_info.pos,
        };
        let offset = match offset {
            Some(offset) => offset,
            None => self.draw_info.offset,
        };

        self.draw_info = DrawInfo::new(pos, offset, visible, order);
    }

    fn name(&self) -> SpriteType {
        return self.name;
    }

    fn is_visible(&self) -> bool {
        return self.draw_info.visible;
    }

    fn toggle(&mut self) {
        self.draw_info.visible = !self.draw_info.visible;
    }

    fn get_order(&self) -> usize {
        self.draw_info.order
    }

    fn update_loc(&mut self, row: usize, col: usize) {
        self.row = row;
        self.col = col;
    }

    fn get_loc(&self) -> (usize, usize) {
        (self.row, self.col)
    }

    fn get_read_artist(&self) -> &dyn Draw {
        self.artist.as_ref()
    }

    fn is_clicked(&self) -> bool {
        self.clicked
    }

    fn set_clicked(&mut self, clicked: bool) {
        self.clicked = clicked;
    }
}
