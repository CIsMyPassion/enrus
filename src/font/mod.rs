use std::path::Path;

use font_kit::{canvas::{Canvas, Format, RasterizationOptions}, hinting::HintingOptions, font::Font};
use pathfinder_geometry::{transform2d::Transform2F, vector::{Vector2I, Vector2F}};

use crate::texture::Texture;
use crate::texture::saver::save;

pub fn test() {

    let path = Path::new(r"Sauce Code Pro Medium Nerd Font Complete Mono.ttf");
    let font = Font::from_path(path, 0).unwrap();

    let glyph_id = font.glyph_for_char('A').unwrap();
    let mut canvas = Canvas::new(Vector2I::splat(32), Format::A8);
    println!("Got glyph_id: {}", glyph_id);

    let bounds = font.raster_bounds(
        glyph_id,
        32.0,
        Transform2F::from_translation(Vector2F::new(0.0, 32.0)),
        HintingOptions::None,
        RasterizationOptions::GrayscaleAa
    ).unwrap();
    println!("Glyph size is: {:?}", bounds);

    font.rasterize_glyph(
        &mut canvas,
        glyph_id,
        32.0,
        Transform2F::from_translation(Vector2F::new(0.0, 32.0)),
        HintingOptions::None,
        RasterizationOptions::GrayscaleAa
    ).unwrap();
    println!("Rasterized");
    print_canvas(&canvas);

    let texture = Texture::new(Vector2I::splat(32), crate::texture::Format::Grayscale, canvas.pixels.clone());
    save(&texture, String::from("out.png"));
}

fn print_canvas(canvas: &Canvas) {

    for y in 0..canvas.size.y() {
        let mut line = String::new();
        for x in 0..canvas.size.x() {
            line.push(value_to_char(canvas.pixels[(x + y * canvas.size.y()) as usize]));
        }
        println!("{}", line);
    }
}

fn value_to_char(value: u8) -> char {
    if value < 63 { return ' ' }
    if value < 127 { return '\u{2591}' }
    if value < 191 { return '\u{2592}' }
    return '\u{2588}'
}
