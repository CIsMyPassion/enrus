use font_kit::{source::SystemSource, canvas::{Canvas, Format, RasterizationOptions}, hinting::HintingOptions};
use pathfinder_geometry::{transform2d::Transform2F, vector::{Vector2I, Vector2F}};

pub fn test() {

    let font = SystemSource::new()
        .select_by_postscript_name("ArialMT")
        .unwrap()
        .load()
        .unwrap();
    println!("Font loaded");

    let glyph_id = font.glyph_for_char('A').unwrap();
    let mut canvas = Canvas::new(Vector2I::splat(32), Format::A8);
    println!("Got glyph_id: {}", glyph_id);

    font.rasterize_glyph(
        &mut canvas,
        glyph_id,
        32.0,
        Transform2F::from_translation(Vector2F::new(0.0, 32.0)),
        HintingOptions::None,
        RasterizationOptions::GrayscaleAa,)
        .unwrap();
    println!("Rasterized");
}
