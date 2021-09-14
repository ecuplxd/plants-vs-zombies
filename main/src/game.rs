use std::cell::Cell;
use std::rc::Rc;

use wasm_bindgen::JsCast;
use web_sys::{CanvasRenderingContext2d, HtmlCanvasElement, MouseEvent};

use crate::behavior::model::BehaviorType;
use crate::builder::Builder;
use crate::callback::ErasedFnPointer;
use crate::data::{CANVAS_HEIGHT, CANVAS_WIDTH};
use crate::loc::Loc;
use crate::log;
use crate::model::{Callback, Event, Interface, LevelData, Plant, SpriteType, State};
use crate::scene::home::HomeScene;
use crate::scene::level::LevelScene;
use crate::sprites::guideline::Guideline;
use crate::sprites::model::{Pos, Update};
use crate::sprites::plants::plant::PlantSprite;
use crate::sprites::sprite_wrap::SpriteWrap;
use crate::util::window;
use crate::{fps::Fps, time_system::TimeSystem};

pub struct Game {
    pub time_system: TimeSystem,
    pub fps: Fps,
    pub time_rate: f64,

    pub builder: Builder,

    pub canvas: HtmlCanvasElement,
    pub context: CanvasRenderingContext2d,

    pub enter: Rc<Cell<bool>>,
    pub pressed: Rc<Cell<bool>>,
    pub dragging: Rc<Cell<bool>>,
    pub cur_pos: Rc<Cell<Pos>>,

    pub sprites: Vec<SpriteWrap>,

    pub now: f64,

    pub leval_data: Vec<LevelData>,

    pub cur_level: LevelData,

    pub last_gc_time: f64,

    pub state: State,

    pub sun: u32,
    pub sun_produce_rate: f64,
}

impl Game {
    pub fn new() -> Game {
        let document = window().document().unwrap();
        let canvas = Game::create_canvas(CANVAS_WIDTH, CANVAS_HEIGHT);
        let context = Game::get_canvas_context(&canvas);

        document.body().unwrap().append_child(&canvas).unwrap();

        Game {
            now: 0.0,
            time_system: TimeSystem::new(),
            fps: Fps::new(),
            time_rate: 1.0,

            builder: Builder::new(),

            canvas,
            context,

            enter: Rc::new(Cell::new(false)),
            cur_pos: Rc::new(Cell::new(Pos::new(0.0, 0.0))),
            pressed: Rc::new(Cell::new(false)),
            dragging: Rc::new(Cell::new(false)),

            sprites: vec![],

            leval_data: vec![],

            cur_level: LevelData::new_default(),

            last_gc_time: 0.0,

            state: State::new(),

            sun: 150,
            sun_produce_rate: 5000.0,
        }
    }

    pub fn create_canvas(width: u32, height: u32) -> HtmlCanvasElement {
        let document = window().document().unwrap();
        let canvas = document
            .create_element("canvas")
            .unwrap()
            .dyn_into::<HtmlCanvasElement>()
            .unwrap();

        canvas.set_width(width);
        canvas.set_height(height);

        return canvas;
    }

    pub fn get_canvas_context(canvas: &HtmlCanvasElement) -> CanvasRenderingContext2d {
        return canvas
            .get_context("2d")
            .unwrap()
            .unwrap()
            .dyn_into::<CanvasRenderingContext2d>()
            .unwrap();
    }

    pub fn init(&mut self) {
        self.time_system.start();
        // self.create_home_scene();
        self.debug_start();
    }

    fn debug_start(&mut self) {
        self.state.selected_card = true;
        self.create_level_background();
        self.choose_plant_card();
        self.create_plant_card_left();
    }

    fn sunback(&mut self) {
        let mut seed_chooser = LevelScene::seed_chooser(&self.builder);
        let mut sunback = seed_chooser.remove(1);

        sunback.sprite.toggle();

        self.sprites.push(sunback);
        self.toggle_sunback();
    }

    fn find_sprite(&mut self, sprite_type: SpriteType) -> Option<&mut SpriteWrap> {
        return self
            .sprites
            .iter_mut()
            .find(|sprite_wrap| sprite_wrap.sprite.name() == sprite_type);
    }

