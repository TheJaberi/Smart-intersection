pub const WINDOW_SIZE: u32 = 800;
pub const LINE_SPACING: i32 = (WINDOW_SIZE / 14) as i32;
pub const SQUARE_SIZE: u32 = LINE_SPACING as u32;
pub const FRAME_DURATION: std::time::Duration = std::time::Duration::from_millis(1000 / 60);
// pub const SQUARE_SPAWN_INTERVAL: std::time::Duration = std::time::Duration::new(1, 0);
pub const SQUARE_SPAWN_INTERVAL: std::time::Duration = std::time::Duration::from_millis(500);

pub const MIN_SPEED: i32 = 1;
pub const MAX_SPEED: i32 = 15;
pub const SPEED_INCREMENT: f32 = 0.5;
pub const SAFE_DISTANCE: i32 = 100;
pub const CRITICAL_DISTANCE: i32 = 60;
