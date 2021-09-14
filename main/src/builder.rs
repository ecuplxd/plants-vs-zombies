use std::collections::HashMap;
use std::rc::Rc;

use web_sys::HtmlImageElement;

use crate::artists::artist::Artist;
use crate::behavior::click::ClickBehavior;
use crate::behavior::collision::CollisionBehavior;
use crate::behavior::cycle::CycleBehavior;
use crate::behavior::drag::DragBehavior;
use crate::behavior::frequency::FrequencyBehavior;
use crate::behavior::interval::IntervalBehavior;
use crate::behavior::model::{BehaviorData, BehaviorType};
use crate::behavior::scroll::ScrollBehavior;
use crate::behavior::switch::SwitchBehavior;
use crate::behavior::walk::WalkBehavior;
use crate::behavior::{hover::HoverBehavior, model::Behavior};
use crate::callback::ErasedFnPointer;
use crate::loc::Loc;
use crate::model::{LocInfo, SpriteType};
use crate::sprites::model::{CollisionMargin, DrawInfo, Pos, SpriteCell, SpriteData};
use crate::sprites::plants::plant::PlantSprite;
use crate::sprites::sprite::Sprite;
use crate::sprites::sprite_wrap::SpriteWrap;
use crate::sprites::zombies::zombie::ZombieSprite;

pub struct Builder {
    pub sheets: HashMap<String, Rc<HtmlImageElement>>,
    pub cells: HashMap<String, Vec<SpriteCell>>,
    pub data: HashMap<String, SpriteData>,
}

impl Builder {
    pub fn new() -> Builder {
        Builder {
            sheets: HashMap::new(),
            cells: HashMap::new(),
            data: HashMap::new(),
        }
    }

    pub fn get_data(&self, name: &str) -> SpriteData {
        return match self.data.get(name) {
            Some(sprite_data) => (*sprite_data).clone(),
            None => SpriteData::new(vec![Pos::new(0.0, 0.0)], vec![]),
        };
    }

    pub fn get_cells(&self, name: &str) -> &Vec<SpriteCell> {
        let cells = self.cells.get(name).unwrap();

        return cells;
    }

    pub fn get_cells_may_not_exit(&self, name: &str) -> Option<&Vec<SpriteCell>> {
        return self.cells.get(name);
    }

    pub fn create_sprite(
        &self,
        sheet_name: &str,
        name: &str,
        sprite_data: Option<SpriteData>,
    ) -> Vec<SpriteWrap> {
        let mut sprites: Vec<SpriteWrap> = vec![];
        let cell_name = format!("{}/{}", sheet_name, name);
        let sheet_name = format!("main/assets/images/{}", sheet_name);
        let cells = self.get_cells(&cell_name);
        let image = self.sheets.get(&sheet_name).unwrap();
        let SpriteData {
            constructor,
            pos,
            offset,
            behaviors,
            visible,
            scale,
            order,
            collision_margin,
            ..
        } = match sprite_data {
            Some(sprite_data) => sprite_data,
            None => self.get_data(&cell_name),
        };
        let constructor = constructor.as_str();

        let collision_margin = match constructor {
            "ZombieSprite" => collision_margin,
            "PlantSprite" => collision_margin,
            _ => CollisionMargin::no_collision(),
        };

        for item in pos {
            let image2 = image.clone();
            let image = image.clone();
            let draw_info = DrawInfo::new(item, offset, visible, order);
            let artist = Artist::new(image, cells.to_vec(), scale);
            let sprite = Sprite::new(
                SpriteType::from_str(name),
                Box::new(artist),
                draw_info,
                collision_margin,
            );

            let mut behaviors: Vec<Box<dyn Behavior>> = behaviors
                .iter()
                .map(|behavior_data| {
                    self.create_behavior(&behavior_data, image2.as_ref(), &cells[0], &item, scale)
                })
                .collect();

            match constructor {
                "ZombieSprite" => {
                    // https://stackoverflow.com/questions/38302270/why-does-the-address-of-an-object-change-across-methods
                    // 必须先 Box 不能先给 zombie 注册回调再 Box
                    let mut zombie = Box::new(ZombieSprite::new(sprite));

                    Builder::register_zombie_callback(&mut zombie, &mut behaviors);

                    sprites.push(SpriteWrap::new(zombie, behaviors));
                }
                "PlantSprite" => {
                    let mut plant = Box::new(PlantSprite::new(sprite));

                    Builder::register_plant_callback(&mut plant, &mut behaviors);

                    sprites.push(SpriteWrap::new(plant, behaviors));
                }
                _ => {
                    sprites.push(SpriteWrap::new(Box::new(sprite), behaviors));
                }
            };
        }

        return sprites;
    }

