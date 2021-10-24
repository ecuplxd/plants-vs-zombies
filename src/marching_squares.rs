use std::cell::Cell;
use std::rc::Rc;

use crate::sprites::Pos;

pub struct MarchingSquares {
    state: Rc<Cell<i32>>,
    offset_x: f64,
    offset_y: f64,
}

impl MarchingSquares {
    pub fn new(offset_x: f64, offset_y: f64) -> MarchingSquares {
        MarchingSquares {
            state: Rc::new(Cell::new(0)),
            offset_x,
            offset_y,
        }
    }

    pub fn get(&self, data: &[u8], width: i32, height: i32) -> Vec<Pos> {
        self.get_blob_outline_points(data, width, height)
    }

    fn get_blob_outline_points(&self, data4: &[u8], width: i32, height: i32) -> Vec<Pos> {
        let size = width * height;
        let mut data: Vec<u8> = vec![0; size as usize];

        for i in 0..size {
            data[i as usize] = data4[(i << 2) as usize];
        }

        let starting_point = self.get_first_non_transparent_pixel_top_down(&data, width, height);

        match starting_point {
            Some(starting_point) => {
                self.walk_perimeter(&data, width, height, starting_point.0, starting_point.1)
            }
            None => vec![],
        }
    }

    fn get_first_non_transparent_pixel_top_down(
        &self,
        data: &[u8],
        width: i32,
        height: i32,
    ) -> Option<(i32, i32)> {
        for h in 0..height {
            let mut idx = h * width;

            for w in 0..width {
                if data[idx as usize] > 0 {
                    return Some((w, h));
                }

                idx += 1;
            }
        }

        None
    }

    fn walk_perimeter(
        &self,
        data: &[u8],
        width: i32,
        height: i32,
        start_w: i32,
        start_h: i32,
    ) -> Vec<Pos> {
        let mut point_list: Vec<Pos> = vec![];
        let up = 1;
        let left = 2;
        let down = 3;
        let right = 4;
        let mut w = start_w;
        let mut h = start_h;

        loop {
            if w >= 0 && w < width && h >= 0 && h < height {
                point_list.push(Pos::new(
                    self.offset_x + (w - 1) as f64,
                    self.offset_y + h as f64,
                ));
            }

            let idx = (h - 1) * width + (w - 1);
            let next_step = self.step(idx, data, width);

            if next_step == up {
                h -= 1;
            } else if next_step == left {
                w -= 1;
            } else if next_step == down {
                h += 1;
            } else if next_step == right {
                w += 1;
            }

            if w == start_w && h == start_h {
                break;
            }
        }

        point_list.push(Pos::new(self.offset_x + w as f64, self.offset_x + h as f64));

        point_list
    }

    fn get_pixel(&self, data: &[u8], idx: i32) -> u8 {
        match data.get(idx as usize) {
            Some(value) => *value,
            None => 0,
        }
    }

    fn step(&self, idx: i32, data: &[u8], width: i32) -> i32 {
        let up_left = 0 < self.get_pixel(data, idx + 1);
        let up_right = 0 < self.get_pixel(data, idx + 2);
        let down_left = 0 < self.get_pixel(data, idx + width + 1);
        let down_right = 0 < self.get_pixel(data, idx + width + 2);
        let none = 0;
        let up = 1;
        let left = 2;
        let down = 3;
        let right = 4;
        let state_inner = self.state.clone();
        let mut state = 0;

        if up_left {
            state |= 1;
        }

        if up_right {
            state |= 2;
        }

        if down_left {
            state |= 4;
        }

        if down_right {
            state |= 8
        }

        let new_state = match state {
            1 => up,
            2 => right,
            3 => right,
            4 => left,
            5 => up,
            6 if state_inner.get() == up => left,
            6 => right,
            7 => right,
            8 => down,
            9 if state_inner.get() == right => up,
            9 => down,
            10 => down,
            11 => down,
            12 => left,
            13 => up,
            14 => left,
            _ => none,
        };

        state_inner.set(new_state);

        new_state
    }
}
