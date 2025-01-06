mod square;
mod constants;

use constants::*;
use square::Square;
use square::*;
use sdl2::pixels::Color;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use std::time::Instant;

pub fn main() {
    let sdl_context = sdl2::init().expect("Failed to initialize SDL2");
    let video_subsystem = sdl_context.video().expect("Failed to get SDL2 video subsystem");

    let window = video_subsystem.window("Smart Road", WINDOW_SIZE, WINDOW_SIZE)
        .position_centered()
        .build()
        .expect("Failed to create window");

    let mut canvas = window.into_canvas().build().expect("Failed to create canvas");
    let mut event_pump = sdl_context.event_pump().expect("Failed to get SDL2 event pump");

    let mut squares: Vec<Square> = vec![];
    square::spawn_squares(&mut squares);

    let mut last_square_spawn = Instant::now();

    'running: loop {
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit {..} |
                Event::KeyDown { keycode: Some(Keycode::Escape), .. } => {
                    break 'running;
                },
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

        // lines
        for i in 4..=10 {
            let x = i * LINE_SPACING;
            if i == 7 {
            canvas.set_draw_color(Color::RGB(0, 255, 0)); // Green color for the 4th line
            } else {
            canvas.set_draw_color(Color::RGB(255, 255, 255)); // White color for other lines
            }
            canvas.draw_line((x, 0), (x, WINDOW_SIZE as i32)).unwrap();
            canvas.draw_line((0, x), (WINDOW_SIZE as i32, x)).unwrap();
        }

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
        ::std::thread::sleep(FRAME_DURATION);
    }
}