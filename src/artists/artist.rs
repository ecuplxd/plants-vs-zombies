use std::mem;
use std::rc::{Rc, Weak};

use web_sys::{CanvasRenderingContext2d, HtmlImageElement};

use super::Draw;
use crate::game::Game;
use crate::marching_squares::MarchingSquares;
use crate::sprites::{Pos, Size, SpriteCell};

pub struct Artist {
    pub scale: f64,
    image: Weak<HtmlImageElement>,
    cell_index: usize,
    cells: Vec<SpriteCell>,
    original_cells: Vec<SpriteCell>,
    original_index: usize,
}

impl Artist {
    pub fn new(image: &Rc<HtmlImageElement>, cells: Vec<SpriteCell>, scale: f64) -> Artist {
        Artist {
            scale,
            image: Rc::downgrade(&image),
            cell_index: 0,
            cells: cells.to_vec(),
            original_index: 0,
            original_cells: cells.to_vec(),
        }
    }

    pub fn get_normal_outline_points(size: &Size, offset: &Pos, scale: f64) -> Vec<Pos> {
        offset.to_rect_points(size, scale)
    }

    pub fn get_irregular_outline_points(
        image: &HtmlImageElement,
        cell: &SpriteCell,
        offset: &Pos,
        scale: f64,
    ) -> Vec<Pos> {
        let (width, height) = (cell.width, cell.height);
        let (width_u32, height_u32) = (cell.width as u32, cell.height as u32);
        let (width_i32, height_i32) = (cell.width as i32, cell.height as i32);
        let offscreen_canvas = Game::create_canvas(width_u32, height_u32);
        let offscreen_context = Game::get_canvas_context(&offscreen_canvas);
        let pos = Pos::new(0.0, 0.0);

        Artist::execute_draw_image(&offscreen_context, image, &pos, &cell, scale);

        let image_data = offscreen_context
            .get_image_data(0.0, 0.0, width, height)
            .unwrap()
            .data();
        let marching_squares = MarchingSquares::new(offset.left, offset.top);
        let outline_points = marching_squares.get(&image_data, width_i32, height_i32);

        return outline_points;
    }

    pub fn execute_draw_image(
        context: &CanvasRenderingContext2d,
        image: &HtmlImageElement,
        pos: &Pos,
        cell: &SpriteCell,
        scale: f64,
    ) {
        context
            .draw_image_with_html_image_element_and_sw_and_sh_and_dx_and_dy_and_dw_and_dh(
                image,
                cell.left,
                cell.top,
                cell.width,
                cell.height,
                pos.left,
                pos.top,
                cell.width * scale,
                cell.height * scale,
            )
            .unwrap();
    }

    pub fn goto(&mut self, index: usize) {
        self.cell_index = index;
    }

    fn in_cell(&self, index: usize) -> bool {
        return self.cell_index == index;
    }

    pub fn get_resource(&self) -> (Option<Rc<HtmlImageElement>>, Option<&SpriteCell>) {
        let cell = self.get_current_cell();
        let image = self.image.upgrade();

        (image, cell)
    }

    fn swap_cell(&mut self) {
        mem::swap(&mut self.cells, &mut self.original_cells);
        mem::swap(&mut self.cell_index, &mut self.original_index);
    }
}

impl Draw for Artist {
    fn draw_image(&self, context: &CanvasRenderingContext2d, pos: &Pos) {
        let (image, cell) = self.get_resource();

        match (cell, image) {
            (Some(cell), Some(image)) => {
                Artist::execute_draw_image(context, image.as_ref(), pos, cell, self.scale)
            }
            _ => (),
        }
    }

    fn switch(&mut self, cells: &Vec<SpriteCell>) {
        self.swap_cell();
        self.cells = cells.to_vec();
        self.goto(0);
    }

    fn revert(&mut self) {
        self.swap_cell();
    }

    fn advance(&mut self) {
        let prex_index = self.cell_index;
        let index = match prex_index == self.cells.len() - 1 {
            true => 0,
            false => prex_index + 1,
        };

        self.goto(index);
    }

    fn in_last_cell(&self) -> bool {
        self.in_cell(self.cells.len() - 1)
    }

    fn get_current_cell(&self) -> Option<&SpriteCell> {
        self.cells.get(self.cell_index)
    }
}
