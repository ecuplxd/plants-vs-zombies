use web_sys::CanvasRenderingContext2d;

use crate::sprites::model::{CollisionMargin, Pos, SpriteCell};

pub trait Stroke {
    fn rect(
        &self,
        context: &CanvasRenderingContext2d,
        pos: &Pos,
        cell: &SpriteCell,
        scale: f64,
        collision_margin: &CollisionMargin,
    ) {
        let (width, height) = (cell.width * scale, cell.height * scale);
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

    fn outline(&self, context: &CanvasRenderingContext2d, points: &Vec<Pos>) {
        if points.len() == 0 {
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

pub trait Resource {
    fn get_current_cell(&self) -> Option<&SpriteCell> {
        None
    }

    fn in_last_cell(&self) -> bool {
        true
    }

    fn advance(&mut self) {}

    fn goto(&mut self, _index: usize) {}
}

pub trait Draw: Resource {
    fn draw(
        &self,
        context: &CanvasRenderingContext2d,
        pos: &Pos,
        collision_margin: &CollisionMargin,
    );

    fn width_height(&self, context: &CanvasRenderingContext2d) -> (f64, f64) {
        let canvas = context.canvas().unwrap();
        let width = canvas.width() as f64;
        let height = canvas.height() as f64;

        return (width, height);
    }

    fn clear(&self, context: &CanvasRenderingContext2d) {
        let (width, height) = self.width_height(context);

        context.clear_rect(0.0, 0.0, width, height);
    }

    fn update_scale(&mut self, _scale: f64) {}

    fn switch(&mut self, _cells: &Vec<SpriteCell>) {}

    fn revert(&mut self) {}
}
