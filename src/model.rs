use std::cmp::Ordering;
use std::collections::HashMap;
use std::fmt;
use std::rc::Rc;

use serde::Deserialize;
use wasm_bindgen::JsValue;
use web_sys::HtmlImageElement;

use crate::sprites::{SpriteCell, SpriteData};

#[derive(Debug, Clone, Copy)]
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

pub const CANVAS_WIDTH: u32 = 900;

pub const CANVAS_HEIGHT: u32 = 600;

pub const CANVAS_WIDTH_F64: f64 = CANVAS_WIDTH as f64;

pub const CANVAS_HEIGHT_F64: f64 = CANVAS_HEIGHT as f64;

pub const ROW_Y_COORD: [f64; 6] = [75.0, 175.0, 270.0, 380.0, 470.0, 575.0];

pub const COL_X_COORD: [f64; 17] = [
    100.0, 140.0, 220.0, 295.0, 379.0, 460.0, 540.0, 625.0, 695.0, 775.0, 855.0, 935.0,
    // 定位僵尸初始位置
    1015.0, 1095.0, 1175.0, 1255.0, 1335.0,
];

#[derive(Debug, Clone, Deserialize)]
pub struct LevelData {
    pub name: String,
    pub scenes: Vec<String>,
    pub flag_num: usize,
    pub plant_cards: Vec<String>,
    pub zombie_cards: Vec<String>,
}

impl Default for LevelData {
    fn default() -> LevelData {
        LevelData {
            name: Default::default(),
            scenes: Default::default(),
            flag_num: 2,
            plant_cards: Default::default(),
            zombie_cards: Default::default(),
        }
    }
}

impl LevelData {
    pub fn new_from_json(json: &JsValue) -> Vec<LevelData> {
        json.into_serde().unwrap()
    }
}

pub struct Resource {
    pub sheets: HashMap<String, Rc<HtmlImageElement>>,
    pub cells: HashMap<String, Vec<SpriteCell>>,
    pub data: HashMap<String, SpriteData>,
}

impl Resource {
    pub fn new() -> Resource {
        Resource {
            sheets: HashMap::new(),
            cells: HashMap::new(),
            data: HashMap::new(),
        }
    }

    pub fn get_data(&self, name: &str) -> SpriteData {
        return match self.data.get(name) {
            Some(sprite_data) => (*sprite_data).clone(),
            None => SpriteData::new(vec![], vec![]),
        };
    }

    pub fn get_sheet(&self, name: &str) -> &Rc<HtmlImageElement> {
        self.sheets.get(name).unwrap()
    }

    pub fn get_cell(&self, name: &str) -> Vec<SpriteCell> {
        match self.cells.get(name) {
            Some(cell) => cell.to_vec(),
            None => vec![SpriteCell::default()],
        }
    }

    pub fn get_cells_may_not_exit(&self, name: &str) -> Option<&Vec<SpriteCell>> {
        self.cells.get(name)
    }

    pub fn get_name(sheet_kind: SheetKind, name: &str) -> (String, String) {
        let sheet_name = sheet_kind.to_string().to_lowercase();
        let cell_name = format!("{}/{}", sheet_name, name);
        let sheet_name = format!("assets/images/{}", sheet_name);

        (cell_name, sheet_name)
    }
}

#[derive(Debug, Clone, Copy)]
pub enum SheetKind {
    Text,
    Interface,
    Card,
    Plant,
    Zombie,
}

impl fmt::Display for SheetKind {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        fmt::Debug::fmt(self, f)
    }
}

pub const INTERFACE: SheetKind = SheetKind::Interface;
pub const CARD: SheetKind = SheetKind::Card;
pub const ZOMBIE: SheetKind = SheetKind::Zombie;
pub const PLANT: SheetKind = SheetKind::Plant;

#[derive(Debug, PartialEq, Eq, Ord)]
pub struct Order(pub usize, pub usize);

impl Order {
    #[inline]
    pub fn index(&self) -> usize {
        self.0
    }
}

