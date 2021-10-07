use std::cell::Cell;
use std::rc::Rc;

use wasm_bindgen::JsCast;
use web_sys::{CanvasRenderingContext2d, HtmlCanvasElement, MouseEvent};

use crate::artists::Draw;
use crate::behaviors::{BehaviorFactory, BehaviorType};
use crate::callback::ErasedFnPointer;
use crate::fps::Fps;
use crate::loc::Loc;
use crate::log;
use crate::model::{
    Callback, Event, Interface, LevelData, Order, Plant, Resource, SheetKind, SpriteType, State,
    Text, CANVAS_HEIGHT, CANVAS_HEIGHT_F64, CANVAS_WIDTH, CANVAS_WIDTH_F64, INTERFACE, PLANT,
};
use crate::scenes::{HomeScene, LevelScene};
use crate::sprites::{
    BaseUpdate, Guideline, Life, PlantSprite, Pos, Sprite, SpritePointer, Update, Zombie,
};
use crate::time_system::TimeSystem;
use crate::util::{set_sprite_clicked, window};

pub struct Game {
    time_system: TimeSystem,
    pub now: f64,
    fps: Fps,
    time_rate: f64,

    pub canvas: HtmlCanvasElement,
    pub context: CanvasRenderingContext2d,

    pub resource: Resource,

    pub leval_data: Vec<LevelData>,
    pub cur_level: LevelData,
    cur_level_index: usize,
    pub seed_pos: Vec<Pos>,
    pub card_pos: Vec<Pos>,
    pub card_cursor: usize,

    // TODO：使用 hashmap
    pub sprites: Vec<Box<dyn Update>>,
    temp_sprites: Vec<Box<dyn Update>>,

    enter: Rc<Cell<bool>>,
    pressed: Rc<Cell<bool>>,
    dragging: Rc<Cell<bool>>,
    cur_pos: Rc<Cell<Pos>>,

    draw_order: Vec<Order>,

    guideline: Guideline,

    pub state: State,

    gc_rate: f64,
    last_gc: f64,

    pub name: String,

    be_planted_id: String,

    sun_produce_rate: f64,
    last_sun_produce: f64,

    over: bool,
}