    pub fn register_plant_callback(
        plant: &mut PlantSprite,
        behaviors: &mut Vec<Box<dyn Behavior>>,
    ) {
        for behavior in behaviors {
            match behavior.name() {
                BehaviorType::Switch => {
                    let pointer =
                        ErasedFnPointer::from_associated(plant, PlantSprite::swtich_callback);

                    behavior.set_cb(pointer);
                }
                BehaviorType::Interval => {
                    let pointer =
                        ErasedFnPointer::from_associated(plant, PlantSprite::interval_callback);

                    behavior.set_cb(pointer);
                }
                _ => (),
            }
        }
    }

    pub fn register_zombie_callback(
        zombie: &mut ZombieSprite,
        behaviors: &mut Vec<Box<dyn Behavior>>,
    ) {
        for behavior in behaviors {
            match behavior.name() {
                BehaviorType::Switch => {
                    let pointer =
                        ErasedFnPointer::from_associated(zombie, ZombieSprite::swtich_callback);

                    behavior.set_cb(pointer);
                }
                BehaviorType::Collision => {
                    let pointer =
                        ErasedFnPointer::from_associated(zombie, ZombieSprite::collision_callback);

                    behavior.set_cb(pointer);
                }
                _ => (),
            }
        }
    }

    pub fn from_names_and_poss(
        &self,
        sheet_name: &str,
        names: Vec<&str>,
        locs: Vec<Vec<Pos>>,
    ) -> Vec<SpriteWrap> {
        return locs
            .into_iter()
            .enumerate()
            .flat_map(|(i, pos)| self.from_name_and_pos(sheet_name, names[i], pos))
            .collect();
    }

    pub fn from_name_and_pos(
        &self,
        sheet_name: &str,
        name: &str,
        pos: Vec<Pos>,
    ) -> Vec<SpriteWrap> {
        let cell_name = format!("{}/{}", sheet_name, name);
        let mut sprite_data = self.get_data(&cell_name);

        sprite_data.pos = pos;

        return self.create_sprite(sheet_name, name, Some(sprite_data));
    }

    pub fn from_names(&self, sheet_name: &str, names: Vec<&str>) -> Vec<SpriteWrap> {
        return names
            .iter()
            .flat_map(|name| self.create_sprite(sheet_name, name, None))
            .collect();
    }

    pub fn create_behavior(
        &self,
        behavior_data: &BehaviorData,
        image: &HtmlImageElement,
        cell: &SpriteCell,
        pos: &Pos,
        scale: f64,
    ) -> Box<dyn Behavior> {
        let BehaviorData {
            name,
            duration,
            interval,
            rate,
            distance,
            normal_shape,
            infinite,
            switch_cells,
            direction,
            ..
        } = behavior_data;

        return match name {
            BehaviorType::Hover => {
                let points = Artist::get_image_outline_points2(
                    image,
                    cell,
                    pos.left,
                    pos.top,
                    *normal_shape,
                    scale,
                );

                Box::new(HoverBehavior::new(points))
            }
            BehaviorType::Click => {
                let points = Artist::get_image_outline_points2(
                    image,
                    cell,
                    pos.left,
                    pos.top,
                    *normal_shape,
                    scale,
                );

                Box::new(ClickBehavior::new(points))
            }
            BehaviorType::Cycle => Box::new(CycleBehavior::new(*duration, *interval)),
            BehaviorType::Walk => {
                Box::new(WalkBehavior::new(*rate, *duration, *direction, *distance))
            }
            BehaviorType::Switch => {
                let cells: Vec<Vec<SpriteCell>> = switch_cells
                    .iter()
                    .filter_map(|switch_cell| {
                        self.get_cells_may_not_exit(switch_cell)
                            .map(|cell| cell.to_vec())
                    })
                    .collect();

                Box::new(SwitchBehavior::new(cells, *duration, *infinite))
            }
            BehaviorType::Frequency => Box::new(FrequencyBehavior::new(*duration)),
            BehaviorType::Scroll => Box::new(ScrollBehavior::new(*distance, *rate)),
            BehaviorType::Collision => Box::new(CollisionBehavior::new()),
            BehaviorType::Drag => Box::new(DragBehavior::new()),
            BehaviorType::Interval => Box::new(IntervalBehavior::new(interval.unwrap())),
        };
    }

    pub fn create_plant(&self, loc_info: &LocInfo, is_plant: bool) -> SpriteWrap {
        let sheet_name = match is_plant {
            true => "plant",
            false => "zombie",
        };
        let cell_name = format!("{}/{}", sheet_name, loc_info.0);
        let cell = self.get_cells(&cell_name)[0];
        let pos = Loc::put_on_cell_bottom(loc_info.1, loc_info.2, cell.width, cell.height);
        let mut sprite_wrap = self
            .from_name_and_pos(sheet_name, &loc_info.0, vec![pos])
            .remove(0);

        sprite_wrap.sprite.update_loc(loc_info.1, loc_info.2);

        return sprite_wrap;
    }

    pub fn create_plants(&self, loc_infos: &Vec<LocInfo>, is_plant: bool) -> Vec<SpriteWrap> {
        return loc_infos
            .into_iter()
            .map(|loc_info| self.create_plant(loc_info, is_plant))
            .collect();
    }
}
