mod constants;
mod image;
mod metrics;
mod text;
mod car;
use constants::*;
use image::draw_image;
use metrics::*;  // Changed to import all metrics functions
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;
// Remove unused Instant import
use text::draw_text;
use crate::car::Car;
use rand::Rng;
use sdl2::image::LoadTexture;

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
}

fn spawn_random_car(cars: &mut Vec<Car>, next_id: u32) {
    let mut rng = rand::thread_rng();
    let behaviors = [
        ("RU", "West"), ("RL", "West"), ("RD", "West"),
        ("DU", "North"), ("DL", "North"), ("DR", "North"),
        ("LU", "East"), ("LR", "East"), ("LD", "East"),
        ("UD", "South"), ("UR", "South"), ("UL", "South"),
    ];
    let (behavior, direction) = behaviors[rng.gen_range(0..behaviors.len())];
    Car::spawn_if_can(cars, next_id, behavior, direction);
}

fn render_simulation(
    canvas: &mut sdl2::render::Canvas<sdl2::video::Window>,
    event_pump: &mut sdl2::EventPump,
) {
    let mut is_random_generation = false;
    let mut next_id: u32 = 0;
    let mut cars: Vec<Car> = Vec::new();

    // Create texture for cars
    let texture_creator = canvas.texture_creator();
    let car_texture = texture_creator.load_texture("assets/car.png").expect("Could not load car texture");
    
    draw_lines(canvas);

    'simulation_loop: loop {
        // Handle events
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. } => std::process::exit(0),
                Event::KeyDown { keycode: Some(Keycode::Escape), .. } => break 'simulation_loop,
                Event::KeyDown { keycode: Some(Keycode::Up), .. } => {
                    spawn_car_with_direction(&mut cars, next_id, "UD", "South");
                    next_id += 1;
                    increment_vehicle_count();  // Add metrics tracking
                }
                Event::KeyDown { keycode: Some(Keycode::Down), .. } => {
                    spawn_car_with_direction(&mut cars, next_id, "DU", "North");
                    next_id += 1;
                }
                Event::KeyDown { keycode: Some(Keycode::Left), .. } => {
                    spawn_car_with_direction(&mut cars, next_id, "LR", "East");
                    next_id += 1;
                }
                Event::KeyDown { keycode: Some(Keycode::Right), .. } => {
                    spawn_car_with_direction(&mut cars, next_id, "RL", "West");
                    next_id += 1;
                }
                Event::KeyDown { keycode: Some(Keycode::R), .. } => is_random_generation = !is_random_generation,
                _ => {}
            }
        }

        if is_random_generation && rand::random::<f32>() < 0.02 {
            spawn_random_car(&mut cars, next_id);
            next_id += 1;
            increment_vehicle_count();  // Add metrics tracking
        }

        // Clear and draw background
        canvas.set_draw_color(Color::RGB(0, 0, 0));
        canvas.clear();
        draw_lines(canvas);

        // Draw and update all cars
        let mut i = 0;
        while i < cars.len() {
            let mut temp_cars = cars.clone();
            
            // Update car
            cars[i].update_radar(i, &temp_cars);
            cars[i].adjust_current_speed();
            cars[i].move_one_step_if_no_collide(&mut temp_cars);
            
            // Draw car
            cars[i].draw_all_components(canvas, &car_texture, true)
                .expect("Failed to draw car");

            // Check for collisions
            if cars[i].check_for_collision(&mut temp_cars) {
                increment_close_call_count();
            }

            i += 1;
        }

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
