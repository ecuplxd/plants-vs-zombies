use std::{mem, rc::Rc};

use web_sys::{CanvasRenderingContext2d, HtmlImageElement};

use crate::{
    game::Game,
    marching_squares::MarchingSquares,
    sprites::model::{CollisionMargin, Pos, SpriteCell},
};

use super::model::{Draw, Resource, Stroke};

pub struct Artist {
    pub image: Rc<HtmlImageElement>,
    pub cell_index: usize,
    pub cells: Vec<SpriteCell>,
    original_cells: Vec<SpriteCell>,
    original_index: usize,
    pub scale: f64,
}

impl Artist {
    pub fn new(image: Rc<HtmlImageElement>, cells: Vec<SpriteCell>, scale: f64) -> Artist {
        Artist {
            image,
            cell_index: 0,
            cells: cells.to_vec(),
            original_index: 0,
            original_cells: cells.to_vec(),
            scale,
        }
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

    pub fn get_image_outline_points2(
        image: &HtmlImageElement,
        cell: &SpriteCell,
        offset_x: f64,
        offset_y: f64,
        normal_shape: bool,
        scale: f64,
    ) -> Vec<Pos> {
        if normal_shape {
            return vec![
                Pos::new(offset_x, offset_y),
                Pos::new(offset_x + cell.width * scale, offset_y),
                Pos::new(
                    offset_x + cell.width * scale,
                    offset_y + cell.height * scale,
                ),
                Pos::new(offset_x, offset_y + cell.height * scale),
            ];
        }

        let width = cell.width as u32;
        let height = cell.height as u32;
        let offscreen_canvas = Game::create_canvas(width, height);
        let offscreen_context = Game::get_canvas_context(&offscreen_canvas);
        let pos = Pos::new(0.0, 0.0);

        Artist::execute_draw_image(&offscreen_context, image, &pos, &cell, 1.0);

        let image_data = offscreen_context
            .get_image_data(0.0, 0.0, width as f64, height as f64)
            .unwrap()
            .data();
        let marching_squares = MarchingSquares::new(offset_x, offset_y);
        let outline_points = marching_squares.get(&image_data, width as i32, height as i32);

        return outline_points;
    }

    pub fn draw_image(&self, context: &CanvasRenderingContext2d, pos: &Pos, cell: &SpriteCell) {
        Artist::execute_draw_image(context, self.image.as_ref(), pos, cell, self.scale);
    }

    pub fn get_image_outline_points(&self, pos: &Pos) -> Vec<Pos> {
        let offscreen_canvas = Game::create_canvas(900, 600);
        let offscreen_context = Game::get_canvas_context(&offscreen_canvas);
        let cell = self.get_current_cell();

        if let Some(cell) = cell {
            self.draw_image(&offscreen_context, pos, cell);

            let (width, height) = self.width_height(&offscreen_context);
            let image_data = offscreen_context
                .get_image_data(0.0, 0.0, width, height)
                .unwrap()
                .data();
            let marching_squares = MarchingSquares::new(0.0, 0.0);
            let outline_points = marching_squares.get(&image_data, width as i32, height as i32);

            return outline_points;
        } else {
            return vec![];
        }
    }

    fn in_cell(&self, index: usize) -> bool {
        return self.cell_index == index;
    }

    fn swap_cell(&mut self) {
        mem::swap(&mut self.cells, &mut self.original_cells);
        mem::swap(&mut self.cell_index, &mut self.original_index);
    }
}

impl Stroke for Artist {}

impl Resource for Artist {
    fn get_current_cell(&self) -> Option<&SpriteCell> {
        self.cells.get(self.cell_index)
    }

    fn in_last_cell(&self) -> bool {
        self.in_cell(self.cells.len() - 1)
    }

    fn advance(&mut self) {
        let prex_index = self.cell_index;

        if prex_index == self.cells.len() - 1 {
            self.goto(0);
        } else {
            self.goto(prex_index + 1);
        }
    }

    fn goto(&mut self, index: usize) {
        self.cell_index = index;
    }
}

impl Draw for Artist {
    fn draw(
        &self,
        context: &CanvasRenderingContext2d,
        pos: &Pos,
        collision_margin: &CollisionMargin,
    ) {
        let cell = self.get_current_cell();

        if let Some(cell) = cell {
            self.draw_image(context, pos, cell);
            self.rect(context, pos, cell, self.scale, collision_margin);
        }
    }

    fn update_scale(&mut self, scale: f64) {
        self.scale = scale;
    }

    fn switch(&mut self, cells: &Vec<SpriteCell>) {
        self.swap_cell();
        self.cells = cells.to_vec();
        self.goto(0);
    }

    fn revert(&mut self) {
        self.swap_cell();
    }
}
