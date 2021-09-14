use crate::{
    artists::model::Draw,
    model::SpriteType,
    sprites::{
        model::{DrawInfo, Offset, Pos, Update},
        sprite::Sprite,
    },
};

pub struct PlantSprite {
    life: f64,
    attack: f64,
    sprite: Sprite,
    switched: bool,
    gen_sun: bool,
    shoot: bool,
}

impl PlantSprite {
    pub fn new(sprite: Sprite) -> PlantSprite {
        PlantSprite {
            life: 100.0,
            attack: 1.0,
            sprite,
            switched: false,
            gen_sun: false,
            shoot: false,
        }
    }

    pub fn get_sun_pos(sprite: &Box<dyn Update>) -> Pos {
        let draw_info = sprite.get_draw_info().unwrap();
        let pos = draw_info.pos;

        Pos::new(pos.left + 73.0, pos.top - 37.0)
    }

    pub fn get_bullet_pos(sprite: &Box<dyn Update>) -> (Pos, SpriteType) {
        let draw_info = sprite.get_draw_info().unwrap();
        let pos = draw_info.pos;
        let cell = sprite.get_read_artist().get_current_cell().unwrap();

        (
            Pos::new(pos.left + cell.width / 1.5, pos.top),
            sprite.name(),
        )
    }

    pub fn interval_callback(&mut self) {
        self.switched = true;
        self.shoot = true;
    }

    pub fn swtich_callback(&mut self) {
        self.switched = false;
        self.gen_sun = true;
    }
}

impl Update for PlantSprite {
    fn draw(&self, context: &web_sys::CanvasRenderingContext2d) {
        self.sprite.draw(context);
    }

    fn get_artist(&mut self) -> &mut dyn Draw {
        self.sprite.get_artist()
    }

    fn get_read_artist(&self) -> &dyn Draw {
        self.sprite.get_read_artist()
    }

    fn get_draw_info(&self) -> Option<&DrawInfo> {
        Some(&self.sprite.draw_info)
    }

    fn name(&self) -> SpriteType {
        self.sprite.name
    }

    fn update_draw_info(&mut self, pos: Option<Pos>, offset: Option<Offset>) {
        self.sprite.update_draw_info(pos, offset);
    }

    fn is_visible(&self) -> bool {
        self.sprite.draw_info.visible
    }

    fn toggle(&mut self) {
        self.sprite.draw_info.visible = !self.sprite.draw_info.visible;
    }

    fn get_order(&self) -> usize {
        self.sprite.draw_info.order
    }

    fn update_loc(&mut self, row: usize, col: usize) {
        self.sprite.update_loc(row, col);
    }

    fn get_loc(&self) -> (usize, usize) {
        self.sprite.get_loc()
    }

    fn is_clicked(&self) -> bool {
        self.sprite.clicked
    }

    fn set_clicked(&mut self, clicked: bool) {
        self.sprite.set_clicked(clicked);
    }

    fn tirgger_switch(&mut self) -> (bool, usize) {
        (self.switched, 0)
    }

    fn get_gen_sun_flag(&self) -> bool {
        self.gen_sun
    }

    fn set_gen_sun_flag(&mut self, gened: bool) {
        self.gen_sun = gened;
    }

    fn get_shoot(&self) -> bool {
        self.shoot
    }

    fn set_shoot(&mut self, shoot: bool) {
        self.shoot = shoot;
    }
}
