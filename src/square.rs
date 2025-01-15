use crate::direction::Direction;
use crate::{constants::*, metrics};
use lazy_static::lazy_static;
use rand::Rng;
use sdl2::pixels::Color;
use sdl2::rect::Rect;
use std::time::Instant;

// Represents a cell in the intersection where collisions could occur
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct IntersectionCell {
    x: i32,
    y: i32,
    width: i32,
    height: i32,
}

impl IntersectionCell {
    fn contains(&self, square: &Square) -> bool {
        let square_center_x = square.rect.x() + square.rect.width() as i32 / 2;
        let square_center_y = square.rect.y() + square.rect.height() as i32 / 2;

        square_center_x >= self.x
            && square_center_x <= self.x + self.width
            && square_center_y >= self.y
            && square_center_y <= self.y + self.height
    }
}

lazy_static! {
    static ref INTERSECTION_CELLS: Vec<IntersectionCell> = {
        let mut cells = Vec::new();
        // 286/57 ≈ 5, so we start from position 5
        // Create a 4x4 grid of cells for the intersection area
        for i in 0..4 {
            for j in 0..4 {
                let cell = IntersectionCell {
                    x: (5 + i) * LINE_SPACING,
                    y: (5 + j) * LINE_SPACING,
                    width: LINE_SPACING,
                    height: LINE_SPACING,
                };
                println!("Created intersection cell at x={}, y={}", cell.x, cell.y);
                cells.push(cell);
            }
        }
        cells
    };
}

#[derive(PartialEq)]
pub struct Square {
    pub rect: Rect,
    pub color: Color,
    pub initial_direction: Direction,
    pub target_direction: Direction,
    current_direction: Direction,
    turn_x: Option<i32>,
    turn_y: Option<i32>,
    pub velocity: f32,  // Change velocity to f32 for smoother speed adjustments
    pub target_velocity: f32,
    pub in_intersection: bool,
    pub entry_time: Option<Instant>,
}

#[derive(Debug)]
struct CalculatedCoordinates {
    starting_x: i32,
    starting_y: i32,
    turn_x: Option<i32>,
    turn_y: Option<i32>,
}

impl Square {
    pub fn new(
        x: i32,
        y: i32,
        initial_direction: Direction,
        target_direction: Direction,
        size: u32,
        turn_x: Option<i32>,
        turn_y: Option<i32>,
        velocity: i32,
    ) -> Self {
        let mut rng = rand::thread_rng();
        let color = Color::RGB(
            rng.gen_range(0..=255),
            rng.gen_range(0..=255),
            rng.gen_range(0..=255),
        );
        let rect = Rect::new(x, y, size, size);

        Square {
            rect,
            color,
            initial_direction,
            target_direction,
            // if we initialize with UP, then our target is DOWN
            current_direction: initial_direction.opposite(),
            turn_x,
            turn_y,
            velocity: velocity as f32,
            target_velocity: MAX_SPEED as f32,
            in_intersection: false,
            entry_time: None,
        }
    }

    pub fn priority(&self) -> i32 {
        match self.current_direction {
            Direction::Right => 1,
            Direction::Up => 2,
            Direction::Left => 3,
            Direction::Down => 4,
        }
    }

    pub fn distance_to_intersection(&self, intersection_x: i32, intersection_y: i32) -> i32 {
        // Calculate the distance to the intersection center
        let dx = (self.rect.x() - intersection_x).abs();
        let dy = (self.rect.y() - intersection_y).abs();
        dx + dy
    }

    pub fn update(&mut self) {
        // Convert velocity to i32 for movement
        let movement = self.velocity as i32;
        match self.current_direction {
            Direction::Down => self.rect.set_y(self.rect.y() + movement),
            Direction::Up => self.rect.set_y(self.rect.y() - movement),
            Direction::Left => self.rect.set_x(self.rect.x() - movement),
            Direction::Right => self.rect.set_x(self.rect.x() + movement),
        }

        // Gradually adjust speed towards target_velocity
        if (self.velocity - self.target_velocity).abs() > SPEED_INCREMENT {
            if self.velocity < self.target_velocity {
                self.velocity += SPEED_INCREMENT;
            } else {
                self.velocity -= SPEED_INCREMENT;
            }
        }

        self.turn_to_target_direction();
        self.update_intersection_status();
    }

    pub fn is_in_bounds(&self, window_size: u32) -> bool {
        let size = self.rect.width() as i32;

        self.rect.x() >= -size
            && self.rect.x() < window_size as i32
            && self.rect.y() >= -size
            && self.rect.y() < window_size as i32
    }

