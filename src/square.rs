use crate::constants::*;
use crate::direction::Direction;
use rand::Rng;
use sdl2::pixels::Color;
use sdl2::rect::Rect;

#[derive(PartialEq)]
pub struct Square {
    pub rect: Rect,
    pub color: Color,
    pub initial_direction: Direction,
    pub target_direction: Direction,
    current_direction: Direction,
    turn_x: Option<i32>,
    turn_y: Option<i32>,
    pub velocity: i32, // to control the speed
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
            velocity,
        }
    }

    pub fn update(&mut self) {
        // TODO: if more than one the `turn_to_target_direction` could break
        match self.current_direction {
            Direction::Down => self.rect.set_y(self.rect.y() + self.velocity),
            Direction::Up => self.rect.set_y(self.rect.y() - self.velocity),
            Direction::Left => self.rect.set_x(self.rect.x() - self.velocity),
            Direction::Right => self.rect.set_x(self.rect.x() + self.velocity),
        }

        self.turn_to_target_direction();
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

        distance_near <= distance
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

    squares.push(Square::new(
        calculated_coordinates.starting_x,
        calculated_coordinates.starting_y,
        initial_direction,
        target_direction,
        SQUARE_SIZE,
        calculated_coordinates.turn_x,
        calculated_coordinates.turn_y,
        velocity,
    ));
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