    fn remove_sprites(&mut self, sprite_types: &Vec<SpriteType>) {
        sprite_types.iter().for_each(|sprite_type| {
            let index = self
                .sprites
                .iter()
                .position(|sprite| sprite.name() == *sprite_type);

            if let Some(index) = index {
                self.sprites.remove(index);
            }
        });
    }

    fn remove_unvisible_sprites(&mut self) {
        let mut remove_indexs: Vec<usize> = self
            .sprites
            .iter()
            .enumerate()
            .filter_map(|(index, sprite)| match sprite.is_visible() {
                true => None,
                false => Some(index),
            })
            .collect();

        remove_indexs.reverse();
        remove_indexs.iter().for_each(|index| {
            self.sprites.remove(*index);
        });
    }

    fn shovel_back(&mut self) -> bool {
        let mut shovel_move = false;
        let shovel_data = self.builder.get_data("interface/Shovel");
        let shovel = self.find_sprite(SpriteType::Interface(Interface::Shovel));

        if let Some(shovel) = shovel {
            let shovel_original_pos = shovel_data.pos[0];

            shovel_move = match shovel.get_draw_info() {
                Some(draw_info) => draw_info.pos.top != 0.0,
                None => false,
            };
            shovel.update_draw_info(Some(shovel_original_pos), None);
        }

        shovel_move
    }

    fn shovel_plant(&mut self) {
        let cur_pos_loc = Loc::get_row_col_by_pos(&self.cur_pos.get());
        let remove_plant_index = self
            .sprites
            .iter()
            .position(|sprite| sprite.is_plant() && cur_pos_loc == sprite.get_loc());

        if let Some(remove_plant_index) = remove_plant_index {
            self.sprites.remove(remove_plant_index);
        }
    }

    fn schedule_draw_order(&mut self) {
        self.sprites
            .sort_by(|a, b| a.get_order().cmp(&b.get_order()))
    }

    pub fn dispatch_event(&mut self, name: Event, event: MouseEvent) {
        let pressed = self.pressed.clone();
        let dragging = self.dragging.clone();
        let cur_pos = self.cur_pos.clone();
        let x = event.offset_x() as f64;
        let y = event.offset_y() as f64;

        cur_pos.set(Pos::new(x, y));

        unsafe {
            log!("{} : {}, {}", name, x, y);
        }

        match name {
            Event::Mousedown => {
                pressed.set(true);
                self.mousedonw_handler(x, y);
            }

            Event::Mouseup => {
                if self.state.selected_card && pressed.get() && dragging.get() && self.shovel_back()
                {
                    self.shovel_plant();
                }

                pressed.set(false);
                dragging.set(false);

                self.mouseup_handler(x, y);
            }

            Event::Mouseenter => {
                self.enter.set(true);
                self.mouseenter_handler(x, y);
            }

            Event::Mouseleave => {
                self.enter.set(false);
                self.mouseleave_handler(x, y);
            }

            Event::Mousemove => {
                if pressed.get() {
                    if !dragging.get() {
                        self.toggle_behaviors(&vec![BehaviorType::Drag], true);
                    }

                    dragging.set(true);
                }

                self.mousemove_handler(x, y);
            }
        };
    }

    fn mouseenter_handler(&mut self, _x: f64, _y: f64) {}

    fn mousedonw_handler(&mut self, _x: f64, _y: f64) {
        self.toggle_behaviors(&vec![BehaviorType::Click], true);
    }

    fn mouseup_handler(&mut self, _x: f64, _y: f64) {
        self.toggle_behaviors(&vec![BehaviorType::Click, BehaviorType::Drag], false);
    }

    fn mousemove_handler(&mut self, _x: f64, _y: f64) {
        self.toggle_behaviors(&vec![BehaviorType::Hover], true);
    }

    fn mouseleave_handler(&mut self, _x: f64, _y: f64) {
        self.toggle_behaviors(&vec![BehaviorType::Hover], false);
    }

    fn toggle_behaviors(&mut self, behavior_types: &Vec<BehaviorType>, flag: bool) {
        for sprite in &mut self.sprites {
            sprite.toggle_behaviors(behavior_types, flag, self.now);
        }
    }

