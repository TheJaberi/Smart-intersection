use crate::car::CAR_SIZE;

pub const WINDOW_SIZE: u32 = 800; // Changed back to 800
pub const LINE_SPACING: i32 = (WINDOW_SIZE / 14) as i32;
pub const FRAME_DURATION: std::time::Duration = std::time::Duration::from_millis(16);
pub const OFFSET: f32 = (LINE_SPACING as f32 / 2.0) - (CAR_SIZE.y / 2.75);
