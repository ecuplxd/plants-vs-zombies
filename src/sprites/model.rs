use std::collections::HashMap;
use std::ops::Add;
use std::ptr::NonNull;

use serde::Deserialize;
use wasm_bindgen::JsValue;
use web_sys::TextMetrics;

use super::Update;
use crate::behaviors::BehaviorData;
use crate::model::{CANVAS_HEIGHT_F64, CANVAS_WIDTH_F64};
use crate::util::get_random_int_inclusive;

#[derive(Debug, Default, Clone, Deserialize)]
pub struct SpriteCell {
    pub left: f64,
    pub top: f64,
    pub width: f64,
    pub height: f64,
}

impl SpriteCell {
    pub fn from_json(json: &JsValue) -> HashMap<String, Vec<SpriteCell>> {
        json.into_serde().unwrap()
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct SpriteData {
    pub constructor: String,
    pub pos: Vec<Pos>,
    pub behaviors: Vec<BehaviorData>,
    #[serde(default)]
    pub offset: Pos,
    #[serde(default = "default_scale")]
    pub scale: f64,
    #[serde(default = "default_order")]
    pub order: usize,
    #[serde(default)]
    pub collision_margin: CollisionMargin,
    #[serde(default = "default_normal_shape")]
    pub normal_shape: bool,
}

fn default_normal_shape() -> bool {
    true
}

fn default_scale() -> f64 {
    1.0
}

fn default_order() -> usize {
    2
}

impl SpriteData {
    pub fn new(pos: Vec<Pos>, behaviors: Vec<BehaviorData>) -> SpriteData {
        SpriteData {
            constructor: String::from("Sprite"),
            pos,
            behaviors,
            offset: Default::default(),
            scale: 1.0,
            order: 2,
            collision_margin: Default::default(),
            normal_shape: true,
        }
    }

    pub fn from_json(json: &JsValue) -> HashMap<String, SpriteData> {
        json.into_serde().unwrap()
    }
}

#[derive(Debug, Default, PartialEq, Clone, Copy, Deserialize)]
pub struct Pos {
    pub left: f64,
    pub top: f64,
}

impl Pos {
    pub fn new(left: f64, top: f64) -> Pos {
        Pos { left, top }
    }

    pub fn random_sun_pos() -> Pos {
        let left = get_random_int_inclusive(100.0, 800.0);
        let top = get_random_int_inclusive(0.0, 75.0);

        Pos::new(left, -top)
    }

    pub fn scale(&self, scale_left: f64, scale_top: f64) -> Pos {
        Pos::new(self.left + scale_left, self.top + scale_top)
    }

    pub fn scale_left(&self, scale: f64) -> Pos {
        Pos::new(self.left + scale, self.top)
    }

    pub fn scale_top(&self, scale: f64) -> Pos {
        Pos::new(self.left, self.top + scale)
    }

    pub fn get_rect_points(&self, size: &Size, scale: f64) -> Vec<Pos> {
        let scale_left = size.width * scale;
        let scale_top = size.height * scale;

        vec![
            *self,
            self.scale_left(scale_left),
            self.scale(scale_left, scale_top),
            self.scale_top(scale_top),
        ]
    }

    pub fn out_of_bound(&self) -> bool {
        self.left < 0.0 || self.left > CANVAS_WIDTH_F64 || self.top > CANVAS_HEIGHT_F64
    }

    pub fn distance(&self) -> f64 {
        (self.left.abs().powf(2.0) + self.top.abs().powf(2.0)).sqrt()
    }
}

impl Add<f64> for Pos {
    type Output = Pos;

    fn add(self, rhs: f64) -> Pos {
        Pos::new(self.left + rhs, self.top)
    }
}

impl Add for Pos {
    type Output = Pos;

    fn add(self, rhs: Pos) -> Pos {
        Pos::new(self.left + rhs.left, self.top + rhs.top)
    }
}

#[derive(Default, Clone, Copy)]
pub struct Size {
    pub width: f64,
    pub height: f64,
}

impl Size {
    pub fn new(width: f64, height: f64) -> Size {
        Size { width, height }
    }
}

impl From<&SpriteCell> for Size {
    fn from(cell: &SpriteCell) -> Size {
        Size::new(cell.width, cell.height)
    }
}

impl From<TextMetrics> for Size {
    fn from(text_metrics: TextMetrics) -> Self {
        Size::new(
            text_metrics.width(),
            text_metrics.font_bounding_box_descent(),
        )
    }
}

#[derive(Debug, Clone, Copy, Deserialize)]
pub struct CollisionMargin {
    pub left: f64,
    pub top: f64,
    pub right: f64,
    pub bottom: f64,
}

impl CollisionMargin {
    pub fn new(left: f64, top: f64, right: f64, bottom: f64) -> CollisionMargin {
        CollisionMargin {
            left,
            top,
            right,
            bottom,
        }
    }

    pub fn no_collision() -> CollisionMargin {
        CollisionMargin {
            left: 0.0,
            top: 0.0,
            right: 0.0,
            bottom: 0.0,
        }
    }
}

impl Default for CollisionMargin {
    fn default() -> Self {
        CollisionMargin {
            left: 15.0,
            top: 5.0,
            right: 10.0,
            bottom: 5.0,
        }
    }
}

pub type SpritePointer = Option<NonNull<dyn Update>>;

#[derive(Debug, Default, Clone, Copy, Deserialize)]
pub struct Velocit {
    pub x: f64,
    pub y: f64,
}

impl Velocit {
    pub fn new(x: f64, y: f64) -> Velocit {
        Velocit { x, y }
    }
}

#[derive(Debug, Clone, Copy)]
pub enum PlantCallback {
    Switch,
}
