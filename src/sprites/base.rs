use std::any::Any;

use web_sys::CanvasRenderingContext2d;

use super::zombies::Zombie;
use super::{BaseUpdate, CollisionMargin, PlantSprite, Pos, Size, SpriteCell, SpriteData, Update};
use crate::artists::{Artist, Draw, Stroke};
use crate::behaviors::{Behavior, BehaviorFactory};
use crate::loc::Loc;
use crate::model::{Resource, SheetKind, SpriteType};
use crate::util::get_random_string;

pub struct Sprite {
    id: String,
    pub name: SpriteType,
    pub size: Size,
    pub pos: Pos,
    collision_margin: CollisionMargin,
    pub artist: Artist,
    pub outlines: Vec<Pos>,
    pub behaviors: Vec<Box<dyn Behavior>>,
    pub visible: bool,
    pub offset: Pos,
    order: usize,
    pub loc: Loc,
    pub clicked: bool,
    normal_shape: bool,
    pub global_alpha: f64,
}

impl Sprite {
    pub fn new(
        name: SpriteType,
        artist: Artist,
        pos: Pos,
        offset: Pos,
        collision_margin: CollisionMargin,
        order: usize,
        normal_shape: bool,
    ) -> Sprite {
        let cell = artist.get_current_cell();
        let size = match cell {
            Some(cell) => cell.into(),
            None => Default::default(),
        };

        Sprite {
            id: get_random_string(name.to_string()),
            name,
            size,
            pos,
            collision_margin,
            artist,
            behaviors: vec![],
            outlines: vec![],
            visible: true,
            offset,
            order,
            loc: Default::default(),
            clicked: false,
            normal_shape,
            global_alpha: 1.0,
        }
    }

    pub fn from_names(
        names: Vec<&str>,
        resouce: &Resource,
        sheet_kind: SheetKind,
    ) -> Vec<Box<dyn Update>> {
        names
            .iter()
            .flat_map(|name| Sprite::from_data(resouce, sheet_kind, name))
            .collect()
    }

    pub fn from_data_one(resouce: &Resource, sheet_kind: SheetKind, name: &str) -> Box<dyn Update> {
        let mut sprites = Sprite::from_data(resouce, sheet_kind, name);

        sprites.remove(0)
    }

    pub fn from_data(resouce: &Resource, sheet_kind: SheetKind, name: &str) -> Vec<Box<dyn Update>> {
        let (cell_name, sheet_name, sheet_kind) = Resource::get_name(sheet_kind, name);
        let SpriteData {
            constructor,
            pos,
            offset,
            scale,
            collision_margin,
            behaviors,
            order,
            normal_shape,
            life,
            hurt,
            ..
        } = resouce.get_data(&cell_name);

        let constructor = constructor.as_str();
        // TODO：使用枚举
        let collision_margin = match constructor {
            "Zombie" => collision_margin,
            "PlantSprite" => collision_margin,
            _ => CollisionMargin::no_collision(),
        };

        let sprites = pos
            .iter()
            .map(|pos| {
                let image = resouce.get_sheet(&sheet_name);
                let cell = resouce.get_cell(&cell_name);
                let artist = Artist::new(image, cell.to_vec(), scale);
                let sprite = Sprite::new(
                    SpriteType::from_str(name),
                    artist,
                    *pos,
                    offset,
                    collision_margin,
                    order,
                    normal_shape,
                );

                let mut sprite: Box<dyn Update> = match constructor {
                    "Zombie" => Box::new(Zombie::new(life, hurt, sprite)),
                    "PlantSprite" => Box::new(PlantSprite::new(life, hurt, sprite)),
                    _ => Box::new(sprite),
                };

                behaviors.iter().for_each(|behavior| {
                    let mut behavior = BehaviorFactory::create(resouce, behavior, &sheet_kind);

                    behavior.set_sprite(sprite.as_mut());
                    sprite.add_behavior(behavior);
                });

                sprite
            })
            .collect();

        sprites
    }
}

impl Stroke for Sprite {}

impl Draw for Sprite {
    fn draw(&self, context: &CanvasRenderingContext2d) {
        let Pos { left, top } = self.offset;

        context.save();
        context.set_global_alpha(self.global_alpha);
        context.translate(-left, -top).unwrap();

        self.artist.draw_image(context, &self.pos);
        self.rect(
            context,
            &self.pos,
            &self.size,
            self.artist.scale,
            &self.collision_margin,
        );

        context.translate(left, top).unwrap();
        context.restore();
    }
}

impl BaseUpdate for Sprite {
    fn id(&self) -> String {
        self.id.clone()
    }

    fn add_behavior(&mut self, behavior: Box<dyn Behavior>) {
        self.behaviors.push(behavior);
    }

    fn get_ref_artist(&self) -> &dyn Draw {
        &self.artist
    }

    fn get_mut_artist(&mut self) -> &mut dyn Draw {
        &mut self.artist
    }

    fn get_mut_behaviors(&mut self) -> &mut Vec<Box<dyn Behavior>> {
        &mut self.behaviors
    }

    fn get_collision_margin(&self) -> CollisionMargin {
        self.collision_margin
    }

    fn show(&mut self) {
        self.visible = true;
    }

    fn hide(&mut self) {
        self.visible = false;
    }

    fn is_visible(&self) -> bool {
        self.visible
    }

    fn name(&self) -> SpriteType {
        self.name
    }

    fn point_in_path(&self, mouse_pos: &Pos, context: &CanvasRenderingContext2d) -> bool {
        self.outline(context, &self.outlines);
        let Pos { left, top } = mouse_pos;

        context.is_point_in_path_with_f64(*left, *top)
    }

    fn update_outlines(&mut self) {
        let (image, cell) = self.artist.get_resource();

        self.outlines = match (cell, image) {
            (Some(cell), Some(image)) => match self.normal_shape {
                true => Artist::get_normal_outline_points(&self.size, &self.pos, self.artist.scale),
                false => Artist::get_irregular_outline_points(
                    image.as_ref(),
                    cell,
                    &self.pos,
                    self.artist.scale,
                ),
            },
            _ => vec![],
        };
    }

    fn get_order(&self) -> usize {
        self.order
    }

    fn set_order(&mut self, order: usize) {
        self.order = order;
    }

    fn get_rect(&self) -> SpriteCell {
        SpriteCell {
            left: self.pos.left,
            top: self.pos.top,
            width: self.size.width,
            height: self.size.height,
        }
    }

    fn get_pos(&self) -> Pos {
        self.pos
    }

    fn get_loc(&self) -> Loc {
        self.loc
    }

    fn update_loc(&mut self, loc: Loc) {
        self.loc = loc;
    }

    fn get_offset(&self) -> Pos {
        self.offset
    }

    fn update_offset(&mut self, offset: Pos) {
        self.offset = offset;
    }

    fn set_clicked(&mut self, clicked: bool) {
        self.clicked = clicked;
    }

    fn is_clicked(&self) -> bool {
        self.clicked
    }

    fn as_any(&mut self) -> &mut dyn Any {
        self
    }
}

impl Update for Sprite {
    fn update_pos(&mut self, pos: Pos) {
        self.pos = pos;
    }
}
