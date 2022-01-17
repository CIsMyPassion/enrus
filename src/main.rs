use enrus::create_window;
use enrus::font::Font;
use enrus::texture::saver::save;

pub (crate) fn main() {

    let font = Font::load_standard();
    save(font.atlas(), "atlas.png".into());

    create_window();
}
