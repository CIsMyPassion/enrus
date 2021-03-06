use std::path::Path;
use std::fs::File;
use std::io::BufWriter;

use crate::texture::{Texture, Format};

pub fn save(texture: &Texture, location: String) {

    let path = Path::new(&location);
    let file = File::create(path).unwrap();
    let ref mut w = BufWriter::new(file);

    let mut encoder = png::Encoder::new(w, texture.size.x() as u32, texture.size.y() as u32);
    let format = match texture.format {
        Format::Grayscale => png::ColorType::Grayscale,
        Format::RGB => png::ColorType::Rgb,
        Format::RGBA => png::ColorType::Rgba,
    };
    encoder.set_color(format);
    encoder.set_depth(png::BitDepth::Eight);
    encoder.set_trns(vec!(0xFFu8, 0xFFu8, 0xFFu8, 0xFFu8));
    encoder.set_source_gamma(png::ScaledFloat::from_scaled(45455)); // 1.0 / 2.2, scaled by 100000
    encoder.set_source_gamma(png::ScaledFloat::new(1.0 / 2.2));     // 1.0 / 2.2, unscaled, but rounded
    let source_chromaticities = png::SourceChromaticities::new(     // Using unscaled instantiation here
        (0.31270, 0.32900),
        (0.64000, 0.33000),
        (0.30000, 0.60000),
        (0.15000, 0.06000)
    );
    encoder.set_source_chromaticities(source_chromaticities);

    let mut writer = encoder.write_header().unwrap();
    writer.write_image_data(&texture.pixels).unwrap();
}
