use crate::behaviors::BehaviorType;
use crate::game::Game;
use crate::model::{Callback, SpriteType, Text, INTERFACE};
use crate::sprites::{Sprite, TextSprite};

pub struct HomeScene;

impl HomeScene {
    pub fn build(game: &mut Game) {
        HomeScene::build_common(game);
        HomeScene::build_button(game);
    }

    /// 首页普通 sprite 对象
    pub fn build_common(game: &mut Game) {
        let scenes = vec![
            "SelectorBackground",
            "SelectorAdventureShadow",
            "SelectorSurvivalShadow",
            "SelectorChallengeShadow",
            "SelectorWoodSign1",
            "SelectorWoodSign2",
            "SelectorWoodSign3",
        ];

        let sprites = Sprite::from_names(scenes, &game.resource, INTERFACE);
        let mut cell = sprites[4].get_rect();

        cell.top += 30.0;

        let name = TextSprite::new(SpriteType::Text(Text::PlayerName), &game.name, 20.0, &cell);

        game.add_sprites(sprites);
        game.add_sprite(Box::new(name));
    }

    /// 僵尸手动画
    pub fn build_hand(game: &mut Game) {
        let hand = Sprite::from_data_one(&game.resource, INTERFACE, "SelectorZombieHand");

        game.register_callback(hand, BehaviorType::Frequency, Callback::ZombieHand);
    }

    /// 开始冒险模式 迷你游戏 益智游戏按钮
    pub fn build_button(game: &mut Game) {
        let buttons = vec![
            "SelectorAdventureButton",
            "SelectorSurvivalButton",
            "SelectorChallengeButton",
        ];

        buttons.iter().for_each(|button| {
            let mut button_sprite = Sprite::from_data_one(&game.resource, INTERFACE, button);

            button_sprite.update_outlines();

            game.register_callback(button_sprite, BehaviorType::Click, Callback::HomeButton);
        });
    }
}
