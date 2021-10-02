use std::ops::Add;

use crate::model::{COL_X_COORD, ROW_Y_COORD};
use crate::sprites::{Pos, Size, SpriteCell};

#[derive(Debug, Default, PartialEq, Clone, Copy)]
pub struct Loc {
    pub row: usize,
    pub col: usize,
}

impl Loc {
    pub fn new(row: usize, col: usize) -> Loc {
        Loc { row, col }
    }

    pub fn put_center(cell: &SpriteCell, size: &Size) -> Pos {
        let left = cell.left + (cell.width - size.width) / 2.0;
        let top = cell.top + (cell.height - size.height) / 2.0;

        Pos::new(left, top)
    }

    pub fn put_on_cell_bottom(loc: &Loc, size: &Size) -> Pos {
        let mut pos = Loc::put_on(loc);
        let pos2 = Loc::put_on(&loc.add(Loc::new(1, 1)));

        pos.left += (pos2.left - pos.left - size.width) / 2.0;
        pos.top = pos2.top - 5.0 - size.height;

        pos
    }

    pub fn put_on(loc: &Loc) -> Pos {
        let x = COL_X_COORD[loc.col];
        let y = ROW_Y_COORD[loc.row];

        Pos::new(x, y)
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

        pos
    }

    pub fn put_increase_y(x: f64, begin: f64, step: f64, count: usize) -> Vec<Pos> {
        let mut pos: Vec<Pos> = vec![];

        for i in 0..count {
            pos.push(Pos::new(x, begin + i as f64 * step));
        }

        pos
    }

    pub fn get_col_by_x(x: f64) -> usize {
        let col = COL_X_COORD.iter().position(|col| *col > x);

        match col {
            Some(col) if col > 0 => col - 1,
            _ => 0,
        }
    }

    pub fn get_row_by_y(y: f64) -> usize {
        let row = ROW_Y_COORD.iter().position(|row| *row > y);

        match row {
            Some(row) if row > 0 => row - 1,
            _ => 0,
        }
    }

    pub fn get_row_col_by_pos(pos: &Pos) -> Loc {
        let row = Loc::get_row_by_y(pos.top);
        let col = Loc::get_col_by_x(pos.left);

        Loc::new(row, col)
    }

    pub fn in_same_row(&self, loc: &Loc) -> bool {
        self.row == loc.row
    }

    pub fn in_same_col(&self, loc: &Loc) -> bool {
        self.col == loc.col
    }

    #[inline]
    pub fn out_of_plant_bound(&self) -> bool {
        self.col == 0 || self.col > 9 || self.row > 4
    }
}

impl Add for Loc {
    type Output = Loc;

    fn add(self, rhs: Loc) -> Loc {
        Loc::new(self.row + rhs.row, self.col + rhs.col)
    }
}
