use fontdb::Source;
use sdl2::pixels::Color;

/// Helper function to load a font with the SDL2 manager using the OS's font catalogue.
pub fn load_fonts<'ttf>(
    manager: &'ttf sdl2::ttf::Sdl2TtfContext,
    font_family: &str,
) -> Result<sdl2::ttf::Font<'ttf, 'static>, String> {
    let mut db = fontdb::Database::new();
    let mut query = fontdb::Query::default();
    let family = [fontdb::Family::Name(font_family)];
    query.families = &family;
    db.load_system_fonts();
    let font = match db.query(&query) {
        Some(x) => x,
        None => return Err("font not found".to_string()),
    };

    let source = match db.face_source(font) {
        Some(x) => x.0,
        None => return Err("font not found".to_string()),
    };

    let path = match source {
        Source::Binary(_) => return Err("cannot use in-memory fonts".to_string()),
        Source::File(x) => x,
        Source::SharedFile(x, _) => x,
    };

    manager.load_font(path.clone(), 22)
}

/// Helper function to render using ttf font.
pub fn render_text<S, T>(
    canvas: &mut sdl2::render::Canvas<T>,
    font: &sdl2::ttf::Font,
    texture_creator: &sdl2::render::TextureCreator<S>,
    center: sdl2::rect::Point,
    text: &str,
) where
    T: sdl2::render::RenderTarget,
{
    let text = font
        .render(text)
        .blended(Color::RGBA(0, 20, 0, 255))
        .unwrap();
    let texture = texture_creator.create_texture_from_surface(text).unwrap();
    let texture_query = texture.query();
    let rect_target = sdl2::rect::Rect::new(
        center.x - 3,
        center.y - 10,
        texture_query.width,
        texture_query.height,
    );
    canvas
        .copy_ex(&texture, None, rect_target, 0.0, center, false, false)
        .unwrap();
}
