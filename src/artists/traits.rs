use web_sys::CanvasRenderingContext2d;

use crate::sprites::{CollisionMargin, Pos, Size, SpriteCell};

pub trait Draw {
    fn draw(&self, _context: &CanvasRenderingContext2d) {}

    fn draw_image(&self, _context: &CanvasRenderingContext2d, _pos: &Pos) {}

    fn draw_text(&self, _context: &CanvasRenderingContext2d, _text: &str, _size: f64, _pos: &Pos) {}

    fn revert(&mut self) {}

    fn switch(&mut self, _cells: &[SpriteCell]) {}

    fn advance(&mut self) {}

    fn in_last_cell(&self) -> bool {
        true
    }

    fn get_current_cell(&self) -> Option<&SpriteCell> {
        None
    }

    fn update_scale(&mut self, _scale: f64) {}

    fn goto(&mut self, _index: usize) {}
}

pub trait Stroke {
    fn rect(
        &self,
        context: &CanvasRenderingContext2d,
        pos: &Pos,
        size: &Size,
        scale: f64,
        collision_margin: &CollisionMargin,
    ) {
        let (width, height) = (size.width * scale, size.height * scale);
        let CollisionMargin {
            left,
            top,
            right,
            bottom,
        } = collision_margin;

        context.save();

        context.set_stroke_style(&"#00CCFF".into());
        context.stroke_rect(
            pos.left + left,
            pos.top + top,
            width - right - left,
            height - bottom - top,
        );

        context.set_stroke_style(&"#FF0000".into());
        context.stroke_rect(pos.left, pos.top, width, height);

        context.restore();
    }

    fn outline(&self, context: &CanvasRenderingContext2d, points: &[Pos]) {
        if points.is_empty() {
            return;
        }

        context.save();

        context.begin_path();
        context.set_stroke_style(&"#00CCFF".into());

        context.move_to(points[0].left, points[1].top);

        points
            .iter()
            .skip(1)
            .for_each(|path| context.line_to(path.left, path.top));

        context.close_path();
        context.stroke();

        context.restore();
    }
}
