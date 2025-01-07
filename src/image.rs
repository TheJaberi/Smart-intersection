use sdl2::{image::LoadTexture, rect::Rect, render::Canvas};

pub fn draw_image(
    canvas: &mut Canvas<sdl2::video::Window>,
    x: i32,
    y: i32,
    width: u32,
    height: u32,
    image_path: &str,
    angle: f64,
) {
    let texture_creator = canvas.texture_creator();
    let texture = texture_creator.load_texture(image_path).unwrap();

    let target = Rect::new(x, y, width, height);
    canvas
        .copy_ex(&texture, None, Some(target), angle, None, false, false)
        .unwrap();
}
