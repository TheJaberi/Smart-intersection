use lazy_static::lazy_static;
use std::sync::Mutex;

#[derive(Clone, Copy)]
pub struct Metrics {
    pub vehicle_count: u32,
    pub min_vehicle_speed: f32,
    pub max_vehicle_speed: f32,
    pub min_intersection_pass_time: f32,
    pub max_intersection_pass_time: f32,
    pub close_call_count: u32,
}

lazy_static! {
    static ref METRICS: Mutex<Metrics> = Mutex::new(Metrics {
        vehicle_count: 0,
        min_vehicle_speed: f32::MAX,
        max_vehicle_speed: f32::MIN,
        min_intersection_pass_time: f32::MAX,
        max_intersection_pass_time: f32::MIN,
        close_call_count: 0,
    });
}

// Public functions for metrics updates
pub fn increment_vehicle_count() {
    let mut metrics = METRICS.lock().unwrap();
    metrics.vehicle_count += 1;
}

pub fn increment_close_call_count() {
    let mut metrics = METRICS.lock().unwrap();
    metrics.close_call_count += 1;
}

pub fn update_intersection_time(time: f32) {
    let mut metrics = METRICS.lock().unwrap();
    if time < metrics.min_intersection_pass_time {
        metrics.min_intersection_pass_time = time;
    }
    if time > metrics.max_intersection_pass_time {
        metrics.max_intersection_pass_time = time;
    }
}

pub fn update_vehicle_speed(speed: f32) {
    let mut metrics = METRICS.lock().unwrap();
    if speed < metrics.min_vehicle_speed {
        metrics.min_vehicle_speed = speed;
    }
    if speed > metrics.max_vehicle_speed {
        metrics.max_vehicle_speed = speed;
    }
}

// Public function to retrieve a copy of the metrics
pub fn get_metrics() -> Metrics {
    *METRICS.lock().unwrap()
}