    fn turn_to_target_direction(&mut self) {
        // If the car is supposed to turn at a specific x-coordinate
        if let Some(turn_x) = self.turn_x {
            if (self.current_direction == Direction::Left && self.rect.x() <= turn_x)
                || (self.current_direction == Direction::Right && self.rect.x() >= turn_x)
            {
                self.rect.set_x(turn_x); // Snap to exact position
                self.current_direction = self.target_direction;
                self.turn_x = None; // Clear the turn position after turning
            }
        }

        // If the car is supposed to turn at a specific y-coordinate
        if let Some(turn_y) = self.turn_y {
            if (self.current_direction == Direction::Up && self.rect.y() <= turn_y)
                || (self.current_direction == Direction::Down && self.rect.y() >= turn_y)
            {
                self.rect.set_y(turn_y); // Snap to exact position
                self.current_direction = self.target_direction;
                self.turn_y = None; // Clear the turn position after turning
            }
        }
    }

    pub fn has_collision(&self, other: &Square) -> bool {
        self.rect.has_intersection(other.rect)
    }

    pub fn is_near(&self, other: &Square, distance: i32) -> bool {
        // Calculate the centers of both squares
        let center_self_x = self.rect.x() + self.rect.width() as i32 / 2;
        let center_self_y = self.rect.y() + self.rect.height() as i32 / 2;
        let center_other_x = other.rect.x() + other.rect.width() as i32 / 2;
        let center_other_y = other.rect.y() + other.rect.height() as i32 / 2;

        // Calculate the distance between the centers
        let dx = center_self_x - center_other_x;
        let dy = center_self_y - center_other_y;
        let distance_near = (((dx.pow(2) + dy.pow(2)) as f64).sqrt()) as i32;

        // Return true if the distance between cars is less than or equal to the specified safe distance
        distance_near <= distance
    }

    pub fn update_intersection_status(&mut self) {
        let was_in_intersection = self.in_intersection;
        self.in_intersection = INTERSECTION_CELLS.iter().any(|cell| cell.contains(self));

        if self.in_intersection {
            println!(
                "Vehicle at position ({}, {}) is in intersection!",
                self.rect.x(),
                self.rect.y()
            );
        }

        // If we just entered the intersection, record the time
        if !was_in_intersection && self.in_intersection {
            self.entry_time = Some(Instant::now());
        }
        // If we just left the intersection, calculate the time taken
        else if was_in_intersection && !self.in_intersection {
            if let Some(entry_time) = self.entry_time {
                let duration = entry_time.elapsed();
                // TODO: Update metrics with intersection pass time
            }
            self.entry_time = None;
        }
    }

    pub fn adjust_speed(&mut self, other: &Square, safe_distance: i32) {
        let center_self_x = self.rect.x() + self.rect.width() as i32 / 2;
        let center_self_y = self.rect.y() + self.rect.height() as i32 / 2;
        let center_other_x = other.rect.x() + other.rect.width() as i32 / 2;
        let center_other_y = other.rect.y() + other.rect.height() as i32 / 2;

        let dx = center_self_x - center_other_x;
        let dy = center_self_y - center_other_y;
        let distance = (((dx.pow(2) + dy.pow(2)) as f64).sqrt()) as i32;

        // Calculate relative velocity based on distance
        if distance < CRITICAL_DISTANCE {
            // Critical distance - slow down significantly
            self.target_velocity = MIN_SPEED as f32;
        } else if distance < safe_distance {
            // Scale speed based on distance
            let speed_factor = ((distance - CRITICAL_DISTANCE) as f32 
                / (safe_distance - CRITICAL_DISTANCE) as f32)
                .max(0.2);
            self.target_velocity = (MAX_SPEED as f32 * speed_factor).max(MIN_SPEED as f32);
        } else {
            // No nearby vehicles - gradually return to max speed
            self.target_velocity = MAX_SPEED as f32;
        }

        // Additional speed adjustment based on relative positions
        if self.should_yield_to(other) {
            self.target_velocity = self.target_velocity.min(other.velocity - SPEED_INCREMENT);
        }
    }

    pub fn should_yield_to(&self, other: &Square) -> bool {
        if !self.in_intersection && !other.in_intersection {
            return false;
        }

        let self_center_x = self.rect.x() + self.rect.width() as i32 / 2;
        let self_center_y = self.rect.y() + self.rect.height() as i32 / 2;
        let other_center_x = other.rect.x() + other.rect.width() as i32 / 2;
        let other_center_y = other.rect.y() + other.rect.height() as i32 / 2;

        // Distance to intersection center
        let self_dist = ((self_center_x - 400).pow(2) + (self_center_y - 400).pow(2)) as f32;
        let other_dist = ((other_center_x - 400).pow(2) + (other_center_y - 400).pow(2)) as f32;

        // Consider both distance and priority
        if (self_dist - other_dist).abs() > 5000.0 {
            return self_dist > other_dist;
        }

        // If distances are similar, use direction-based priority
        if self.priority() == other.priority() {
            // Same priority - yield to the vehicle on the right
            match self.current_direction {
                Direction::Up => other.current_direction == Direction::Right,
                Direction::Right => other.current_direction == Direction::Down,
                Direction::Down => other.current_direction == Direction::Left,
                Direction::Left => other.current_direction == Direction::Up,
            }
        } else {
            self.priority() > other.priority()
        }
    }
}

