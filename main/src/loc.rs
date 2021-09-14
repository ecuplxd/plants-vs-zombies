use crate::{
    data::{COL_X_COORD, ROW_Y_COORD},
    sprites::model::Pos,
};

pub struct Loc {}

impl Loc {
    pub fn _new() -> Loc {
        Loc {}
    }

    pub fn put_on_cell_bottom(row: usize, col: usize, width: f64, height: f64) -> Pos {
        let mut pos = Loc::put_on(row, col);
        let pos2 = Loc::put_on(row + 1, col + 1);

        pos.left += (pos2.left - pos.left - width) / 2.0;
        pos.top = pos2.top - 5.0 - height;

        return pos;
    }

    pub fn put_on(row: usize, col: usize) -> Pos {
        let x = COL_X_COORD[col];
        let y = ROW_Y_COORD[row];

        return Pos::new(x, y);
    }

    pub fn _put_any(left: f64, top: f64) -> Pos {
        Pos::new(left, top)
    }

    pub fn put_increase_x(
        y: f64,
        begin: f64,
        step: f64,
        count: usize,
        max: usize,
        item_height: f64,
    ) -> Vec<Pos> {
        let mut pos: Vec<Pos> = vec![];
        let max = max + 1;

        for i in 0..count {
            let (row, col) = (i / max, i % max);

            pos.push(Pos::new(
                begin + step * col as f64,
                y + item_height * row as f64,
            ));
        }

        return pos;
    }

    pub fn put_increase_y(x: f64, begin: f64, step: f64, count: usize) -> Vec<Pos> {
        let mut pos: Vec<Pos> = vec![];

        for i in 0..count {
            pos.push(Pos::new(x, begin + i as f64 * step));
        }

        return pos;
    }

    pub fn get_col_by_x(x: f64) -> usize {
        let col = COL_X_COORD.iter().position(|col| *col > x);

        match col {
            Some(col) if col > 0 => col - 1,
            _ => 99,
        }
    }

    pub fn get_row_by_y(y: f64) -> usize {
        let row = ROW_Y_COORD.iter().position(|row| *row > y);

        match row {
            Some(row) if row > 0 => row - 1,
            _ => 99,
        }
    }

    pub fn get_row_col_by_pos(pos: &Pos) -> (usize, usize) {
        return (Loc::get_row_by_y(pos.top), Loc::get_col_by_x(pos.left));
    }
}