    pub fn collect_sun(&mut self) {
        let sun_index = self.sprites.iter().position(|sprite| {
            sprite.name() == SpriteType::Interface(Interface::Sun) && sprite.is_clicked()
        });

        if let Some(sun_index) = sun_index {
            self.sprites.remove(sun_index);
        }

        if self.state.cur_sun > 1 {
            self.state.cur_sun -= 1;
        }

        self.sun += 50;
    }

    pub fn run(&mut self) {
        self.now = self.time_system.calculate_game_time();

        if self.now - self.last_gc_time > self.sun_produce_rate {
            self.last_gc_time = self.now;

            if self.state.selected_card && self.state.cur_sun < self.state.max_sun {
                self.drop_sun(None);
                self.state.cur_sun += 1;
            }

            self.gc();
        }

        self.fps.calc(self.now, self.time_rate);
        self.update();
        self.fps.update(self.now);
    }

    fn map_callback(&mut self, callback: Callback) -> ErasedFnPointer {
        let pointer = match callback {
            Callback::ShowReady => ErasedFnPointer::from_associated(self, Game::show_ready),
            Callback::TurnToLevelPrepareScene => {
                ErasedFnPointer::from_associated(self, Game::turn_to_level_prepare_scene)
            }
            Callback::ChoosePlantCard => {
                ErasedFnPointer::from_associated(self, Game::choose_plant_card)
            }
            Callback::BattleSetup => ErasedFnPointer::from_associated(self, Game::battle_setup),
            Callback::ResetCardSelect => {
                ErasedFnPointer::from_associated(self, Game::reset_card_select)
            }
            Callback::SelectPlantSeed => {
                ErasedFnPointer::from_associated(self, Game::select_plant_seed)
            }
            Callback::StartBattle => ErasedFnPointer::from_associated(self, Game::start_battle),
            Callback::CollectSun => ErasedFnPointer::from_associated(self, Game::collect_sun),
        };

        return pointer;
    }

    fn register_callbacks(
        &mut self,
        sprites: &mut Vec<SpriteWrap>,
        sprite_type: SpriteType,
        behavior_type: BehaviorType,
        callbacks: &Vec<Callback>,
    ) {
        let behaviors = SpriteWrap::find_sprite_behaviors(sprites, sprite_type, behavior_type);
        let mut i: usize = 0;

        for behavior in behaviors {
            let callback = match callbacks.get(i) {
                Some(callback) => callback,
                None => &callbacks[0],
            };
            let pointer = self.map_callback(*callback);

            behavior.set_cb(pointer);
            i += 1;
        }
    }

    fn register_callback(
        &mut self,
        sprites: &mut Vec<SpriteWrap>,
        sprite_type: SpriteType,
        behavior_type: BehaviorType,
        callback: Callback,
    ) {
        let behavior = SpriteWrap::find_sprite_behavior(sprites, sprite_type, behavior_type);

        if let Some(behavior) = behavior {
            let pointer = self.map_callback(callback);

            behavior.set_cb(pointer);
        }
    }

    fn drop_sun(&mut self, pos: Option<&Pos>) {
        let mut sun = LevelScene::drop_sun(&self.builder, pos);

        sun[0].toggle_behaviors(
            &vec![BehaviorType::Cycle, BehaviorType::Walk],
            true,
            self.now,
        );

        self.register_callback(
            &mut sun,
            SpriteType::Interface(Interface::Sun),
            BehaviorType::Click,
            Callback::CollectSun,
        );

        self.sprites.extend(sun);
    }

    fn shoot(&mut self, pos: Pos, sprite_type: &SpriteType) {
        let bullet = match sprite_type {
            SpriteType::Plant(Plant::Peashooter) => Some("PB00"),
            SpriteType::Plant(Plant::SnowPea) => Some("PB100"),
            _ => None,
        };

        if let Some(bullet) = bullet {
            let mut bullet_sprite = self.builder.from_name_and_pos("plant", bullet, vec![pos]);

            for bullet in &mut bullet_sprite {
                bullet.toggle_behaviors(&vec![BehaviorType::Walk], true, self.now);
            }

            self.sprites.extend(bullet_sprite);
        }
    }

    /* sprite 创建 */
    fn create_guideline(&mut self) {
        self.sprites.push(Guideline::new());
    }

