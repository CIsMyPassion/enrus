use std::{path::Path, error::Error};
use std::collections::HashMap;
use std::fmt;

use font_kit::{canvas::{Canvas, Format, RasterizationOptions}, hinting::HintingOptions};
use pathfinder_geometry::{transform2d::Transform2F, vector::{Vector2I, Vector2F}, rect::{RectI, RectF}};

use crate::texture::Texture;

const FONT_SIZE: u32 = 48;
const MAX_CANVAS_SIZE: u32 = 512;

pub struct Font {
    atlas: Texture,
    map: HashMap<char, RectF>,
}

#[derive(Debug)]
pub enum FontError {
    CanvasTooSmall(u32, u32),
    SomeOther
}

impl fmt::Display for FontError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            FontError::CanvasTooSmall(x, y) => write!(f, "Error: Font canvas is too small: width {}, height {}", x, y),
            FontError::SomeOther => write!(f, "Error: Some other error"),
        }
    }
}

impl Error for FontError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        None
    }
}

fn render_glyphs(font: font_kit::font::Font) -> (Canvas, HashMap<char, RectF>) {
    let mut temp_hashmap = HashMap::new();
    for character in (65 as u8)..(90 as u8) {
        let glyph_result = font.glyph_for_char(character as char);

        match glyph_result {
            Some(glyph_id) => {

                let bounds = font.raster_bounds(
                    glyph_id,
                    FONT_SIZE as f32,
                    Transform2F::from_translation(Vector2F::new(0.0, FONT_SIZE as f32)),
                    HintingOptions::None,
                    RasterizationOptions::GrayscaleAa
                ).unwrap();

                let offset = Vector2I::new(1, 1);
                let adapted = RectI::new(Vector2I::zero(), bounds.size() + offset);

                temp_hashmap.insert(character as char, adapted);
            },
            None => ()
        }

    }

    let (packed_map, size) = glyph_packing(temp_hashmap).unwrap();
    let mut canvas = Canvas::new(size, Format::A8);

    //handle error better pls
    let hashmap = rasterize_glyphs(&mut canvas, packed_map, font).unwrap();
    (canvas, hashmap)
}


fn glyph_packing(rectangles: HashMap<char, RectI>) -> Result<(HashMap<char, RectI>, Vector2I), FontError> {

    let mut packed_map = HashMap::new();
    let mut size = Vector2I::splat(32);

    let pairs = rectangles.iter();
    let mut list: Vec<_> = pairs.collect();

    list.sort_by(|(_a, a), (_b, b)| b.size().y().cmp(&a.size().y()));

    let mut pos = Vector2I::zero();
    let mut largest_h_this_row = 0;

    for (character, rect) in list {

        if pos.x() + rect.width() > MAX_CANVAS_SIZE as i32 {
            pos.set_y(pos.y() + largest_h_this_row);
            pos.set_x(0);
            largest_h_this_row = 0;
        }

        if pos.y() + rect.height() > MAX_CANVAS_SIZE as i32 {
            return Err(FontError::CanvasTooSmall(MAX_CANVAS_SIZE, MAX_CANVAS_SIZE));
        }

        let packed_rect = RectI::new(pos, rect.size());
        packed_map.insert(*character, packed_rect);

        pos.set_x(pos.x() + rect.width());

        if rect.height() > largest_h_this_row {
            largest_h_this_row = rect.height();
            size.set_y(pos.y() + largest_h_this_row);
        }

        if pos.x() > size.x() {
            size.set_x(pos.x());
        }

    }
    println!("size {:?}", size);

    Ok((packed_map, size))
}

fn rasterize_glyphs(canvas: &mut Canvas, packed_map: HashMap<char, RectI>, font: font_kit::font::Font) -> Result<HashMap<char, RectF>, FontError>{

    for (character, rect) in packed_map {
        let glyph_result = font.glyph_for_char(character as char);

        match glyph_result {
            Some(glyph_id) => {

                let vector = Vector2F::new(rect.origin().x() as f32, (rect.origin().y() + rect.size().y()) as f32);
                let transform = Transform2F::from_translation(vector);

                font.rasterize_glyph(
                    canvas,
                    glyph_id,
                    FONT_SIZE as f32,
                    transform,
                    HintingOptions::None,
                    RasterizationOptions::GrayscaleAa
                ).unwrap();
            },
            None => ()
        }
    }

    let uv_map = HashMap::new();
    Ok(uv_map)
}

impl Font {

    pub fn new(atlas: Texture, map: HashMap<char, RectF>) -> Self {
        Font { atlas, map }
    }

    pub fn load_standard() -> Self {

        let path = Path::new(r"Sauce Code Pro Medium Nerd Font Complete Mono.ttf");
        let font = font_kit::font::Font::from_path(path, 0).unwrap();
        let (canvas, hashmap) = render_glyphs(font);
        let texture = Texture::new(canvas.size, crate::texture::Format::Grayscale, canvas.pixels.clone());

        Font::new(texture, hashmap)
    }

    pub fn atlas(&self) -> &Texture { &self.atlas }
    pub fn map(&self) -> &HashMap<char, RectF> { &self.map }
}
