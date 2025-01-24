mod car;
mod constants;
mod image;
mod metrics;
mod text;
use crate::car::{check_perpendicular_and_move_back, Car, FRect, Vec2}; // Add FRect and Vec2 imports
use constants::*;
use image::draw_image;
use metrics::*; // Changed to import all metrics functions
use rand::Rng;
use sdl2::event::Event;
use sdl2::image::LoadTexture;
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;
use std::time::Duration;
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

fn spawn_car_with_direction(cars: &mut Vec<Car>, next_id: u32, behavior: &str, direction: &str) {
    Car::spawn_if_can(cars, next_id, behavior, direction);
    increment_spawn_count();
}

fn spawn_random_car(cars: &mut Vec<Car>, next_id: u32) {
    let mut rng = rand::thread_rng();
    let behaviors = [
        ("RU", "West"),
        ("RL", "West"),
        ("RD", "West"),
        ("DU", "North"),
        ("DL", "North"),
        ("DR", "North"),
        ("LU", "East"),
        ("LR", "East"),
        ("LD", "East"),
        ("UD", "South"),
        ("UR", "South"),
        ("UL", "South"),
    ];
    let (behavior, direction) = behaviors[rng.gen_range(0..behaviors.len())];
    Car::spawn_if_can(cars, next_id, behavior, direction);
}

fn get_random_behavior_for_direction(direction: &str) -> &'static str {
    let mut rng = rand::thread_rng();
    match direction {
        "West" => {
            let behaviors = ["RU", "RL", "RD"];
            behaviors[rng.gen_range(0..behaviors.len())]
        }
        "North" => {
            let behaviors = ["DU", "DL", "DR"];
            behaviors[rng.gen_range(0..behaviors.len())]
        }
        "East" => {
            let behaviors = ["LU", "LR", "LD"];
            behaviors[rng.gen_range(0..behaviors.len())]
        }
        "South" => {
            let behaviors = ["UD", "UR", "UL"];
            behaviors[rng.gen_range(0..behaviors.len())]
        }
        _ => panic!("Invalid direction"),
    }
}
// XXX debug, remove at submission
fn draw_intersection_bounds(canvas: &mut sdl2::render::Canvas<sdl2::video::Window>) {
    let intersection_rect = sdl2::rect::Rect::new(
        (4 * LINE_SPACING) as i32,
        (4 * LINE_SPACING) as i32,
        (6 * LINE_SPACING) as u32,
        (6 * LINE_SPACING) as u32,
    );
    canvas.set_draw_color(Color::RGB(255, 0, 0));
    canvas.draw_rect(intersection_rect).unwrap();
}

