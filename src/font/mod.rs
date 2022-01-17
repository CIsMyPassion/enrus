use std::path::Path;
use std::collections::HashMap;

use font_kit::{canvas::{Canvas, Format, RasterizationOptions}, hinting::HintingOptions};
use pathfinder_geometry::{transform2d::Transform2F, vector::{Vector2I, Vector2F}, rect::RectF};

use crate::texture::Texture;

pub struct Font {
    atlas: Texture,
    map: HashMap<char, RectF>,
}

impl Font {

    pub fn new(atlas: Texture, map: HashMap<char, RectF>) -> Self {
        Font { atlas, map }
    }

    pub fn load_standard() -> Self {

        let path = Path::new(r"Sauce Code Pro Medium Nerd Font Complete Mono.ttf");
        let font = font_kit::font::Font::from_path(path, 0).unwrap();

        let glyph_id = font.glyph_for_char('A').unwrap();
        let mut canvas = Canvas::new(Vector2I::splat(32), Format::A8);

        let bounds = font.raster_bounds(
            glyph_id,
            32.0,
            Transform2F::from_translation(Vector2F::new(0.0, 32.0)),
            HintingOptions::None,
            RasterizationOptions::GrayscaleAa
        ).unwrap();

        font.rasterize_glyph(
            &mut canvas,
            glyph_id,
            32.0,
            Transform2F::from_translation(Vector2F::new(0.0, 32.0)),
            HintingOptions::None,
            RasterizationOptions::GrayscaleAa
        ).unwrap();

        let texture = Texture::new(Vector2I::splat(32), crate::texture::Format::Grayscale, canvas.pixels.clone());
        let hashmap = HashMap::new();
        Font::new(texture, hashmap)
    }

    pub fn atlas(&self) -> &Texture { &self.atlas }
    pub fn map(&self) -> &HashMap<char, RectF> { &self.map }
}