    fn create_home_scene(&mut self) {
        let mut homes = HomeScene::create(&self.builder);

        self.register_callback(
            &mut homes,
            SpriteType::Interface(Interface::SelectorAdventureButton),
            BehaviorType::Click,
            Callback::ShowReady,
        );

        self.register_callback(
            &mut homes,
            SpriteType::Interface(Interface::SelectorZombieHand),
            BehaviorType::Frequency,
            Callback::TurnToLevelPrepareScene,
        );

        self.sprites.extend(homes);
    }

    fn create_level_background(&mut self) {
        let mut level_scenes = LevelScene::create(&self.builder, &self.cur_level.scenes[0..1]);

        self.register_callback(
            &mut level_scenes,
            SpriteType::Interface(Interface::Background1),
            BehaviorType::Scroll,
            Callback::ChoosePlantCard,
        );

        self.sprites.extend(level_scenes);
    }

    fn create_level_scene(&mut self) {
        let mut level_scenes = LevelScene::create(&self.builder, &self.cur_level.scenes[1..]);

        self.register_callback(
            &mut level_scenes,
            SpriteType::Interface(Interface::PrepareGrowPlants),
            BehaviorType::Frequency,
            Callback::StartBattle,
        );

        self.sprites.extend(level_scenes);
    }

    fn create_flag(&mut self) {
        let flags = LevelScene::flag(&self.builder, self.cur_level.flag_num);

        self.sprites.extend(flags);
    }

    fn create_plant_card_left(&mut self) {
        let mut plant_card = LevelScene::plant_card(&self.builder, &self.cur_level.plant_cards);

        plant_card
            .iter_mut()
            .for_each(|card| card.update_scale(1.0));

        self.sprites.extend(plant_card);
    }

    fn create_plants(&mut self) {
        let plants = LevelScene::plants(&self.builder, &self.cur_level.plants);

        self.sprites.extend(plants);
    }

    fn create_zombies(&mut self) {
        let zombies = LevelScene::zombies(&self.builder, &self.cur_level.zombies);

        self.sprites.extend(zombies);
    }

    /* 回调 */
    // 僵尸手动画结束转场
    fn turn_to_level_prepare_scene(&mut self) {
        self.sprites.clear();

        self.create_level_background();
        self.toggle_behaviors(&vec![BehaviorType::Scroll], true);
    }

    // 背景移动到最右/最左开始战斗
    fn choose_plant_card(&mut self) {
        if self.state.selected_card {
            self.toggle_sunback();
            self.create_flag();
            self.create_guideline();
            self.create_level_scene();

            self.schedule_draw_order();

            self.toggle_behaviors(&vec![BehaviorType::Frequency], true);
        } else {
            self.prepare_choose_card();
        }
    }

    // 最右选择卡片
    fn prepare_choose_card(&mut self) {
        let mut seed_chooser = LevelScene::seed_chooser(&self.builder);
        let mut plant_card = LevelScene::plant_seed(&self.builder, &self.cur_level.plant_cards);

        self.register_callbacks(
            &mut seed_chooser,
            SpriteType::Interface(Interface::SelectCardButton),
            BehaviorType::Click,
            &vec![Callback::ResetCardSelect, Callback::BattleSetup],
        );

        plant_card.iter_mut().for_each(|card| {
            card.register_callback(
                BehaviorType::Click,
                self.map_callback(Callback::SelectPlantSeed),
            )
        });

        self.sprites.extend(seed_chooser);
        self.sprites.extend(plant_card);
        self.create_plant_card_left();
    }

    fn reset_card_select(&mut self) {}

    fn toggle_sunback(&mut self) {
        let sun_back = self.find_sprite(SpriteType::Interface(Interface::SunBack));

        if let Some(sun_back) = sun_back {
            sun_back.sprite.toggle();

            if sun_back.is_visible() {
                sun_back.update_draw_info(Some(Pos::new(100.0, 0.0)), None);
            }
        }
    }

    fn start_battle(&mut self) {
        self.create_plants();
        self.create_zombies();
        self.sunback();

        // self.schedule_draw_order();

        self.toggle_behaviors(
            &vec![
                BehaviorType::Cycle,
                BehaviorType::Walk,
                BehaviorType::Switch,
                BehaviorType::Collision,
                BehaviorType::Interval,
            ],
            true,
        );
    }

