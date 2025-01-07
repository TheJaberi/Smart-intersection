mod constants;
mod image;
mod square;
use constants::*;
use image::draw_image;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;
use square::Square;
use square::*;
use std::time::Instant;

pub fn main() {
    let sdl_context = sdl2::init().expect("Failed to initialize SDL2");
    let video_subsystem = sdl_context
        .video()
        .expect("Failed to get SDL2 video subsystem");

    let window = video_subsystem
        .window("Smart Road", WINDOW_SIZE, WINDOW_SIZE)
        .position_centered()
        .build()
        .expect("Failed to create window");

    let mut canvas = window
        .into_canvas()
        .build()
        .expect("Failed to create canvas");

    let mut event_pump = sdl_context
        .event_pump()
        .expect("Failed to get SDL2 event pump");

    let mut squares: Vec<Square> = vec![];
    square::spawn_squares(&mut squares);

    let mut last_square_spawn = Instant::now();

    // Draw the lines once at the start
    draw_lines(&mut canvas);

    'running: loop {
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. }
                | Event::KeyDown {
                    keycode: Some(Keycode::Escape),
                    ..
                } => {
                    break 'running;
                }
                _ => {}
            }
        }

        // Add a new square every 5 seconds
        if last_square_spawn.elapsed() >= SQUARE_SPAWN_INTERVAL {
            spawn_squares(&mut squares);
            last_square_spawn = Instant::now();
        }

        // background
        canvas.set_draw_color(Color::RGB(0, 0, 0));
        canvas.clear();

        // Draw the lines every frame
        draw_lines(&mut canvas);

        // square
        // Draw and update squares
        for square in &mut squares {
            canvas.set_draw_color(square.color);
            canvas.fill_rect(square.rect).unwrap();
            square.update();
        }

        // Remove out-of-bounds squares
        squares.retain(|square| square.is_in_bounds(WINDOW_SIZE));

        canvas.present();
        std::thread::sleep(FRAME_DURATION);
    }
}

// Function to draw the lines once
fn draw_lines(canvas: &mut sdl2::render::Canvas<sdl2::video::Window>) {
    draw_image(
        canvas,
        200 + LINE_SPACING / 2,
        150,
        LINE_SPACING as u32,
        LINE_SPACING as u32,
        "assets/arrow.up.png",
    );

    // the x point to stop at (before intersection)
    let before_intersection: i32 = 4 * LINE_SPACING; // eman approved nadeer is Supercalifragilisticexpialidocious
    let after_intersection: i32 = 10 * LINE_SPACING;

    for i in 4..=10 {
        let x = i * LINE_SPACING;
        if i == 4 || i == 10 || i == 7 {
            canvas.set_draw_color(Color::RGB(255, 255, 255)); // White color for the first, middle, and last lines
        } else {
            canvas.set_draw_color(Color::RGB(128, 128, 128)); // Gray color for other lines
        }

        if i == 7 {
            // vertical line
            canvas.draw_line((x, 0), (x, WINDOW_SIZE as i32)).unwrap();
            // horizontal line
            canvas.draw_line((0, x), (WINDOW_SIZE as i32, x)).unwrap();
        } else {
            // vertical line before intersection
            canvas.draw_line((x, 0), (x, before_intersection)).unwrap();
            // vertical line after intersection
            canvas
                .draw_line((x, after_intersection), (x, WINDOW_SIZE as i32))
                .unwrap();

            // horizontal line before intersection
            canvas.draw_line((0, x), (before_intersection, x)).unwrap();
            // horizontal line after intersection
            canvas
                .draw_line((after_intersection, x), (WINDOW_SIZE as i32, x))
                .unwrap();
        }
    }
}
