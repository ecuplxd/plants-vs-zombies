use crate::artists::Draw;
use crate::behaviors::{Behavior, BehaviorCallback, BehaviorType, DragBehavior, WalkBehavior};
use crate::game::Game;
use crate::loc::Loc;
use crate::model::{Callback, Plant, SpriteType, Text, CARD, INTERFACE, PLANT, ZOMBIE};
use crate::sprites::{BaseUpdate, Pos, Sprite, TextSprite, Update, Velocit, ZombieSprite};
use crate::util::get_random_int_inclusive;

pub struct LevelScene;

impl LevelScene {
    pub fn build(game: &mut Game) {
        // let background = Sprite::from_data_one(&game.resource, INTERFACE, "Background1");

        // game.add_sprite(background);

        LevelScene::build_common(game);
        LevelScene::build_flags(game);
        LevelScene::build_prepare(game);
    }

    /// 场景普通 sprite
    pub fn build_common(game: &mut Game) {
        LevelScene::build_shovel_back(game);
        LevelScene::build_shovel(game);
        LevelScene::build_lawn_cleaner(game);
    }

    pub fn build_shovel_back(game: &mut Game) {
        let shovel_back = Sprite::from_data_one(&game.resource, INTERFACE, "ShovelBack");

        game.add_sprite(shovel_back);
    }

    pub fn build_lawn_cleaner(game: &mut Game) {
        let mut lawn_cleaners = Sprite::from_data(&game.resource, INTERFACE, "LawnCleaner");

        lawn_cleaners
            .iter_mut()
            .enumerate()
            .for_each(|(index, lawn_cleaner)| lawn_cleaner.update_loc(Loc::new(index, 0)));

        game.add_sprites(lawn_cleaners);
    }

    pub fn build_shovel(game: &mut Game) {
        let mut shovel = Sprite::from_data_one(&game.resource, INTERFACE, "Shovel");

        shovel.update_outlines();

        game.add_sprite(shovel);
    }

    /// 场景背景
    pub fn build_background(game: &mut Game) {
        let background = Sprite::from_data_one(&game.resource, INTERFACE, "Background1");
        let mut back_button = Sprite::from_data_one(&game.resource, INTERFACE, "Button");
        let cell = back_button.get_rect();
        let back_text = TextSprite::new(SpriteType::Text(Text::Back), "返回", 32.0, &cell);

        back_button.update_outlines();

        game.register_callback(background, BehaviorType::Scroll, Callback::BgScroll);
        game.register_callback(back_button, BehaviorType::Click, Callback::BackButton);
        game.add_sprite(Box::new(back_text));

        LevelScene::build_zombies(game);
    }

    /// 准备 安放 植物
    pub fn build_prepare(game: &mut Game) {
        let prepare = Sprite::from_data_one(&game.resource, INTERFACE, "PrepareGrowPlants");

        game.register_callback(prepare, BehaviorType::Frequency, Callback::Prepare);
    }

    /// 游戏进度
    pub fn build_flags(game: &mut Game) {
        let flags = vec![
            "FlagMeterEmpty",
            "FlagMeterParts1",
            "FlagMeterLevelProgress",
        ];

        let mut flag_sprites = Sprite::from_names(flags, &game.resource, INTERFACE);
        let banners = LevelScene::build_flag_banner(game);

        flag_sprites.extend(banners);

        game.add_sprites(flag_sprites);
    }

    /// 进度旗帜
    pub fn build_flag_banner(game: &mut Game) -> Vec<Box<dyn Update>> {
        let flag_num = game.cur_level.flag_num;
        let flag_step = 140.0 / flag_num as f64;
        let pos = Loc::put_increase_x(557.0, 705.0, flag_step, flag_num, flag_num, 18.0);
        let banners: Vec<Box<dyn Update>> = pos
            .iter()
            .map(|item| {
                let mut flag = Sprite::from_data_one(&game.resource, INTERFACE, "FlagMeterParts2");

                flag.update_pos(*item);

                flag
            })
            .collect();

        banners
    }

    /// 种子选择对话框
    pub fn build_seed_chooser(game: &mut Game) {
        let seed_bg = Sprite::from_data_one(&game.resource, INTERFACE, "SeedChooserBackground");
        let mut cell = seed_bg.get_rect();

        cell.height = 34.0;

        let title = TextSprite::new(
            SpriteType::Text(Text::SeedTitle),
            "选择你的卡片",
            20.0,
            &cell,
        );

        game.add_sprite(Box::new(title));
        game.add_sprite(seed_bg);

        LevelScene::build_seed_shooser_button(game);
        LevelScene::build_plant_seed(game);
        LevelScene::build_sunback(game);
    }

    /// 阳光数量
    pub fn build_sunback(game: &mut Game) {
        let sun_back = Sprite::from_data_one(&game.resource, INTERFACE, "SunBack");
        let cell = sun_back.get_rect();
        let sun_num = game.format_sun_num();
        let mut sun_text = TextSprite::new(SpriteType::Text(Text::SunNum), &sun_num, 32.0, &cell);

        sun_text.pos = Pos::new(138.0, 560.0);

        game.add_sprite(sun_back);
        game.add_sprite(Box::new(sun_text));
    }

