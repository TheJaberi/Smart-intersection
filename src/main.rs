mod constants;
mod direction;
mod image;
mod metrics;
mod square;
mod text;
use constants::*;
use direction::Direction;
use image::draw_image;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;
use square::{spawn_random_square, spawn_square_with_direction, Square};
use std::time::Instant;
use text::draw_text;

pub fn main() {
    let sdl_context = sdl2::init().expect("Failed to initialize SDL2");
    let video_subsystem = sdl_context
        .video()
        .expect("Failed to get SDL2 video subsystem");
    let ttf_context = sdl2::ttf::init().expect("Failed to initialize SDL2 TTF");

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

    render_simulation(&mut canvas, &mut event_pump);

    render_metrics(&mut canvas, &mut event_pump, &ttf_context);
}

fn render_simulation(
    mut canvas: &mut sdl2::render::Canvas<sdl2::video::Window>,
    event_pump: &mut sdl2::EventPump,
) {
    let mut squares: Vec<Square> = vec![];

    let mut last_square_spawn = Instant::now();
    let mut is_random_generation = false;

    // Draw the lines once at the start
    draw_lines(&mut canvas);

    let mut game_over = false;
    'simulation_loop: loop {
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. }
                | Event::KeyDown {
                    keycode: Some(Keycode::Escape),
                    ..
                } => {
                    game_over = true;
                    break 'simulation_loop;
                }

                _ => {}
            }

            if last_square_spawn.elapsed() >= SQUARE_SPAWN_INTERVAL {
                match event {
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
                        spawn_square_with_direction(
                            &mut squares,
                            Direction::Right,
                            Direction::Left,
                        );
                    }
                    Event::KeyDown {
                        keycode: Some(Keycode::Right),
                        ..
                    } => {
                        spawn_square_with_direction(
                            &mut squares,
                            Direction::Left,
                            Direction::Right,
                        );
                    }
                    Event::KeyDown {
                        keycode: Some(Keycode::R),
                        ..
                    } => {
                        is_random_generation = !is_random_generation;
                    }
                    _ => {}
                }
                last_square_spawn = Instant::now();
            }
        }

        // Add a new square every 5 seconds
        if is_random_generation && last_square_spawn.elapsed() >= SQUARE_SPAWN_INTERVAL {
            spawn_random_square(&mut squares);
            last_square_spawn = Instant::now();
        }

        if game_over {
            break 'simulation_loop;
        }
        const INTERSECTION_X: i32 = 400;
        const INTERSECTION_Y: i32 = 400;
        // Collision check: Look for collisions between squares
        for i in 0..squares.len() {
            let mut is_car_near = false;
            let mut can_move = true;

            for j in (i + 1)..squares.len() {
                if i != j {
                    // Compare distances to the intersection
                    let distance_i =
                        squares[i].distance_to_intersection(INTERSECTION_X, INTERSECTION_Y);
                    let distance_j =
                        squares[j].distance_to_intersection(INTERSECTION_X, INTERSECTION_Y);

                    if distance_j < distance_i
                        || (distance_j == distance_i
                            && squares[j].priority() < squares[i].priority())
                    {
                        can_move = false;
                        println!(
                            "Car {} must wait for car {}: Priority {} vs {}",
                            i,
                            j,
                            squares[i].priority(),
                            squares[j].priority()
                        );
                        break;
                    }
                }

                if can_move {
                    // Move the car normally
                    squares[i].update();
                    println!(
                        "Car {} is moving: Position {:?}, Velocity {}",
                        i, squares[i].rect, squares[i].velocity
                    );
                } else {
                    // Slow down or stop the car
                    squares[i].velocity = (squares[i].velocity - 1).max(0);
                }

                // check if cars are too close
                if i != j && squares[i].is_near(&squares[j], 50) {
                    is_car_near = true;
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
        }

        if game_over {
            break;
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

fn render_metrics(
    mut canvas: &mut sdl2::render::Canvas<sdl2::video::Window>,
    event_pump: &mut sdl2::EventPump,
    ttf_context: &sdl2::ttf::Sdl2TtfContext,
) {
    // Game over text
    canvas.set_draw_color(Color::RGB(0, 0, 0));
    canvas.clear();
    draw_text(
        &canvas.texture_creator(),
        "assets/Roboto-Regular.ttf",
        48,
        "Simulation Over",
        Color::RGB(255, 255, 255),
        WINDOW_SIZE as i32 / 2 - 180,
        WINDOW_SIZE as i32 / 2 - 50,
        &mut canvas,
        &ttf_context,
    )
    .unwrap();

    canvas.present();

    'metrics_loop: loop {
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. }
                | Event::KeyDown {
                    keycode: Some(Keycode::Escape),
                    ..
                } => {
                    break 'metrics_loop;
                }
                _ => {}
            }
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
