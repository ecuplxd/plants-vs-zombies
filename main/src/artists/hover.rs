use web_sys::CanvasRenderingContext2d;

use derives::Resource;

use crate::sprites::model::{CollisionMargin, Pos, SpriteCell};

use super::{
    artist::Artist,
    model::{Draw, Resource, Stroke},
};

#[derive(Resource)]
pub struct HoverArtist {
    artist: Artist,
    points: Vec<Pos>,
}

impl HoverArtist {
    pub fn new(artist: Artist, pos: &Pos) -> HoverArtist {
        let points = artist.get_image_outline_points(pos);

        HoverArtist { artist, points }
    }
}

impl Draw for HoverArtist {
    fn draw(
        &self,
        context: &CanvasRenderingContext2d,
        pos: &Pos,
        _collision_margin: &CollisionMargin,
    ) {
        let cell = self.artist.get_current_cell();

        if let Some(cell) = cell {
            self.artist.draw_image(context, pos, cell);
            self.outline(context, &self.points);
        }
    }
}

impl Stroke for HoverArtist {}
