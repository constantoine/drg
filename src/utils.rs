use sdl2::pixels::Color;

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
        center.x,
        center.y,
        texture_query.width,
        texture_query.height,
    );
    canvas
        .copy_ex(&texture, None, rect_target, 0.0, center, false, false)
        .unwrap();
}