    fn battle_setup(&mut self) {
        let mut remove_sprites = vec![
            SpriteType::Interface(Interface::SeedChooserBackground),
            SpriteType::Interface(Interface::SelectCardButton),
            SpriteType::Interface(Interface::SelectCardButton),
        ];

        remove_sprites.extend(
            self.cur_level
                .plant_cards
                .iter()
                .map(|plant_card| SpriteType::from_str(plant_card)),
        );

        self.toggle_sunback();
        self.remove_sprites(&remove_sprites);
        self.state.selected_card = true;
        self.toggle_behaviors(&vec![BehaviorType::Scroll], true);
    }

    fn select_plant_seed(&mut self) {
        unsafe { log!("选择植物") };
    }

    /* 行为激活 */
    fn show_ready(&mut self) {
        self.state.home_ready = true;
        self.toggle_behaviors(&vec![BehaviorType::Frequency], true);
    }

    // TOOD：优化
    fn before_update(&mut self) {
        let mut sun_poss: Vec<Pos> = vec![];
        let mut bullet_poss: Vec<(Pos, SpriteType)> = vec![];

        for sprite in &mut self.sprites {
            match sprite.name() {
                SpriteType::Plant(Plant::SunFlower1) => {
                    let gened = sprite.sprite.get_gen_sun_flag();

                    if gened {
                        sun_poss.push(PlantSprite::get_sun_pos(&sprite.sprite));

                        sprite.sprite.set_gen_sun_flag(false);
                    }
                }
                SpriteType::Plant(Plant::Peashooter) | SpriteType::Plant(Plant::SnowPea) => {
                    let shoot = sprite.sprite.get_shoot();

                    if shoot {
                        bullet_poss.push(PlantSprite::get_bullet_pos(&sprite.sprite));

                        sprite.sprite.set_shoot(false);
                    }
                }
                _ => (),
            }
        }

        for sun_pos in &sun_poss {
            self.drop_sun(Some(sun_pos));
        }

        for (pos, sprite_type) in &bullet_poss {
            self.shoot(*pos, sprite_type)
        }
    }

    /* 绘制、行为处理 */
    fn update(&mut self) {
        self.before_update();
        self.check_collision();
        self.update_spirte_behaviors();
        self.draw_sprites();
    }

    fn update_spirte_behaviors(&mut self) {
        let cur_pos = self.cur_pos.clone();
        let cur_pos = &cur_pos.get();

        for sprite in &mut self.sprites {
            sprite.update(
                self.now,
                self.fps.last_animation_frame_time,
                cur_pos,
                &self.context,
            );
        }
    }

    fn collect_collision_sprites(&self) -> Vec<&Box<dyn Update>> {
        let mut sprites: Vec<&Box<dyn Update>> = vec![];

        self.sprites
            .iter()
            .filter(|sprite| sprite.can_check_collision())
            .for_each(|sprite| sprites.push(&sprite.sprite));

        return sprites;
    }

    fn collect_candidate_sprites(&self) -> Vec<&Box<dyn Update>> {
        let mut candidate_sprites: Vec<&Box<dyn Update>> = vec![];

        self.sprites
            .iter()
            .filter(|sprite| sprite.can_candidate_for_collision())
            .for_each(|sprite| candidate_sprites.push(&sprite.sprite));

        return candidate_sprites;
    }

    fn check_collision(&self) {
        let sprites = self.collect_collision_sprites();

        if sprites.len() != 0 {
            let candidate_sprites = self.collect_candidate_sprites();

            for sprite in sprites {
                sprite.check_collision(&candidate_sprites);
            }
        }
    }

    fn draw_sprites(&self) {
        self.sprites
            .iter()
            .filter(|sprite| sprite.is_visible())
            .for_each(|sprite| sprite.draw(&self.context));

        self.draw_sun_num();
    }

    fn draw_sun_num(&self) {
        let num = match self.sun > 99999 {
            true => String::from("9999+"),
            false => self.sun.to_string(),
        };

        self.context.save();
        self.context.set_font("32px 黑体");
        self.context.fill_text(&num, 138.0, 30.0).unwrap();
        self.context.restore();
    }

    fn gc(&mut self) {
        self.remove_unvisible_sprites();
    }
}