pub fn spawn_random_square(squares: &mut Vec<Square>) {
    let initial_direction = Direction::new(None);
    let target_direction = Direction::new(Some(initial_direction));
    spawn_square_with_direction(squares, initial_direction, target_direction);
}

pub fn spawn_square_with_direction(
    squares: &mut Vec<Square>,
    initial_direction: Direction,
    target_direction: Direction,
) {
    let calculated_coordinates = calculate_coordinates(initial_direction, target_direction);

    let velocity = 1;
    // let mut rng = rand::thread_rng();
    // let velocity = rng.gen_range(1..=5);

    let square = Square::new(
        calculated_coordinates.starting_x,
        calculated_coordinates.starting_y,
        initial_direction,
        target_direction,
        SQUARE_SIZE,
        calculated_coordinates.turn_x,
        calculated_coordinates.turn_y,
        velocity,
    );

    for other in squares.iter() {
        if square.has_collision(other) {
            return;
        }
    }

    metrics::increment_vehicle_count();

    squares.push(square);
}

fn calculate_coordinates(
    current_direction: Direction,
    target_direction: Direction,
) -> CalculatedCoordinates {
    // lanes from 4 to 10
    match (current_direction, target_direction) {
        (Direction::Up, Direction::Up) => unreachable!(),
        (Direction::Up, Direction::Down) => CalculatedCoordinates {
            starting_x: 5 * LINE_SPACING,
            starting_y: -LINE_SPACING,
            turn_x: None,
            turn_y: None,
        },
        (Direction::Up, Direction::Left) => CalculatedCoordinates {
            starting_x: 4 * LINE_SPACING,
            starting_y: -LINE_SPACING,
            turn_x: None,
            turn_y: Some(4 * LINE_SPACING),
        },
        (Direction::Up, Direction::Right) => CalculatedCoordinates {
            starting_x: 6 * LINE_SPACING,
            starting_y: -LINE_SPACING,
            turn_x: None,
            turn_y: Some(7 * LINE_SPACING),
        },
        (Direction::Down, Direction::Up) => CalculatedCoordinates {
            starting_x: 8 * LINE_SPACING,
            starting_y: WINDOW_SIZE as i32,
            turn_x: None,
            turn_y: None,
        },
        (Direction::Down, Direction::Down) => unreachable!(),
        (Direction::Down, Direction::Left) => CalculatedCoordinates {
            starting_x: 7 * LINE_SPACING,
            starting_y: WINDOW_SIZE as i32,
            turn_x: None,
            turn_y: Some(6 * LINE_SPACING),
        },
        (Direction::Down, Direction::Right) => CalculatedCoordinates {
            starting_x: 9 * LINE_SPACING,
            starting_y: WINDOW_SIZE as i32,
            turn_x: None,
            turn_y: Some(9 * LINE_SPACING),
        },
        (Direction::Left, Direction::Up) => CalculatedCoordinates {
            starting_x: -LINE_SPACING,
            starting_y: 7 * LINE_SPACING,
            turn_x: Some(7 * LINE_SPACING),
            turn_y: None,
        },
        (Direction::Left, Direction::Down) => CalculatedCoordinates {
            starting_x: -LINE_SPACING,
            starting_y: 9 * LINE_SPACING,
            turn_x: Some(4 * LINE_SPACING),
            turn_y: None,
        },
        (Direction::Left, Direction::Left) => unreachable!(),
        (Direction::Left, Direction::Right) => CalculatedCoordinates {
            starting_x: -LINE_SPACING,
            starting_y: 8 * LINE_SPACING,
            turn_x: None,
            turn_y: None,
        },
        (Direction::Right, Direction::Up) => CalculatedCoordinates {
            starting_x: WINDOW_SIZE as i32,
            starting_y: 4 * LINE_SPACING,
            turn_x: Some(9 * LINE_SPACING),
            turn_y: None,
        },
        (Direction::Right, Direction::Down) => CalculatedCoordinates {
            starting_x: WINDOW_SIZE as i32,
            starting_y: 6 * LINE_SPACING,
            turn_x: Some(6 * LINE_SPACING),
            turn_y: None,
        },
        (Direction::Right, Direction::Left) => CalculatedCoordinates {
            starting_x: WINDOW_SIZE as i32,
            starting_y: 5 * LINE_SPACING,
            turn_x: None,
            turn_y: None,
        },
        (Direction::Right, Direction::Right) => unreachable!(),
    }
}
