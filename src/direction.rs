use rand::Rng;

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum Direction {
    Up,
    Down,
    Left,
    Right,
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
