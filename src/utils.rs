use fontdb::Source;
use sdl2::pixels::Color;
use std::path::PathBuf;

use sdl2::image::LoadTexture;
use sdl2::rect::{Point, Rect};
use sdl2::render::{Texture, TextureCreator, WindowCanvas};
use sdl2::video::WindowContext;

pub struct TextureAtlas<'a> {
    pub texture: Texture<'a>,
}

impl<'a> TextureAtlas<'a> {
    pub fn new_from_path(
        creator: &'a TextureCreator<WindowContext>,
        filename: PathBuf,
    ) -> Result<TextureAtlas<'a>, String> {
        match creator.load_texture(filename) {
            Ok(t) => Ok(TextureAtlas { texture: t }),
            Err(e) => Err(e),
        }
    }

    pub fn from_bytes(
        creator: &'a TextureCreator<WindowContext>,
        bytes: &[u8],
    ) -> Result<TextureAtlas<'a>, String> {
        match creator.load_texture_bytes(bytes) {
            Ok(t) => Ok(TextureAtlas { texture: t }),
            Err(e) => Err(e),
        }
    }

    pub fn render(&self, canvas: &mut WindowCanvas, pos: Point) -> Result<(), String> {
        let query = self.texture.query();

        let sprite_rect = Rect::new(0, 0, query.width, query.height);

        let dest_rest = Rect::new(pos.x(), pos.y(), query.width, query.height);

        canvas.copy(&self.texture, sprite_rect, dest_rest)
    }
}

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