fn render_simulation(
    canvas: &mut sdl2::render::Canvas<sdl2::video::Window>,
    event_pump: &mut sdl2::EventPump,
) {
    // --- 1) Variables that must persist across frames ---
    let mut is_random_generation = false;
    let mut next_id: u32 = 0;
    let mut cars: Vec<Car> = Vec::new();
    let mut last_spawn_time = Instant::now();
    let spawn_delay = Duration::from_millis(100);

    // Create texture for cars
    let texture_creator = canvas.texture_creator();
    let car_texture = texture_creator
        .load_texture("assets/car.png")
        .expect("Could not load car texture");

    // Define intersection area
    let core_intersection = FRect::new(
        (4 * LINE_SPACING) as f32,
        (4 * LINE_SPACING) as f32,
        (6 * LINE_SPACING) as f32,
        (6 * LINE_SPACING) as f32,
    );

    // --- 2) Main game loop ---
    'simulation_loop: loop {
        // ---------------------------------------
        // A) Handle events (keyboard, quit, etc.)
        // ---------------------------------------
        for event in event_pump.poll_iter() {
            match event {
                // Quit or Escape
                Event::Quit { .. }
                | Event::KeyDown {
                    keycode: Some(Keycode::Escape),
                    ..
                } => {
                    break 'simulation_loop;
                }

                // Manually spawn with arrow keys
                Event::KeyDown {
                    keycode: Some(Keycode::Right),
                    ..
                } => {
                    let behavior = get_random_behavior_for_direction("West");
                    spawn_car_with_direction(&mut cars, next_id, behavior, "West");
                    next_id += 1;
                    increment_vehicle_count();
                }
                Event::KeyDown {
                    keycode: Some(Keycode::Left),
                    ..
                } => {
                    let behavior = get_random_behavior_for_direction("East");
                    spawn_car_with_direction(&mut cars, next_id, behavior, "East");
                    next_id += 1;
                    increment_vehicle_count();
                }
                Event::KeyDown {
                    keycode: Some(Keycode::Up),
                    ..
                } => {
                    let behavior = get_random_behavior_for_direction("South");
                    spawn_car_with_direction(&mut cars, next_id, behavior, "South");
                    next_id += 1;
                    increment_vehicle_count();
                }
                Event::KeyDown {
                    keycode: Some(Keycode::Down),
                    ..
                } => {
                    let behavior = get_random_behavior_for_direction("North");
                    spawn_car_with_direction(&mut cars, next_id, behavior, "North");
                    next_id += 1;
                    increment_vehicle_count();
                }

                // Toggle random generation with 'R'
                Event::KeyDown {
                    keycode: Some(Keycode::R),
                    ..
                } => {
                    is_random_generation = !is_random_generation;
                    last_spawn_time = Instant::now();
                }
                _ => {}
            }
        }

        // ---------------------------------------
        // B) Random car spawning on a timer
        // ---------------------------------------
        if is_random_generation && last_spawn_time.elapsed() >= spawn_delay {
            spawn_random_car(&mut cars, next_id);
            next_id += 1;
            increment_vehicle_count();
            last_spawn_time = Instant::now();
        }

        // ---------------------------------------
        // C) Clear the screen and draw the grid
        // ---------------------------------------
        canvas.set_draw_color(Color::RGB(0, 0, 0));
        canvas.clear();
        draw_lines(canvas);
        draw_intersection_bounds(canvas); // optional debug intersection

        // Let each car set waiting flags, if needed
        {
            let temp_cars = cars.clone();
            for car in cars.iter_mut() {
                car.communicate_with_intersection(&temp_cars, &core_intersection);
            }
        }

        // ---------------------------------------
        // D) First pass: Radar & Speed updates
        // ---------------------------------------
        {
            let temp_cars = cars.clone();
            for i in 0..cars.len() {
                let previous_speed = cars[i].current_speed;

                // 1) Radar + speed
                cars[i].adjust_current_speed();
                cars[i].update_radar(i, &temp_cars);

                // 2) "Close call" detection
                if (previous_speed > 0.0 && cars[i].current_speed == 0.0)
                    || (previous_speed > cars[i].current_speed * 2.0)
                {
                    increment_close_call_count();
                }

                // 3) Turning logic
                cars[i].turn_if_can(&temp_cars);

                // 4) Track speed in metrics
                update_vehicle_speed(cars[i].current_speed);
                // 5) Check for perpendicular cars with speed 0 and move one back
                for j in (i + 1)..cars.len() {
                    check_perpendicular_and_move_back(&mut cars, i, j);
                }
            }
        }

        // ---------------------------------------
        // E) Second pass: Move each car exactly once
        // ---------------------------------------
        {
            let mut temp_cars = cars.clone();
            for i in 0..cars.len() {
                // Only move if not waiting
                if !cars[i].waiting_flag {
                    cars[i].move_one_step_if_no_collide(&mut temp_cars, &core_intersection);
                }
            }
        }

        // ---------------------------------------
        // F) Draw all cars
        // ---------------------------------------
        for car in &cars {
            car.draw_all_components(canvas, &car_texture, true)
                .expect("Failed to draw car");
        }
        // Optionally draw intersection bounds again
        draw_intersection_bounds(canvas);

        // ---------------------------------------
        // G) Remove cars that have reached destination
        // ---------------------------------------
        cars.retain(|car| {
            let distance_to_dest =
                Vec2::new(car.car_rect.x, car.car_rect.y).distance(car.dest_point);
            if distance_to_dest < 20.0 {
                update_intersection_time(car.lifetime.elapsed().as_secs_f32());
                increment_vehicle_count();
                false
            } else {
                true
            }
        });

        // Present the frame and wait
        canvas.present();
        std::thread::sleep(FRAME_DURATION);
    }
}

fn render_metrics(
    mut canvas: &mut sdl2::render::Canvas<sdl2::video::Window>,
    event_pump: &mut sdl2::EventPump,
    ttf_context: &sdl2::ttf::Sdl2TtfContext,
) {
    canvas.set_draw_color(Color::RGB(0, 0, 0));
    canvas.clear();

    let (title, stats) = get_metrics_display();

    // Draw title
    draw_text(
        &canvas.texture_creator(),
        "assets/Roboto-Regular.ttf",
        48,
        &title,
        Color::RGB(255, 255, 255),
        WINDOW_SIZE as i32 / 2 - 200,
        100,
        &mut canvas,
        &ttf_context,
    )
    .unwrap();

    // Draw stats
    for (i, stat) in stats.iter().enumerate() {
        draw_text(
            &canvas.texture_creator(),
            "assets/Roboto-Regular.ttf",
            32,
            stat,
            Color::RGB(255, 255, 255),
            WINDOW_SIZE as i32 / 2 - 200,
            150 + (i as i32 * 50),
            &mut canvas,
            &ttf_context,
        )
        .unwrap();
    }

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
