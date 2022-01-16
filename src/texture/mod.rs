use pathfinder_geometry::vector::Vector2I;

pub mod saver;

pub enum Format {
    Grayscale,
    RGB,
    RGBA,
}

impl Format {

    pub fn bytes_for_format(format: Format) -> u8 {
        match format {
            Format::Grayscale => 1,
            Format::RGB => 3,
            Format::RGBA => 4,
        }
    }
}


pub struct Texture {
    size: Vector2I,
    format: Format,
    pixels: Vec<u8>,
}

impl Texture {

    pub fn new(size: Vector2I, format: Format, pixels: Vec<u8>) -> Self {
        Texture{ size, format, pixels }
    }
}