    /// 种子选择对话框按钮 重置/开始
    pub fn build_seed_shooser_button(game: &mut Game) {
        let mut buttons = Sprite::from_data(&game.resource, INTERFACE, "SelectCardButton");
        let mut reset_button = buttons.remove(0);
        let mut ok_button = buttons.remove(0);
        let reset_cell = reset_button.get_rect();
        let ok_cell = ok_button.get_rect();
        let reset_text = TextSprite::new(SpriteType::Text(Text::Reset), "重置", 24.0, &reset_cell);
        let ok_text = TextSprite::new(SpriteType::Text(Text::Start), "开始", 24.0, &ok_cell);

        reset_button.update_outlines();
        ok_button.update_outlines();

        game.register_callback(reset_button, BehaviorType::Click, Callback::ResetButton);
        game.register_callback(ok_button, BehaviorType::Click, Callback::OkButton);
        game.add_sprites(vec![Box::new(reset_text), Box::new(ok_text)]);
    }

    /// 植物种子
    pub fn build_plant_seed(game: &mut Game) {
        game.cur_level
            .plant_cards
            .to_vec()
            .iter()
            .enumerate()
            .for_each(|(index, card)| {
                let pos = game.seed_pos[index];
                let mut seed = Sprite::from_data_one(&game.resource, CARD, card);

                seed.update_pos(pos);
                seed.update_outlines();

                game.register_callback(seed, BehaviorType::Click, Callback::SeedClick);
            });
    }

    /// 左边植物卡片
    pub fn build_plant_card(game: &mut Game, name: String, pos: Pos) {
        let mut card = Sprite::from_data_one(&game.resource, CARD, &name);

        card.get_mut_artist().update_scale(1.0);
        card.update_loc(Loc::new(99, 99));
        card.update_pos(pos);
        card.update_outlines();

        game.register_callback(card, BehaviorType::Click, Callback::CardClick);
    }

    pub fn is_seed_disabled(artist: &mut dyn Draw) -> bool {
        let in_last_cell = artist.in_last_cell();

        if !in_last_cell {
            artist.goto(1);
        }

        in_last_cell
    }

    /// 僵尸
    pub fn build_zombies(game: &mut Game) {
        game.cur_level
            .zombie_cards
            .to_vec()
            .iter()
            .enumerate()
            .for_each(|(index, card)| {
                let mut zombie = Sprite::from_data_one(&game.resource, ZOMBIE, card);
                let zombie_sprite = zombie.as_any().downcast_mut::<ZombieSprite>().unwrap();

                zombie_sprite.init_pos(index);
                zombie_sprite
                    .find_behavior(BehaviorType::Collision)
                    .unwrap()
                    .set_game(game);
                zombie.toggle_behavior(BehaviorType::Cycle, true, game.now);

                game.add_sprite(zombie);
            });
    }

    /// 僵尸头
    pub fn build_zombie_head(game: &mut Game, pos: Pos) {
        let mut zombie_head = Sprite::from_data_one(&game.resource, ZOMBIE, "ZombieHead");
        let new_pos = pos + Pos::new(5.0, -15.0);

        zombie_head.update_pos(new_pos);
        zombie_head.start_all_behavior(game.now);

        game.add_sprite(zombie_head);
    }

    pub fn build_plant(game: &mut Game, name: &str, pos: Pos) -> Box<dyn Update> {
        let mut plant = Sprite::from_data_one(&game.resource, PLANT, name);
        let mut drag: Box<dyn Behavior> = Box::new(DragBehavior::new());

        drag.set_sprite(plant.as_mut());
        plant.add_behavior(drag);
        plant.update_pos(pos);
        plant.set_clicked(true);
        plant.set_order(99);

        plant
    }

    /// 子弹
    pub fn build_bullet(shooter: SpriteType, pos: Pos, game: &mut Game) {
        let bullet = match shooter {
            SpriteType::Plant(Plant::Peashooter) => Some("PB00"),
            SpriteType::Plant(Plant::SnowPea) => Some("PB100"),
            _ => None,
        };

        if let Some(bullet) = bullet {
            let loc = Loc::get_row_col_by_pos(&pos);
            let mut bullet = Sprite::from_data_one(&game.resource, PLANT, bullet);

            bullet.update_loc(loc);
            bullet.update_pos(pos);
            bullet.start_all_behavior(game.now);

            game.toggle_behaviors(&[BehaviorType::Collision], true);
            game.add_sprite(bullet);
        }
    }

    /// 阳光
    pub fn build_sun(pos: Option<Pos>, game: &mut Game) {
        let (pos, distance) = match pos {
            Some(pos) => (pos, 50.0),
            None => {
                let distance = get_random_int_inclusive(150.0, 550.0);

                (Pos::random_sun_pos(), distance)
            }
        };
        let mut sun = Sprite::from_data_one(&game.resource, INTERFACE, "Sun");
        let mut walk = Box::new(WalkBehavior::new(
            Velocit::new(0.0, 20.0),
            300.0,
            Some(distance),
        ));
        let id = sun.id();

        walk.set_sprite(sun.as_mut());
        sun.add_behavior(walk);

        sun.update_pos(pos);
        sun.update_outlines();

        game.register_callbacks(
            sun,
            vec![BehaviorType::Click, BehaviorType::Interval],
            vec![Callback::SunClick, Callback::SunInterval],
        );

        let index = game.find_sprite_by_id(id);

        game.sprites[index].start_all_behavior(game.now);
    }

    /// 僵尸吃了你的脑子
    pub fn build_zombies_won(game: &mut Game) {
        let zombies_won = Sprite::from_data_one(&game.resource, INTERFACE, "ZombiesWon");

        game.add_sprite(zombies_won);
    }
}
