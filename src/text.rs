use sdl2::pixels::Color;
use sdl2::rect::Rect;
use sdl2::render::TextureCreator;
use sdl2::ttf::Sdl2TtfContext;
use sdl2::video::WindowContext;

pub fn draw_text<'a>(
    texture_creator: &'a TextureCreator<WindowContext>,
    font_path: &str,
    font_size: u16,
    text: &str,
    color: Color,
    x: i32,
    y: i32,
    canvas: &mut sdl2::render::Canvas<sdl2::video::Window>,
    ttf_context: &'a Sdl2TtfContext,
) -> Result<(), String> {
    let font = ttf_context
        .load_font(font_path, font_size)
        .map_err(|e| e.to_string())?;
    let surface = font
        .render(text)
        .blended(color)
        .map_err(|e| e.to_string())?;
    let texture = texture_creator
        .create_texture_from_surface(&surface)
        .map_err(|e| e.to_string())?;

    let target = Rect::new(x, y, surface.width(), surface.height());
    canvas.copy(&texture, None, Some(target))?;
    Ok(())
}
