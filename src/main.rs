mod constants;
mod direction;
mod image;
mod metrics;
mod square;
mod text;
use constants::*;
use direction::Direction;
use image::draw_image;
use metrics::get_metrics;
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

    // Define intersection points with their initial occupancy status
    let mut intersection_points = vec![
        ((285, 285), false),
        ((285, 342), false),
        ((285, 399), false),
        ((285, 456), false),
        ((342, 285), false),
        ((342, 342), false),
        ((342, 399), false),
        ((342, 456), false),
        ((399, 285), false),
        ((399, 342), false),
        ((399, 399), false),
        ((399, 456), false),
        ((456, 285), false),
        ((456, 342), false),
        ((456, 399), false),
        ((456, 456), false),
    ];

    draw_lines(&mut canvas);

    'simulation_loop: loop {
        // Handle events
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. } => std::process::exit(0),
                Event::KeyDown {
                    keycode: Some(Keycode::Escape),
                    ..
                } => break 'simulation_loop,
                Event::KeyDown {
                    keycode: Some(Keycode::Up),
                    ..
                } => spawn_square_with_direction(&mut squares, Direction::Down, Direction::Up),
                Event::KeyDown {
                    keycode: Some(Keycode::Down),
                    ..
                } => spawn_square_with_direction(&mut squares, Direction::Up, Direction::Down),
                Event::KeyDown {
                    keycode: Some(Keycode::Left),
                    ..
                } => spawn_square_with_direction(&mut squares, Direction::Right, Direction::Left),
                Event::KeyDown {
                    keycode: Some(Keycode::Right),
                    ..
                } => spawn_square_with_direction(&mut squares, Direction::Left, Direction::Right),
                Event::KeyDown {
                    keycode: Some(Keycode::R),
                    ..
                } => is_random_generation = !is_random_generation,
                _ => {}
            }
        }

        // Spawn a random square periodically if random generation is enabled
        if is_random_generation && last_square_spawn.elapsed() >= SQUARE_SPAWN_INTERVAL {
            spawn_random_square(&mut squares);
            last_square_spawn = Instant::now();
        }

        // Predict collisions between squares
        for i in 0..squares.len() {
            let square_a = &squares[i];
            let predicted_position_a = predict_square_position(square_a);

            for j in (i + 1)..squares.len() {
                let square_b = &squares[j];
                let predicted_position_b = predict_square_position(square_b);

                // Check if the predicted positions will overlap based on X and Y ranges
                let a_left = predicted_position_a.0;
                let a_right = predicted_position_a.0 + square_a.rect.width() as i32;
                let a_top = predicted_position_a.1;
                let a_bottom = predicted_position_a.1 + square_a.rect.height() as i32;

                let b_left = predicted_position_b.0;
                let b_right = predicted_position_b.0 + square_b.rect.width() as i32;
                let b_top = predicted_position_b.1;
                let b_bottom = predicted_position_b.1 + square_b.rect.height() as i32;

                if a_right > b_left && a_left < b_right && a_bottom > b_top && a_top < b_bottom {
                    println!(
                        "Collision predicted: Square A (current: ({}, {}), predicted: ({}, {})) and Square B (current: ({}, {}), predicted: ({}, {}))",
                        square_a.rect.x(),
                        square_a.rect.y(),
                        predicted_position_a.0,
                        predicted_position_a.1,
                        square_b.rect.x(),
                        square_b.rect.y(),
                        predicted_position_b.0,
                        predicted_position_b.1
                    );
                }
            }
        }

        // Update squares
        for square in &mut squares {
            // Check each intersection point
            for (point, occupied) in &mut intersection_points {
                let (x, y) = *point;
                let intersection_distance = square.distance_to_intersection(x, y);

                if intersection_distance < SAFE_DISTANCE {
                    if *occupied {
                        // Slow down the square if the intersection is occupied
                        square.target_velocity = LOW_SPEED as f32;
                    } else {
                        // Resume normal speed if the intersection is free
                        square.target_velocity = HIGH_SPEED as f32;
                    }
                }

                // Update intersection status based on the square's presence
                if square.rect.contains_point((x, y)) {
                    *occupied = true;
                } else if *occupied {
                    *occupied = false;
                }

                // Print intersection status
                // println!(
                //     "Intersection at ({}, {}) is {}",
                //     x,
                //     y,
                //     if *occupied { "occupied" } else { "free" }
                // );
            }

            square.update();
        }

        // Render
        canvas.set_draw_color(Color::RGB(0, 0, 0));
        canvas.clear();
        draw_lines(&mut canvas);

        for square in &squares {
            canvas.set_draw_color(square.color);
            canvas.fill_rect(square.rect).unwrap();
        }

        // Remove out-of-bounds squares
        squares.retain(|square| square.is_in_bounds(WINDOW_SIZE));

        canvas.present();

        // Control frame rate
        std::thread::sleep(FRAME_DURATION);
    }
}


 fn predict_square_position(square: &Square) -> (i32, i32) {
    let movement = square.velocity as i32;
    match square.current_direction {
        Direction::Up => (square.rect.x(), square.rect.y() - movement),
        Direction::Down => (square.rect.x(), square.rect.y() + movement),
        Direction::Left => (square.rect.x() - movement, square.rect.y()),
        Direction::Right => (square.rect.x() + movement, square.rect.y()),
    }
}

