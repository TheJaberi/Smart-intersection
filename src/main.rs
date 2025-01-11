mod constants;
mod direction;
mod image;
mod square;
use constants::*;
use direction::Direction;
use image::draw_image;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;
use square::{spawn_random_square, spawn_square_with_direction, Square};
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

    let mut last_square_spawn = Instant::now();
    let mut is_random_generation = false;

    // Draw the lines once at the start
    draw_lines(&mut canvas);

    let mut game_over = false;
    'running: loop {
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. }
                | Event::KeyDown {
                    keycode: Some(Keycode::Escape),
                    ..
                } => {
                    game_over = true;
                    break 'running;
                }
                // Vehicle Controls
                Event::KeyDown {
                    keycode: Some(Keycode::Up),
                    ..
                } => {
                    spawn_square_with_direction(&mut squares, Direction::Down, Direction::Up);
                }
                Event::KeyDown {
                    keycode: Some(Keycode::Down),
                    ..
                } => {
                    spawn_square_with_direction(&mut squares, Direction::Up, Direction::Down);
                }
                Event::KeyDown {
                    keycode: Some(Keycode::Left),
                    ..
                } => {
                    spawn_square_with_direction(&mut squares, Direction::Right, Direction::Left);
                }
                Event::KeyDown {
                    keycode: Some(Keycode::Right),
                    ..
                } => {
                    spawn_square_with_direction(&mut squares, Direction::Left, Direction::Right);
                }
                Event::KeyDown {
                    keycode: Some(Keycode::R),
                    ..
                } => {
                    is_random_generation = !is_random_generation;
                }
                _ => {}
            }
        }

        if game_over {
            break 'running;
        }

        // Collision check: Look for collisions between squares
        for i in 0..squares.len() {
            for j in (i + 1)..squares.len() {
                let mut is_car_near = false;
                // check if cars are too close
                if squares[i].is_near(&squares[j], 2 * LINE_SPACING) {
                    println!(
                        "Cars are too close between car {} and car {}: {:?} and {:?}",
                        i, j, squares[i].rect, squares[j].rect
                    );
                }

                if is_car_near {
                    squares[i].velocity = (squares[i].velocity - 1).max(1);
                } else {
                    squares[i].velocity = (squares[i].velocity + 1).min(5);
                }

                squares[i].update();

                print!("Car {} velocity: {}\n", i, squares[i].velocity);

                // check if cars collide
                if squares[i].has_collision(&squares[j]) {
                    println!(
                        "Collision detected between car {} and car {}: {:?} and {:?}",
                        i, j, squares[i].rect, squares[j].rect
                    );
                }
            }

            if game_over {
                break;
            }

            // Add a new square every 5 seconds
            if is_random_generation && last_square_spawn.elapsed() >= SQUARE_SPAWN_INTERVAL {
                spawn_random_square(&mut squares);
                last_square_spawn = Instant::now();
            }

            // background
            canvas.set_draw_color(Color::RGB(80, 80, 80));
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
}

// Function to draw the lines once
fn draw_lines(canvas: &mut sdl2::render::Canvas<sdl2::video::Window>) {
    // the x point to stop at (before intersection)
    let before_intersection: i32 = 4 * LINE_SPACING; // eman approved nadeer is Supercalifragilisticexpialidocious
    let after_intersection: i32 = 10 * LINE_SPACING;

    for line in 4..=10 {
        let x = line * LINE_SPACING;
        if line == 4 || line == 10 || line == 7 {
            canvas.set_draw_color(Color::RGB(255, 255, 255)); // White color for the first, middle, and last lines
        } else {
            canvas.set_draw_color(Color::RGB(128, 128, 128)); // Gray color for other lines
        }

        if line == 7 {
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
        draw_arrows(canvas, line, x, before_intersection, after_intersection);
    }
}

fn draw_arrows(
    canvas: &mut sdl2::render::Canvas<sdl2::video::Window>,
    line: i32,
    x: i32,
    before_intersection: i32,
    after_intersection: i32,
) {
    match line {
        4 => {
            draw_image(
                canvas,
                x,
                before_intersection - LINE_SPACING,
                LINE_SPACING as u32,
                LINE_SPACING as u32,
                "assets/arrow.turn.png",
                180.0,
            );
            draw_image(
                canvas,
                after_intersection,
                x,
                LINE_SPACING as u32,
                LINE_SPACING as u32,
                "assets/arrow.turn.png",
                270.0,
            );
        }
        5 => {
            draw_image(
                canvas,
                x,
                before_intersection - LINE_SPACING,
                LINE_SPACING as u32,
                LINE_SPACING as u32,
                "assets/arrow.up.png",
                180.0,
            );
            draw_image(
                canvas,
                after_intersection,
                x,
                LINE_SPACING as u32,
                LINE_SPACING as u32,
                "assets/arrow.up.png",
                270.0,
            );
        }
        6 => {
            draw_image(
                canvas,
                x,
                before_intersection - LINE_SPACING,
                LINE_SPACING as u32,
                LINE_SPACING as u32,
                "assets/arrow.turn.left.png",
                180.0,
            );
            draw_image(
                canvas,
                after_intersection,
                x,
                LINE_SPACING as u32,
                LINE_SPACING as u32,
                "assets/arrow.turn.left.png",
                270.0,
            );
        }
        7 => {
            draw_image(
                canvas,
                before_intersection - LINE_SPACING,
                x,
                LINE_SPACING as u32,
                LINE_SPACING as u32,
                "assets/arrow.turn.left.png",
                90.0,
            );
            draw_image(
                canvas,
                x,
                after_intersection,
                LINE_SPACING as u32,
                LINE_SPACING as u32,
                "assets/arrow.turn.left.png",
                0.0,
            );
        }
        8 => {
            draw_image(
                canvas,
                before_intersection - LINE_SPACING,
                x,
                LINE_SPACING as u32,
                LINE_SPACING as u32,
                "assets/arrow.up.png",
                90.0,
            );
            draw_image(
                canvas,
                x,
                after_intersection,
                LINE_SPACING as u32,
                LINE_SPACING as u32,
                "assets/arrow.up.png",
                0.0,
            );
        }
        9 => {
            draw_image(
                canvas,
                before_intersection - LINE_SPACING,
                x,
                LINE_SPACING as u32,
                LINE_SPACING as u32,
                "assets/arrow.turn.png",
                90.0,
            );
            draw_image(
                canvas,
                x,
                after_intersection,
                LINE_SPACING as u32,
                LINE_SPACING as u32,
                "assets/arrow.turn.png",
                0.0,
            );
        }
        _ => {}
    }
}