impl Game {
    pub fn new() -> Game {
        let document = window().document().unwrap();
        let canvas = Game::create_canvas(CANVAS_WIDTH, CANVAS_HEIGHT);
        let context = Game::get_canvas_context(&canvas);

        document.body().unwrap().append_child(&canvas).unwrap();

        Game {
            time_system: TimeSystem::new(),
            now: 0.0,
            fps: Fps::new(),
            time_rate: 1.0,

            canvas,
            context,

            resource: Resource::new(),

            leval_data: vec![],
            cur_level: LevelData::default(),
            cur_level_index: 0,
            seed_pos: vec![],
            card_pos: vec![],
            card_cursor: 0,

            sprites: vec![],
            temp_sprites: vec![],

            enter: Rc::new(Cell::new(false)),
            pressed: Rc::new(Cell::new(false)),
            dragging: Rc::new(Cell::new(false)),
            cur_pos: Rc::new(Cell::new(Pos::new(0.0, 0.0))),

            draw_order: vec![],

            guideline: Guideline::new(),

            state: State::new(),

            gc_rate: 5000.0,
            last_gc: 0.0,

            name: String::from("超萌超可爱"),

            be_planted_id: String::from(""),

            sun_produce_rate: 10000.0,
            last_sun_produce: 0.0,

            over: false,
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

        canvas
    }

    pub fn get_canvas_context(canvas: &HtmlCanvasElement) -> CanvasRenderingContext2d {
        canvas
            .get_context("2d")
            .unwrap()
            .unwrap()
            .dyn_into::<CanvasRenderingContext2d>()
            .unwrap()
    }

    pub fn init(&mut self) {
        self.time_system.start();

        self.goto_home_scene(None);
        // self.goto_level_scene(None);

        // self.debug_start();
    }

    fn debug_start(&mut self) {
        self.state.selected_card = true;
        self.show_seed_chooser(None);
        self.build_test_plant_card();
        self.build_test_plant();

        LevelScene::build_sunback(self);
        LevelScene::build_zombies(self);

        self.update_sunback_offset();
        self.start_battle(None);
    }

    fn build_test_plant(&mut self) {
        [
            "Peashooter",
            "SnowPea",
            "Peashooter",
            "SnowPea",
            "Peashooter",
        ]
        .iter()
        .enumerate()
        .for_each(|(index, plant)| {
            let loc = Loc::new(index, 4);
            let plant = LevelScene::build_plant(self, plant, Pos::new(0.0, 0.0));

            self.be_planted_id = plant.id();
            self.register_callback(plant, BehaviorType::Interval, Callback::PlantInterval);
            self.update_card_cursor();
            self.grow_plant(&loc, None);
        });

        self.build_test_torchwood();
    }

    fn build_test_torchwood(&mut self) {
        let mut torchwood = Sprite::from_data_one(&self.resource, PLANT, "Torchwood");
        let loc = Loc::new(0, 5);
        let rect = &torchwood.get_rect();
        let pos = Loc::put_on_cell_bottom(&loc, &rect.into());

        self.be_planted_id = torchwood.id();

        torchwood.update_loc(loc);
        torchwood.update_pos(pos);
        torchwood.start_all_behavior(self.now);

        self.add_sprite(torchwood);
        self.update_card_cursor();
    }

    fn build_test_plant_card(&mut self) {
        self.update_card_cursor();

        LevelScene::build_plant_card(self, "SunFlower".to_string(), self.get_card_pos());
        LevelScene::build_plant_card(self, "SnowPea".to_string(), self.get_card_pos());
        LevelScene::build_plant_card(self, "Peashooter".to_string(), self.get_card_pos());
    }

    fn update_card_cursor(&mut self) {
        self.card_cursor = self.sprites.len();
    }

    pub fn select_level(&mut self, index: usize) {
        let scale = 0.725;

        self.cur_level = self.leval_data[index].clone();
        self.cur_level_index = index;
        self.seed_pos = Loc::put_increase_x(
            30.0,
            115.0,
            scale * 100.0,
            self.cur_level.plant_cards.len(),
            5,
            scale * 60.0,
        );
        self.card_pos = Loc::put_increase_y(0.0, 0.0, 60.0, self.cur_level.plant_cards.len());
    }

    pub fn add_sprite(&mut self, sprite: Box<dyn Update>) {
        self.sprites.push(sprite);
        self.update_draw_order();
    }

    pub fn add_sprites(&mut self, sprites: Vec<Box<dyn Update>>) {
        self.sprites.extend(sprites);
        self.update_draw_order();
    }

    fn remove_sprite(&mut self, index: usize) {
        self.sprites.remove(index);
        self.update_draw_order();
    }

    fn remove_sprite_by_indexs(&mut self, indexs: &[usize]) {
        indexs.iter().for_each(|index| {
            self.sprites.remove(*index);
        });

        self.update_draw_order();
    }

    fn clear_sprites(&mut self) {
        self.sprites.clear();
        self.draw_order.clear();
        self.context
            .clear_rect(0.0, 0.0, CANVAS_WIDTH_F64, CANVAS_HEIGHT_F64);
    }

    fn update_draw_order(&mut self) {
        self.draw_order = self
            .sprites
            .iter()
            .enumerate()
            .map(|(index, sprite)| Order(index, sprite.get_order()))
            .collect();

        self.draw_order.sort();
    }

    pub fn register_callback(
        &mut self,
        mut sprite: Box<dyn Update>,
        behavior_type: BehaviorType,
        callback: Callback,
    ) {
        if let Some(behavior) = sprite.find_behavior(behavior_type) {
            let pointer = self.get_callback(callback);

            behavior.add_callback(pointer);

            BehaviorFactory::whether_to_enable(behavior, self.now);
        }

        self.add_sprite(sprite);
    }

    pub fn register_callbacks(
        &mut self,
        mut sprite: Box<dyn Update>,
        behavior_types: Vec<BehaviorType>,
        callbacks: Vec<Callback>,
    ) {
        behavior_types
            .iter()
            .enumerate()
            .for_each(|(index, behavior_type)| {
                if let Some(behavior) = sprite.find_behavior(*behavior_type) {
                    let pointer = self.get_callback(callbacks[index]);

                    behavior.add_callback(pointer);

                    BehaviorFactory::whether_to_enable(behavior, self.now);
                }
            });

        self.add_sprite(sprite);
    }

    pub fn get_callback(&mut self, callback: Callback) -> ErasedFnPointer<SpritePointer> {
        match callback {
            Callback::HomeButton => ErasedFnPointer::from_associated(self, Game::show_zombie_hand),
            Callback::ZombieHand => ErasedFnPointer::from_associated(self, Game::goto_level_scene),
            Callback::BgScroll => ErasedFnPointer::from_associated(self, Game::show_seed_chooser),
            Callback::Prepare => ErasedFnPointer::from_associated(self, Game::start_battle),
            Callback::SeedClick => ErasedFnPointer::from_associated(self, Game::choose_seed),
            Callback::CardClick => ErasedFnPointer::from_associated(self, Game::grow_or_remove_card),
            Callback::ResetButton => ErasedFnPointer::from_associated(self, Game::reset_cards),
            Callback::OkButton => ErasedFnPointer::from_associated(self, Game::bg_turn_left),
            Callback::BackButton => ErasedFnPointer::from_associated(self, Game::goto_home_scene),
            Callback::PlantInterval => ErasedFnPointer::from_associated(self, Game::plant_action),
            Callback::SunClick => ErasedFnPointer::from_associated(self, Game::collect_sun),
            Callback::SunInterval => ErasedFnPointer::from_associated(self, Game::sun_disappear),
        }
    }

    fn collect_sun(&mut self, sun: SpritePointer) {
        if let Some(mut sun) = sun {
            unsafe {
                self.state.sun += 150;

                sun.as_mut().hide()
            }
        }
    }

    fn sun_disappear(&mut self, sun: SpritePointer) {
        if let Some(mut sun) = sun {
            unsafe {
                sun.as_mut().hide();
            }
        }
    }

    fn goto_home_scene(&mut self, _back_button: SpritePointer) {
        self.state.selected_card = false;
        self.clear_sprites();

        HomeScene::build(self);
    }

    // TODO：移到 plant
    fn plant_action(&mut self, plant: SpritePointer) {
        if let Some(mut plant) = plant {
            unsafe {
                let plant = plant
                    .as_mut()
                    .as_any()
                    .downcast_mut::<PlantSprite>()
                    .unwrap();
                let name = plant.name();

                match name {
                    SpriteType::Plant(Plant::SunFlower) => {
                        let sun_pos = plant.get_sun_pos();

                        LevelScene::build_sun(Some(sun_pos), self);
                    }
                    SpriteType::Plant(Plant::Peashooter) | SpriteType::Plant(Plant::SnowPea) => {
                        let loc = plant.get_loc();

                        if self.row_has_zombie(&loc) {
                            let bullet_pos = plant.get_bullet_pos();

                            LevelScene::build_bullet(name, bullet_pos, self);
                        }
                    }
                    _ => (),
                }
            }
        }
    }

    // TODO：有优化空间，不用每个 plant 都进行判断
    fn row_has_zombie(&mut self, loc: &Loc) -> bool {
        self.sprites
            .iter_mut()
            .any(|zombie| match SpriteType::is_zombie(zombie.name()) {
                true if loc.in_same_row(&zombie.get_loc()) => {
                    !zombie.as_any().downcast_mut::<Zombie>().unwrap().is_die()
                }
                _ => false,
            })
    }

    #[inline]
    fn get_card_pos(&self) -> Pos {
        let index = self.sprites.len() - self.card_cursor;

        self.card_pos[index]
    }

    /// 种子对话框点击种子回调 在最左边生成对应的植物卡片
    fn choose_seed(&mut self, seed: SpritePointer) {
        if let Some(mut seed) = seed {
            unsafe {
                let name = seed.as_ref().name().short_name();
                let artist = seed.as_mut().get_mut_artist();

                if !LevelScene::is_seed_disabled(artist) {
                    LevelScene::build_plant_card(self, name, self.get_card_pos());
                }
            }
        }
    }

    /// 植物卡片点击回调 取消选择该植物
    fn grow_or_remove_card(&mut self, card: SpritePointer) {
        if let Some(mut card) = card {
            unsafe {
                let name = card.as_ref().name();

                match self.state.selected_card {
                    true => {
                        let plant_name = name.short_name();
                        let pos = card.as_ref().get_pos();
                        let plant = LevelScene::build_plant(self, &plant_name, pos);

                        self.temp_sprites.push(plant);

                        card.as_mut().set_clicked(false);
                    }
                    false => {
                        let remove_index = self
                            .sprites
                            .iter()
                            .skip(self.card_cursor)
                            .position(|sprite| sprite.name() == name);

                        if let Some(remove_index) = remove_index {
                            self.remove_sprite(remove_index + self.card_cursor);
                            self.update_card_pos();
                            self.enable_seed(name);
                        }
                    }
                }
            }
        }
    }

    fn no_card_select(&self) -> bool {
        self.card_cursor == self.sprites.len()
    }

    /// 种子选择对话框重置按钮回调 取消所有已选择种子
    fn reset_cards(&mut self, _reset_button: SpritePointer) {
        if self.no_card_select() {
            return;
        }

        self.cur_level
            .plant_cards
            .to_vec()
            .iter()
            .for_each(|card| self.enable_seed(SpriteType::from_str(card)));

        let indexs: Vec<usize> = (self.card_cursor..self.sprites.len())
            .into_iter()
            .rev()
            .collect();

        self.remove_sprite_by_indexs(&indexs);
    }

    fn enable_seed(&mut self, name: SpriteType) {
        let seed = self.find_sprite(name);

        if let Some(seed) = seed {
            seed.get_mut_artist().goto(0);
        }
    }

    /// 取消选择该植物后需要更新其余植物卡片位置
    fn update_card_pos(&mut self) {
        self.sprites
            .iter_mut()
            .skip(self.card_cursor)
            .enumerate()
            .for_each(|(index, card)| {
                let pos = self.card_pos[index];

                card.update_pos(pos);
                card.update_outlines();
            });
    }

    /// 首页按钮回调 显示僵尸手
    fn show_zombie_hand(&mut self, _home_button: SpritePointer) {
        HomeScene::build_hand(self);
    }

    /// 僵尸手动画结束回调 转到关卡场景
    fn goto_level_scene(&mut self, _zombie_hand: SpritePointer) {
        self.clear_sprites();
        self.reset_last_sun_produce();

        LevelScene::build_background(self);
    }

    fn find_sprite(&mut self, sprite_type: SpriteType) -> Option<&mut Box<dyn Update>> {
        return self
            .sprites
            .iter_mut()
            .find(|sprite| sprite.name() == sprite_type);
    }

    // TODO：优化 使用删除后重新创建而不是查找
    fn update_sunback_offset(&mut self) {
        if let Some(sun_back) = self.find_sprite(SpriteType::Interface(Interface::SunBack)) {
            sun_back.update_offset(Pos::new(0.0, 0.0));
        }
    }

    /// 关卡背景滚动到右边/左边回调 显示植物种子选择对话框/绘制关卡其它场景
    fn show_seed_chooser(&mut self, _background: SpritePointer) {
        match self.state.selected_card {
            true => LevelScene::build(self),
            false => {
                LevelScene::build_seed_chooser(self);

                self.update_card_cursor();
            }
        }
    }

    /// 根据类别删除 sprite
    fn remove_sprite_by_types(&mut self, sprite_types: &[SpriteType]) {
        sprite_types.iter().for_each(|sprite_type| {
            let index = self
                .sprites
                .iter()
                .position(|sprite| sprite.name() == *sprite_type);

            if let Some(index) = index {
                self.sprites.remove(index);
            }
        });

        self.update_draw_order();
    }

    /// 种子选择对话框确定按钮回调
    fn bg_turn_left(&mut self, _ok_button: SpritePointer) {
        if self.no_card_select() {
            return;
        }

        self.state.finished_select_card();

        let mut remove_sprites = vec![
            SpriteType::Interface(Interface::SeedChooserBackground),
            SpriteType::Interface(Interface::SelectCardButton),
            SpriteType::Interface(Interface::SelectCardButton),
            SpriteType::Text(Text::Reset),
            SpriteType::Text(Text::Start),
            SpriteType::Text(Text::SeedTitle),
            SpriteType::Text(Text::SunNum),
        ];

        remove_sprites.extend(
            self.cur_level
                .plant_cards
                .iter()
                .map(|plant_card| SpriteType::from_str(plant_card)),
        );

        self.remove_sprite_by_types(&remove_sprites);
        self.update_sunback_offset();
        self.scroll_bg_to_left();
    }

    fn scroll_bg_to_left(&mut self) {
        let now = self.now;
        let bg = self.find_sprite(SpriteType::Interface(Interface::Background1));

        if let Some(bg) = bg {
            bg.toggle_behavior(BehaviorType::Scroll, true, now);
        }
    }

    /// 准备 安放 植物 动画结束回调 开始游戏
    fn start_battle(&mut self, _prepare: SpritePointer) {
        unsafe { log!("开始") }

        self.reset_last_sun_produce();
        self.sprites
            .iter_mut()
            .filter(|sprite| SpriteType::is_zombie(sprite.name()))
            .for_each(|zombie| {
                let zombie = zombie.as_any().downcast_mut::<Zombie>().unwrap();

                zombie.change_to_walk(self.now);
                zombie.start_all_behavior(self.now);
                zombie.toggle_behavior(BehaviorType::Interval, false, self.now);
            });
    }

    pub fn toggle_behaviors(&mut self, behavior_types: &[BehaviorType], run: bool) {
        self.sprites.iter_mut().for_each(|sprite| {
            sprite
                .get_mut_behaviors()
                .iter_mut()
                .filter(|behavior| behavior_types.contains(&behavior.name()))
                .for_each(|behavior| behavior.toggle(run, self.now));
        });
    }

    fn is_dragging(&self) -> bool {
        let pressed = self.pressed.clone();
        let dragging = self.dragging.clone();

        self.state.selected_card && pressed.get() && dragging.get()
    }

    fn sprite_had_moved(&mut self, sheet_kind: SheetKind, name: &str) -> bool {
        let mut moved = false;
        let (cell_name, ..) = Resource::get_name(sheet_kind, name);
        let sprite_data = self.resource.get_data(&cell_name);

        let sprite = self.find_sprite(SpriteType::from_str(name));

        if let Some(sprite) = sprite {
            let original_pos = sprite_data.pos[0];
            let pos = sprite.get_pos();

            moved = pos != original_pos;
            sprite.update_pos(original_pos);
        }

        moved
    }

    fn shovel_plant(&mut self, loc: &Loc, has_plant: Option<usize>) {
        if let (false, Some(index)) = (loc.out_of_plant_bound(), has_plant) {
            self.remove_sprite(index);
            self.toggle_behaviors(&[BehaviorType::ZombieCollision], true);
        }
    }

    fn loc_has_plant(&self, loc: Loc) -> Option<usize> {
        self.sprites
            .iter()
            .position(|sprite| SpriteType::is_plant(sprite.name()) && loc == sprite.get_loc())
    }

    pub fn dispatch_event(&mut self, name: Event, event: MouseEvent) {
        let pressed = self.pressed.clone();
        let dragging = self.dragging.clone();
        let cur_pos = self.cur_pos.clone();
        let x = event.offset_x() as f64;
        let y = event.offset_y() as f64;

        cur_pos.set(Pos::new(x, y));

        // unsafe { log!("{} : {}, {}", name, x, y) }

        match name {
            Event::Mousedown => {
                pressed.set(true);
                self.mousedonw_handler(x, y);
            }

            Event::Mouseup => {
                self.drag_end_handler();

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
                        if let Some(plant) = self.temp_sprites.pop() {
                            self.be_planted_id = plant.id();
                            self.register_callback(
                                plant,
                                BehaviorType::Interval,
                                Callback::PlantInterval,
                            );
                        }

                        self.toggle_behaviors(&[BehaviorType::Drag], true);
                    }

                    dragging.set(true);
                }

                self.mousemove_handler(x, y);
            }
        };
    }

    // TODO：铲掉火炬树有问题 fix
    pub fn find_sprite_by_id(&self, id: String) -> usize {
        let index = self
            .sprites
            .iter()
            .position(|sprite| sprite.id() == id)
            .unwrap();

        index
    }

    fn grow_plant(&mut self, loc: &Loc, has_plant: Option<usize>) {
        let plant_index = self.find_sprite_by_id(self.be_planted_id.clone());

        // 不满足种植条件就移除掉
        if loc.out_of_plant_bound() || has_plant.is_some() {
            self.remove_sprite(plant_index);

            return;
        }

        let plant = &mut self.sprites[plant_index];
        let rect = &plant.get_rect();
        let pos = Loc::put_on_cell_bottom(loc, &rect.into());

        plant.set_order(1);
        plant.update_loc(*loc);
        plant.update_pos(pos);
        // 最后一个是 DragBehavior 种植后已经不需要了
        plant.get_mut_behaviors().pop();
        plant.start_all_behavior(self.now);
    }

    fn drag_end_handler(&mut self) {
        if self.is_dragging() {
            let cur_pos_loc = Loc::get_row_col_by_pos(&self.cur_pos.get());
            let has_plant = self.loc_has_plant(cur_pos_loc);

            if self.sprite_had_moved(INTERFACE, "Shovel") {
                self.shovel_plant(&cur_pos_loc, has_plant);
            }

            if !self.be_planted_id.is_empty() {
                self.grow_plant(&cur_pos_loc, has_plant);
            }
        }
    }

    fn mouseenter_handler(&mut self, _x: f64, _y: f64) {}

    fn mousedonw_handler(&mut self, _x: f64, _y: f64) {
        self.toggle_behaviors(&[BehaviorType::Click], true);
    }

    fn mouseup_handler(&mut self, _x: f64, _y: f64) {
        self.be_planted_id = String::from("");
        self.temp_sprites.clear();
        self.toggle_behaviors(&[BehaviorType::Click, BehaviorType::Drag], false);

        set_sprite_clicked("");
    }

    fn mousemove_handler(&mut self, _x: f64, _y: f64) {
        self.toggle_behaviors(&[BehaviorType::Hover], true);
    }

    fn mouseleave_handler(&mut self, _x: f64, _y: f64) {
        self.toggle_behaviors(&[BehaviorType::Hover], false);
    }

    fn free_sun(&mut self) {
        self.reset_last_sun_produce();

        LevelScene::build_sun(None, self);
    }

    fn reset_last_sun_produce(&mut self) {
        self.last_sun_produce = self.now;
    }

    fn reset_gc(&mut self) {
        self.last_gc = self.now;
    }

    fn should_gen_free_sun(&self) -> bool {
        self.state.selected_card && self.now - self.last_sun_produce > self.sun_produce_rate
    }

    fn should_gc(&self) -> bool {
        self.now - self.last_gc > self.gc_rate
    }

    pub fn run(&mut self) {
        self.now = self.time_system.calculate_game_time();
        self.fps.calc(self.now, self.time_rate);
        self.update();
        self.fps.update(self.now);
    }

    fn update(&mut self) {
        self.update_spirte_behaviors();
        self.draw_sprites();

        if self.should_gc() {
            self.gc();
        }

        if self.should_gen_free_sun() {
            self.free_sun();
        }
    }

    fn update_spirte_behaviors(&mut self) {
        let cur_pos = self.cur_pos.clone();
        let cur_pos = &cur_pos.get();

        self.draw_order.iter().for_each(|order| {
            // TODO：debug 存在莫名的越界访问
            match self.sprites.get_mut(order.index()) {
                Some(sprite) if sprite.is_visible() => sprite.update(
                    self.now,
                    self.fps.last_animation_frame_time,
                    cur_pos,
                    &self.context,
                ),
                Some(_) => (),
                None => unsafe {
                    log!(
                        "update_spirte_behaviors 越界访问 sprites_len:{} draw_order_len:{} \
                         order.index():{}",
                        self.sprites.len(),
                        self.draw_order.len(),
                        order.index()
                    )
                },
            }
        });
    }

    fn draw_sprites(&self) {
        self.context
            .clear_rect(0.0, 0.0, CANVAS_WIDTH_F64, CANVAS_HEIGHT_F64);
        // TODO：debug 存在莫名的越界访问
        self.draw_order
            .iter()
            .for_each(|order| match self.sprites.get(order.index()) {
                Some(sprite) if sprite.is_visible() => sprite.draw(&self.context),
                Some(_) => (),
                None => unsafe {
                    log!(
                        "draw_sprites 越界访问 sprites_len:{} draw_order_len:{} order.index():{}",
                        self.sprites.len(),
                        self.draw_order.len(),
                        order.index()
                    )
                },
            });

        self.guideline.draw(&self.context);

        if self.state.selected_card {
            self.draw_sun_num();
        }
    }

    fn gc(&mut self) {
        let remove_indexs: Vec<usize> = self
            .sprites
            .iter()
            .enumerate()
            .filter_map(|(index, sprite)| match sprite.is_visible() {
                true => None,
                false => Some(index),
            })
            .rev()
            .collect();

        self.remove_sprite_by_indexs(&remove_indexs);
        self.reset_gc();

        unsafe { log!("删除不可见 sprite") }
    }

    fn update_sprites_offset(&mut self, offset: f64) {
        self.sprites
            .iter_mut()
            .filter(|sprite| SpriteType::is_zombie(sprite.name()))
            .for_each(|sprite| sprite.update_offset(Pos::new(offset, 0.0)));
    }

    pub fn format_sun_num(&self) -> String {
        match self.state.sun > 99999 {
            true => String::from("9999+"),
            false => self.state.sun.to_string(),
        }
    }

    fn draw_sun_num(&self) {
        let num = self.format_sun_num();

        self.context.save();

        self.context.set_font("32px 黑体");
        self.context.fill_text(&num, 138.0, 30.0).unwrap();

        self.context.restore();
    }

    pub fn game_over(&mut self) {
        self.over = true;
        self.toggle_behaviors(
            &[
                BehaviorType::Walk,
                BehaviorType::Switch,
                BehaviorType::ZombieCollision,
                BehaviorType::Interval,
            ],
            false,
        );

        LevelScene::build_zombies_won(self);
    }
}