fn get_two_squares(squares: &mut [Square], i: usize, j: usize) -> (&mut Square, &Square) {
    if i < j {
        let (left, right) = squares.split_at_mut(j);
        (&mut left[i], &right[0])
    } else {
        let (left, right) = squares.split_at_mut(i);
        (&mut right[0], &left[j])
    }
}

fn render_metrics(
    mut canvas: &mut sdl2::render::Canvas<sdl2::video::Window>,
    event_pump: &mut sdl2::EventPump,
    ttf_context: &sdl2::ttf::Sdl2TtfContext,
) {
    canvas.set_draw_color(Color::RGB(0, 0, 0));
    canvas.clear();

    let metrics = get_metrics();

    draw_text(
        &canvas.texture_creator(),
        "assets/Roboto-Regular.ttf",
        48,
        "Simulation Stopped",
        Color::RGB(255, 255, 255),
        WINDOW_SIZE as i32 / 2 - 200,
        100,
        &mut canvas,
        &ttf_context,
    )
    .unwrap();

    draw_text(
        &canvas.texture_creator(),
        "assets/Roboto-Regular.ttf",
        32,
        format!("Vehicles passed: {}", metrics.vehicle_count).as_str(),
        Color::RGB(255, 255, 255),
        WINDOW_SIZE as i32 / 2 - 200,
        150,
        &mut canvas,
        &ttf_context,
    )
    .unwrap();
    draw_text(
        &canvas.texture_creator(),
        "assets/Roboto-Regular.ttf",
        32,
        format!("Max Vehicle Velocity: {}", metrics.max_vehicle_speed).as_str(),
        Color::RGB(255, 255, 255),
        WINDOW_SIZE as i32 / 2 - 200,
        200,
        &mut canvas,
        &ttf_context,
    )
    .unwrap();

    draw_text(
        &canvas.texture_creator(),
        "assets/Roboto-Regular.ttf",
        32,
        format!("Min Vehicle Velocity: {}", metrics.min_vehicle_speed).as_str(),
        Color::RGB(255, 255, 255),
        WINDOW_SIZE as i32 / 2 - 200,
        250,
        &mut canvas,
        &ttf_context,
    )
    .unwrap();

    draw_text(
        &canvas.texture_creator(),
        "assets/Roboto-Regular.ttf",
        32,
        format!("Max Time to Pass: {}", metrics.max_intersection_pass_time).as_str(),
        Color::RGB(255, 255, 255),
        WINDOW_SIZE as i32 / 2 - 200,
        300,
        &mut canvas,
        &ttf_context,
    )
    .unwrap();

    draw_text(
        &canvas.texture_creator(),
        "assets/Roboto-Regular.ttf",
        32,
        format!("Min Time to Pass: {}", metrics.min_intersection_pass_time).as_str(),
        Color::RGB(255, 255, 255),
        WINDOW_SIZE as i32 / 2 - 200,
        350,
        &mut canvas,
        &ttf_context,
    )
    .unwrap();

    draw_text(
        &canvas.texture_creator(),
        "assets/Roboto-Regular.ttf",
        32,
        format!("Close Calls: {}", metrics.close_call_count).as_str(),
        Color::RGB(255, 255, 255),
        WINDOW_SIZE as i32 / 2 - 200,
        400,
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