impl PartialOrd for Order {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        match other.1 > self.1 {
            true => Some(Ordering::Less),
            false if other.0 > self.0 => Some(Ordering::Less),
            _ => Some(Ordering::Greater),
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub enum Callback {
    HomeButton,
    ZombieHand,
    BgScroll,
    Prepare,
    SeedClick,
    CardClick,
    ResetButton,
    OkButton,
    BackButton,
    PlantInterval,
    SunClick,
    SunInterval,
}

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
pub enum Text {
    Reset,
    Start,
    Back,
    SunNum,
    PlayerName,
    SeedTitle,
}

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
    LawnCleaner,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Zombie {
    BackupDancer,
    Balloon,
    Buckethead,
    Bungee,
    Catapult,
    Conehead,
    Dancing,
    Digger,
    DolphinRider,
    DuckyTube,
    Flag,
    Football,
    Gargantuar,
    Imp,
    Jackbox,
    Ladder,
    Newspaper,
    PoleVaulting,
    ScreenDoor,
    Snorkel,
    Yeti,
    Zombie,
    BobsledTeam,
    Jump,
    Zomboni,
    Zombie1,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum SpriteType {
    Text(Text),
    Interface(Interface),
    Plant(Plant),
    Zombie(Zombie),
    Unknown,
}

impl SpriteType {
    pub fn is_zombie(name: SpriteType) -> bool {
        matches!(name, SpriteType::Zombie(_))
    }

    pub fn is_plant(name: SpriteType) -> bool {
        matches!(name, SpriteType::Plant(_))
    }

    pub fn is_bullet(name: SpriteType) -> bool {
        matches!(
            name,
            SpriteType::Plant(Plant::PB00) | SpriteType::Plant(Plant::PB100)
        )
    }

    pub fn is_lawn_cleaner(name: SpriteType) -> bool {
        matches!(name, SpriteType::Plant(Plant::LawnCleaner))
    }

    pub fn from_str(name: &str) -> SpriteType {
        match name {
            // Text
            "Reset" => SpriteType::Text(Text::Reset),
            "Start" => SpriteType::Text(Text::Start),
            "Back" => SpriteType::Text(Text::Back),
            "SunNum" => SpriteType::Text(Text::SunNum),
            "PlayerName" => SpriteType::Text(Text::PlayerName),
            "SeedTitle" => SpriteType::Text(Text::SeedTitle),
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
            "LawnCleaner" => SpriteType::Plant(Plant::LawnCleaner),
            // Zombie
            "BackupDancer" => SpriteType::Zombie(Zombie::BackupDancer),
            "Balloon" => SpriteType::Zombie(Zombie::Balloon),
            "Buckethead" => SpriteType::Zombie(Zombie::Buckethead),
            "Bungee" => SpriteType::Zombie(Zombie::Bungee),
            "Catapult" => SpriteType::Zombie(Zombie::Catapult),
            "Conehead" => SpriteType::Zombie(Zombie::Conehead),
            "Dancing" => SpriteType::Zombie(Zombie::Dancing),
            "Digger" => SpriteType::Zombie(Zombie::Digger),
            "DolphinRider" => SpriteType::Zombie(Zombie::DolphinRider),
            "DuckyTube" => SpriteType::Zombie(Zombie::DuckyTube),
            "Flag" => SpriteType::Zombie(Zombie::Flag),
            "Football" => SpriteType::Zombie(Zombie::Football),
            "Gargantuar" => SpriteType::Zombie(Zombie::Gargantuar),
            "Imp" => SpriteType::Zombie(Zombie::Imp),
            "Jackbox" => SpriteType::Zombie(Zombie::Jackbox),
            "Ladder" => SpriteType::Zombie(Zombie::Ladder),
            "Newspaper" => SpriteType::Zombie(Zombie::Newspaper),
            "PoleVaulting" => SpriteType::Zombie(Zombie::PoleVaulting),
            "ScreenDoor" => SpriteType::Zombie(Zombie::ScreenDoor),
            "Snorkel" => SpriteType::Zombie(Zombie::Snorkel),
            "Yeti" => SpriteType::Zombie(Zombie::Yeti),
            "Zombie" => SpriteType::Zombie(Zombie::Zombie),
            "BobsledTeam" => SpriteType::Zombie(Zombie::BobsledTeam),
            "Jump" => SpriteType::Zombie(Zombie::Jump),
            "Zomboni" => SpriteType::Zombie(Zombie::Zomboni),
            "Zombie1" => SpriteType::Zombie(Zombie::Zombie1),
            _ => SpriteType::Unknown,
        }
    }

    #[inline]
    pub fn short_name(&self) -> String {
        let name = self.to_string();
        let begin = name.find('(').unwrap() + 1;
        let end = name.find(')').unwrap();

        String::from(&name[begin..end])
    }
}

impl fmt::Display for SpriteType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        fmt::Debug::fmt(self, f)
    }
}

#[derive(Debug, Default)]
pub struct State {
    pub selected_card: bool,
    pub sun: usize,
}

impl State {
    pub fn new() -> State {
        State {
            selected_card: false,
            sun: 150,
        }
    }

    pub fn finished_select_card(&mut self) {
        self.selected_card = true;
    }
}
