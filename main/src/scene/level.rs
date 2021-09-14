use crate::{
    builder::Builder,
    loc::Loc,
    model::LocInfo,
    sprites::{model::Pos, sprite_wrap::SpriteWrap},
    util::get_random_int,
};

pub struct LevelScene;

impl LevelScene {
    pub fn create(builder: &Builder, scenes: &[String]) -> Vec<SpriteWrap> {
        let scenes: Vec<&str> = scenes.iter().map(|s| s.as_ref()).collect();
        let sprites = builder.from_names("interface", scenes);

        return sprites;
    }

    pub fn flag(builder: &Builder, flag_num: usize) -> Vec<SpriteWrap> {
        let flag_step = 140.0 / flag_num as f64;
        let pos = Loc::put_increase_x(557.0, 705.0, flag_step, flag_num, flag_num, 18.0);
        let mut sprites = builder.from_names(
            "interface",
            vec![
                "FlagMeterEmpty",
                "FlagMeterParts1",
                "FlagMeterLevelProgress",
            ],
        );
        let flags = builder.from_name_and_pos("interface", "FlagMeterParts2", pos);

        sprites.extend(flags);

        return sprites;
    }
    pub fn seed_chooser(builder: &Builder) -> Vec<SpriteWrap> {
        let sprites = builder.from_names(
            "interface",
            vec!["SeedChooserBackground", "SunBack", "SelectCardButton"],
        );

        return sprites;
    }

    pub fn plant_card(builder: &Builder, card_names: &Vec<String>) -> Vec<SpriteWrap> {
        let card_names: Vec<&str> = card_names.iter().map(|s| s.as_ref()).collect();
        let plant_pos = Loc::put_increase_y(0.0, 0.0, 60.0, card_names.len())
            .into_iter()
            .map(|pos| vec![pos])
            .collect();

        return builder.from_names_and_poss("card", card_names, plant_pos);
    }

    pub fn plant_seed(builder: &Builder, card_names: &Vec<String>) -> Vec<SpriteWrap> {
        let scale = 0.725;
        let card_names: Vec<&str> = card_names.iter().map(|s| s.as_ref()).collect();
        let plant_pos = Loc::put_increase_x(
            30.0,
            115.0,
            scale * 100.0,
            card_names.len(),
            5,
            scale * 60.0,
        )
        .into_iter()
        .map(|pos| vec![pos])
        .collect();
        let plants = builder.from_names_and_poss("card", card_names, plant_pos);

        return plants;
    }

    pub fn plants(builder: &Builder, plants: &Vec<LocInfo>) -> Vec<SpriteWrap> {
        return builder.create_plants(plants, true);
    }

    pub fn zombies(builder: &Builder, zombies: &Vec<LocInfo>) -> Vec<SpriteWrap> {
        return builder.create_plants(zombies, false);
    }

    pub fn drop_sun(builder: &Builder, pos: Option<&Pos>) -> Vec<SpriteWrap> {
        let (pos, distance) = match pos {
            Some(pos) => (*pos, 50.0),
            None => {
                let left = get_random_int(100.0, 800.0);
                let distance = get_random_int(150.0, 550.0);
                let top = get_random_int(0.0, 75.0);

                (Pos::new(left, -top), distance)
            }
        };

        let mut sun_data = builder.get_data("interface/Sun");

        sun_data.pos[0] = pos;
        sun_data.behaviors[2].distance = distance;

        let sun = builder.create_sprite("interface", "Sun", Some(sun_data));

        sun
    }
}
