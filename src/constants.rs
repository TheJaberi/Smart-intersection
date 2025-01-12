pub const WINDOW_SIZE: u32 = 800;
pub const LINE_SPACING: i32 = (WINDOW_SIZE / 14) as i32;
pub const SQUARE_SIZE: u32 = LINE_SPACING as u32;
pub const FRAME_DURATION: std::time::Duration = std::time::Duration::from_millis(1000 / 60);
// pub const SQUARE_SPAWN_INTERVAL: std::time::Duration = std::time::Duration::new(1, 0);
pub const SQUARE_SPAWN_INTERVAL: std::time::Duration = std::time::Duration::from_millis(500);
