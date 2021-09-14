use std::fmt;

use serde::Deserialize;
use wasm_bindgen::JsValue;

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
pub enum Interface {
    SelectorBackground,
    SelectorAdventureShadow,
    SelectorAdventureButton,
    SelectorSurvivalShadow,
    SelectorSurvivalButton,
    SelectorChallengeShadow,
    SelectorChallengeButton,
    SelectorZombieHand,
    Sun,
    SelectorWoodSign1,
    SelectorWoodSign2,
    SelectorWoodSign3,
    Background1,
    Button,
    ShovelBack,
    Shovel,
    LawnCleaner,
    PrepareGrowPlants,
    SeedChooserBackground,
    SunBack,
    FlagMeterEmpty,
    FlagMeterFull,
    FlagMeterParts1,
    FlagMeterParts2,
    FlagMeterLevelProgress,
    SelectCardButton,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Plant {
    Blover,
    BoomWallNut,
    Cabbagepult,
    Cactus,
    Catttail,
    CherryBomb,
    Chomper,
    CobCannon,
    CoffeeBean,
    DoomShroom,
    FlowerPot,
    FumeShroom,
    Garlic,
    GatlingPea,
    GloomShroom,
    GoldMagnet,
    GraveBuster,
    HugeWallNut,
    HypnoShroom,
    IceShroom,
    Imitators,
    Jalapeno,
    Kernelpult,
    LilyPad,
    MagnetShroom,
    Marigold,
    Melonpult,
    Peashooter,
    Plantern,
    PotatoMine,
    PuffShroom,
    PumpkinHead,
    Repeater,
    ScaredyShroom,
    SeaShroom,
    SnowPea,
    Spikerock,
    Spikeweed,
    SplitPea,
    Squash,
    Starfruit,
    SunFlower,
    SunFlower1,
    SunShroom,
    TallNut,
    TangleKlep,
    Threepeater,
    Torchwood,
    TwinSunflower,
    UmbrellaLeaf,
    WallNut,
    WinterMelonpult,
    PB100,
    PB00,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Zombie {
    BackupDancer,
    Balloonzombie,
    BucketheadZombie,
    Bungeezombie,
    Catapult,
    ConeheadZombie,
    DancingZombie,
    Diggerzombie,
    DolphinRiderZombie,
    Dr,
    DuckyTubeZombie1,
    FlagZombie,
    FootballZombie,
    Gargantuar,
    Imp,
    JackboxZombie,
    LadderZombie,
    NewspaperZombie,
    PoleVaultingZombie,
    ScreenDoorZombie,
    SnorkelZombie,
    Yeti,
    Zombie,
    ZombieBobsledTeam,
    ZombiesJump,
    Zomboni,
    Zombie1,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum SpriteType {
    Interface(Interface),
    Plant(Plant),
    Zombie(Zombie),
    Nil,
}

impl SpriteType {
    pub fn from_str(name: &str) -> SpriteType {
        return match name {
            // Interface
            "SelectorAdventureButton" => SpriteType::Interface(Interface::SelectorAdventureButton),
            "SelectorBackground" => SpriteType::Interface(Interface::SelectorBackground),
            "SelectorAdventureShadow" => SpriteType::Interface(Interface::SelectorAdventureShadow),
            "SelectorSurvivalShadow" => SpriteType::Interface(Interface::SelectorSurvivalShadow),
            "SelectorSurvivalButton" => SpriteType::Interface(Interface::SelectorSurvivalButton),
            "SelectorChallengeShadow" => SpriteType::Interface(Interface::SelectorChallengeShadow),
            "SelectorChallengeButton" => SpriteType::Interface(Interface::SelectorChallengeButton),
            "SelectorZombieHand" => SpriteType::Interface(Interface::SelectorZombieHand),
            "Sun" => SpriteType::Interface(Interface::Sun),
            "SelectorWoodSign1" => SpriteType::Interface(Interface::SelectorWoodSign1),
            "SelectorWoodSign2" => SpriteType::Interface(Interface::SelectorWoodSign2),
            "SelectorWoodSign3" => SpriteType::Interface(Interface::SelectorWoodSign3),
            "Background1" => SpriteType::Interface(Interface::Background1),
            "Button" => SpriteType::Interface(Interface::Button),
            "ShovelBack" => SpriteType::Interface(Interface::ShovelBack),
            "Shovel" => SpriteType::Interface(Interface::Shovel),
            "LawnCleaner" => SpriteType::Interface(Interface::LawnCleaner),
            "PrepareGrowPlants" => SpriteType::Interface(Interface::PrepareGrowPlants),
            "SeedChooserBackground" => SpriteType::Interface(Interface::SeedChooserBackground),
            "SunBack" => SpriteType::Interface(Interface::SunBack),
            "FlagMeterEmpty" => SpriteType::Interface(Interface::FlagMeterEmpty),
            "FlagMeterFull" => SpriteType::Interface(Interface::FlagMeterFull),
            "FlagMeterParts1" => SpriteType::Interface(Interface::FlagMeterParts1),
            "FlagMeterParts2" => SpriteType::Interface(Interface::FlagMeterParts2),
            "FlagMeterLevelProgress" => SpriteType::Interface(Interface::FlagMeterLevelProgress),
            "SelectCardButton" => SpriteType::Interface(Interface::SelectCardButton),
            // Plant
            "Blover" => SpriteType::Plant(Plant::Blover),
            "BoomWallNut" => SpriteType::Plant(Plant::BoomWallNut),
            "Cabbagepult" => SpriteType::Plant(Plant::Cabbagepult),
            "Cactus" => SpriteType::Plant(Plant::Cactus),
            "Catttail" => SpriteType::Plant(Plant::Catttail),
            "CherryBomb" => SpriteType::Plant(Plant::CherryBomb),
            "Chomper" => SpriteType::Plant(Plant::Chomper),
            "CobCannon" => SpriteType::Plant(Plant::CobCannon),
            "CoffeeBean" => SpriteType::Plant(Plant::CoffeeBean),
            "DoomShroom" => SpriteType::Plant(Plant::DoomShroom),
            "FlowerPot" => SpriteType::Plant(Plant::FlowerPot),
            "FumeShroom" => SpriteType::Plant(Plant::FumeShroom),
            "Garlic" => SpriteType::Plant(Plant::Garlic),
            "GatlingPea" => SpriteType::Plant(Plant::GatlingPea),
            "GloomShroom" => SpriteType::Plant(Plant::GloomShroom),
            "GoldMagnet" => SpriteType::Plant(Plant::GoldMagnet),
            "GraveBuster" => SpriteType::Plant(Plant::GraveBuster),
            "HugeWallNut" => SpriteType::Plant(Plant::HugeWallNut),
            "HypnoShroom" => SpriteType::Plant(Plant::HypnoShroom),
            "IceShroom" => SpriteType::Plant(Plant::IceShroom),
            "Imitators" => SpriteType::Plant(Plant::Imitators),
            "Jalapeno" => SpriteType::Plant(Plant::Jalapeno),
            "Kernelpult" => SpriteType::Plant(Plant::Kernelpult),
            "LilyPad" => SpriteType::Plant(Plant::LilyPad),
            "MagnetShroom" => SpriteType::Plant(Plant::MagnetShroom),
            "Marigold" => SpriteType::Plant(Plant::Marigold),
            "Melonpult" => SpriteType::Plant(Plant::Melonpult),
            "Peashooter" => SpriteType::Plant(Plant::Peashooter),
            "Plantern" => SpriteType::Plant(Plant::Plantern),
            "PotatoMine" => SpriteType::Plant(Plant::PotatoMine),
            "PuffShroom" => SpriteType::Plant(Plant::PuffShroom),
            "PumpkinHead" => SpriteType::Plant(Plant::PumpkinHead),
            "Repeater" => SpriteType::Plant(Plant::Repeater),
            "ScaredyShroom" => SpriteType::Plant(Plant::ScaredyShroom),
            "SeaShroom" => SpriteType::Plant(Plant::SeaShroom),
            "SnowPea" => SpriteType::Plant(Plant::SnowPea),
            "Spikerock" => SpriteType::Plant(Plant::Spikerock),
            "Spikeweed" => SpriteType::Plant(Plant::Spikeweed),
            "SplitPea" => SpriteType::Plant(Plant::SplitPea),
            "Squash" => SpriteType::Plant(Plant::Squash),
            "Starfruit" => SpriteType::Plant(Plant::Starfruit),
            "SunFlower" => SpriteType::Plant(Plant::SunFlower),
            "SunFlower1" => SpriteType::Plant(Plant::SunFlower1),
            "SunShroom" => SpriteType::Plant(Plant::SunShroom),
            "TallNut" => SpriteType::Plant(Plant::TallNut),
            "TangleKlep" => SpriteType::Plant(Plant::TangleKlep),
            "Threepeater" => SpriteType::Plant(Plant::Threepeater),
            "Torchwood" => SpriteType::Plant(Plant::Torchwood),
            "TwinSunflower" => SpriteType::Plant(Plant::TwinSunflower),
            "UmbrellaLeaf" => SpriteType::Plant(Plant::UmbrellaLeaf),
            "WallNut" => SpriteType::Plant(Plant::WallNut),
            "WinterMelonpult" => SpriteType::Plant(Plant::WinterMelonpult),
            "PB100" => SpriteType::Plant(Plant::PB100),
            "PB00" => SpriteType::Plant(Plant::PB00),
            // Zombie
            "BackupDancer" => SpriteType::Zombie(Zombie::BackupDancer),
            "Balloonzombie" => SpriteType::Zombie(Zombie::Balloonzombie),
            "BucketheadZombie" => SpriteType::Zombie(Zombie::BucketheadZombie),
            "Bungeezombie" => SpriteType::Zombie(Zombie::Bungeezombie),
            "Catapult" => SpriteType::Zombie(Zombie::Catapult),
            "ConeheadZombie" => SpriteType::Zombie(Zombie::ConeheadZombie),
            "DancingZombie" => SpriteType::Zombie(Zombie::DancingZombie),
            "Diggerzombie" => SpriteType::Zombie(Zombie::Diggerzombie),
            "DolphinRiderZombie" => SpriteType::Zombie(Zombie::DolphinRiderZombie),
            "Dr" => SpriteType::Zombie(Zombie::Dr),
            "DuckyTubeZombie1" => SpriteType::Zombie(Zombie::DuckyTubeZombie1),
            "FlagZombie" => SpriteType::Zombie(Zombie::FlagZombie),
            "FootballZombie" => SpriteType::Zombie(Zombie::FootballZombie),
            "Gargantuar" => SpriteType::Zombie(Zombie::Gargantuar),
            "Imp" => SpriteType::Zombie(Zombie::Imp),
            "JackboxZombie" => SpriteType::Zombie(Zombie::JackboxZombie),
            "LadderZombie" => SpriteType::Zombie(Zombie::LadderZombie),
            "NewspaperZombie" => SpriteType::Zombie(Zombie::NewspaperZombie),
            "PoleVaultingZombie" => SpriteType::Zombie(Zombie::PoleVaultingZombie),
            "ScreenDoorZombie" => SpriteType::Zombie(Zombie::ScreenDoorZombie),
            "SnorkelZombie" => SpriteType::Zombie(Zombie::SnorkelZombie),
            "Yeti" => SpriteType::Zombie(Zombie::Yeti),
            "Zombie" => SpriteType::Zombie(Zombie::Zombie),
            "ZombieBobsledTeam" => SpriteType::Zombie(Zombie::ZombieBobsledTeam),
            "ZombiesJump" => SpriteType::Zombie(Zombie::ZombiesJump),
            "Zomboni" => SpriteType::Zombie(Zombie::Zomboni),
            "Zombie1" => SpriteType::Zombie(Zombie::Zombie1),
            _ => SpriteType::Nil,
        };
    }
}

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum Event {
    Mousedown,
    Mousemove,
    Mouseup,
    Mouseleave,
    Mouseenter,
}

impl fmt::Display for Event {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        fmt::Debug::fmt(self, f)
    }
}

pub type LocInfo = (String, usize, usize);

#[derive(Debug, Default, Clone, Deserialize)]
pub struct LevelData {
    pub name: String,
    pub scenes: Vec<String>,
    pub flag_num: usize,
    pub plant_cards: Vec<String>,
    pub plants: Vec<LocInfo>,
    pub zombies: Vec<LocInfo>,
}

impl LevelData {
    pub fn new_default() -> LevelData {
        LevelData {
            flag_num: 2,
            ..Default::default()
        }
    }

    pub fn new_from_json(json: &JsValue) -> Vec<LevelData> {
        json.into_serde().unwrap()
    }
}

#[derive(Debug, Default)]
pub struct State {
    pub home_ready: bool,
    pub preparing: bool,
    pub over: bool,
    pub paused: bool,
    pub in_home_scene: bool,
    pub selected_card: bool,
    pub max_sun: usize,
    pub cur_sun: usize,
}

impl State {
    pub fn new() -> State {
        State {
            in_home_scene: true,
            max_sun: 10,
            ..Default::default()
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub enum Callback {
    ShowReady,
    TurnToLevelPrepareScene,
    ChoosePlantCard,
    BattleSetup,
    ResetCardSelect,
    SelectPlantSeed,
    StartBattle,
    CollectSun,
}
