use crate::{builder::Builder, sprites::sprite_wrap::SpriteWrap};

pub struct HomeScene;

impl HomeScene {
    pub fn create(builder: &Builder) -> Vec<SpriteWrap> {
        let sprites = builder.from_names(
            "interface",
            vec![
                "SelectorBackground",
                "SelectorAdventureShadow",
                "SelectorSurvivalShadow",
                "SelectorChallengeShadow",
                "SelectorWoodSign1",
                "SelectorWoodSign2",
                "SelectorWoodSign3",
                "SelectorAdventureButton",
                "SelectorSurvivalButton",
                "SelectorChallengeButton",
                "SelectorZombieHand",
            ],
        );

        return sprites;
    }
}
