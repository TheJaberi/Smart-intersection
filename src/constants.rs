pub const WINDOW_SIZE: u32 = 800;
pub const LINE_SPACING: i32 = (WINDOW_SIZE / 14) as i32;
pub const SQUARE_SIZE: u32 = LINE_SPACING as u32;
pub const FRAME_DURATION: std::time::Duration = std::time::Duration::from_millis(1000 / 60);
// pub const SQUARE_SPAWN_INTERVAL: std::time::Duration = std::time::Duration::new(1, 0);
pub const SQUARE_SPAWN_INTERVAL: std::time::Duration = std::time::Duration::from_millis(250);

// Remove MIN_SPEED, MAX_SPEED, and SPEED_INCREMENT
pub const LOW_SPEED: i32 = 2;
pub const MEDIUM_SPEED: i32 = 5;
pub const HIGH_SPEED: i32 = 8;

pub const SAFE_DISTANCE: i32 = 100;
pub const CRITICAL_DISTANCE: i32 = 60;


pub const GRID_SIZE: i32 = 10; // Size of each grid cell
pub const INTERSECTION_WIDTH: i32 = WINDOW_SIZE as i32 / GRID_SIZE;
pub const INTERSECTION_HEIGHT: i32 = WINDOW_SIZE as i32 / GRID_SIZE;