use crate::constants::*;
use rand::Rng;
use sdl2::pixels::Color;
use sdl2::rect::Rect;

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum Direction {
    Up,
    Down,
    Left,
    Right,
}

#[derive(PartialEq)]
pub struct Square {
    pub rect: Rect,
    pub color: Color,
    pub initial_position: Direction,
    pub target_direction: Direction,
    current_direction: Direction,
    turn_x: Option<i32>,
    turn_y: Option<i32>,
}

impl Direction {
    pub fn new(exclude: Option<Direction>) -> Direction {
        let mut rng = rand::thread_rng();
        loop {
            let new_direction = match rng.gen_range(0..4) {
                0 => Direction::Up,
                1 => Direction::Left,
                2 => Direction::Down,
                3 => Direction::Right,
                _ => unreachable!(),
            };

            if let Some(exclude_dir) = exclude {
                if new_direction != exclude_dir {
                    return new_direction;
                }
            } else {
                return new_direction;
            }
        }
    }

    pub fn opposite(&self) -> Direction {
        match self {
            Direction::Up => Direction::Down,
            Direction::Down => Direction::Up,
            Direction::Left => Direction::Right,
            Direction::Right => Direction::Left,
        }
    }
}

impl Square {
    pub fn new(
        x: i32,
        y: i32,
        initial_position: Direction,
        target_direction: Direction,
        size: u32,
    ) -> Self {
        let mut rng = rand::thread_rng();
        let color = Color::RGB(
            rng.gen_range(0..=255),
            rng.gen_range(0..=255),
            rng.gen_range(0..=255),
        );
        let rect = Rect::new(x, y, size, size);
        let current_direction = initial_position.opposite();

        // Calculate turning positions based on target_direction
        let (turn_x, turn_y) = if target_direction == current_direction {
            (None, None)
        } else {
            match target_direction {
                Direction::Up => (Some(5 * LINE_SPACING), None),
                Direction::Down => (Some(4 * LINE_SPACING), None),
                Direction::Left => (None, Some(4 * LINE_SPACING)),
                Direction::Right => (None, Some(5 * LINE_SPACING)),
            }
        };
        // println!(
        //     "Current Direction: {:?}, Target Direction: {:?}, Turn X: {:?}, Turn Y: {:?}",
        //     current_direction, target_direction, turn_x, turn_y
        // );

        Square {
            rect,
            color,
            initial_position,
            target_direction,
            current_direction,
            turn_x,
            turn_y,
        }
    }

    pub fn update(&mut self) {
        match self.current_direction {
            Direction::Down => self.rect.set_y(self.rect.y() + 2),
            Direction::Up => self.rect.set_y(self.rect.y() - 2),
            Direction::Left => self.rect.set_x(self.rect.x() - 2),
            Direction::Right => self.rect.set_x(self.rect.x() + 2),
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
        if let Some(turn_x) = self.turn_x {
            if self.current_direction != self.target_direction && self.rect.x() == turn_x {
                self.current_direction = self.target_direction;
            }
        }

        if let Some(turn_y) = self.turn_y {
            if self.current_direction != self.target_direction && self.rect.y() == turn_y {
                self.current_direction = self.target_direction;
            }
        }
    }
}

pub fn spawn_squares(squares: &mut Vec<Square>) {
    let initial_position = Direction::new(None);
    let target_direction = Direction::new(Some(initial_position));

    let (x, y) = match initial_position {
        Direction::Up => (4 * LINE_SPACING, -LINE_SPACING),
        Direction::Left => (-LINE_SPACING, 5 * LINE_SPACING),
        Direction::Down => (5 * LINE_SPACING, WINDOW_SIZE as i32),
        Direction::Right => (WINDOW_SIZE as i32, 4 * LINE_SPACING),
    };

    squares.push(Square::new(
        x,
        y,
        initial_position,
        target_direction,
        SQUARE_SIZE,
    ));
}
