use std::collections::HashMap;

use serde::Deserialize;
use wasm_bindgen::JsValue;
use web_sys::CanvasRenderingContext2d;

use crate::{artists::model::Draw, behavior::model::BehaviorData, model::SpriteType};

/* *************** trait *************** */

pub trait Update {
    fn draw(&self, context: &CanvasRenderingContext2d);

    fn get_artist(&mut self) -> &mut dyn Draw;

    fn get_read_artist(&self) -> &dyn Draw;

    fn get_draw_info(&self) -> Option<&DrawInfo> {
        return None;
    }

    fn update_draw_info(&mut self, _pos: Option<Pos>, _offset: Option<Offset>) {}

    fn name(&self) -> SpriteType {
        SpriteType::Nil
    }

    fn toggle(&mut self) {}

    fn is_visible(&self) -> bool {
        return true;
    }

    fn get_order(&self) -> usize {
        0
    }

    fn tirgger_switch(&mut self) -> (bool, usize) {
        (false, 0)
    }

    fn check_collision(&self, _sprites: &Vec<&Box<dyn Update>>) -> bool {
        false
    }

    fn is_collision(&self) -> bool {
        false
    }

    fn update_loc(&mut self, _row: usize, _col: usize) {}

    fn get_loc(&self) -> (usize, usize) {
        (99, 99)
    }

    fn is_clicked(&self) -> bool {
        false
    }

    fn set_clicked(&mut self, _clicked: bool) {}

    fn get_gen_sun_flag(&self) -> bool {
        false
    }

    fn set_gen_sun_flag(&mut self, _gened: bool) {}

    fn get_shoot(&self) -> bool {
        false
    }

    fn set_shoot(&mut self, _shoot: bool) {}
}

/* *************** Struct *************** */

/* CollisionMargin */
#[derive(Debug, Deserialize, Clone, Copy)]
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

/* Pos */
#[derive(Debug, Default, Clone, Copy, Deserialize)]
pub struct Pos {
    pub left: f64,
    pub top: f64,
}

impl Pos {
    pub fn new(left: f64, top: f64) -> Pos {
        Pos { left, top }
    }
}

impl PartialEq for Pos {
    fn eq(&self, other: &Self) -> bool {
        self.left == other.left && self.top == other.top
    }
}

/* Offset */
#[derive(Debug, Default, Clone, Copy, Deserialize)]
pub struct Offset {
    pub x: f64,
    pub y: f64,
}

impl Offset {
    pub fn new(x: f64, y: f64) -> Offset {
        Offset { x, y }
    }
}

/* Velocit */
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

/* SpriteCell */
#[derive(Debug, Default, Clone, Copy, Deserialize)]
pub struct SpriteCell {
    pub left: f64,
    pub top: f64,
    pub width: f64,
    pub height: f64,
}

impl SpriteCell {
    pub fn new(left: f64, top: f64, width: f64, height: f64) -> SpriteCell {
        SpriteCell {
            left,
            top,
            width,
            height,
        }
    }

    pub fn new_from_json(json: &JsValue) -> HashMap<String, Vec<SpriteCell>> {
        json.into_serde().unwrap()
    }
}

#[derive(Debug, Default, Clone, Deserialize)]
pub struct SpriteData {
    pub constructor: String,
    pub pos: Vec<Pos>,
    pub behaviors: Vec<BehaviorData>,
    #[serde(default)]
    pub offset: Offset,
    #[serde(default = "default_visible")]
    pub visible: bool,
    #[serde(default = "default_scale")]
    pub scale: f64,
    #[serde(default)]
    pub order: usize,
    #[serde(default)]
    pub collision_margin: CollisionMargin,
}

fn default_visible() -> bool {
    true
}

fn default_scale() -> f64 {
    1.0
}

impl SpriteData {
    pub fn new(pos: Vec<Pos>, behaviors: Vec<BehaviorData>) -> SpriteData {
        SpriteData {
            constructor: String::from("Sprite"),
            pos,
            behaviors,
            offset: Default::default(),
            visible: true,
            scale: 1.0,
            order: 0,
            collision_margin: Default::default(),
        }
    }

    pub fn new_from_json(json: &JsValue) -> HashMap<String, SpriteData> {
        json.into_serde().unwrap()
    }
}

#[derive(Debug, Default, Clone, Deserialize)]
pub struct DrawInfo {
    pub pos: Pos,
    pub offset: Offset,
    pub visible: bool,
    pub order: usize,
}

impl DrawInfo {
    pub fn new(pos: Pos, offset: Offset, visible: bool, order: usize) -> DrawInfo {
        DrawInfo {
            pos,
            offset,
            visible,
            order,
        }
    }
}
