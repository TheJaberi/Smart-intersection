use crate::Square;
use lazy_static::lazy_static;
use std::sync::Mutex;

pub struct Metrics {
    pub vehicle_count: u32,
    pub min_vehicle_speed: f32,
    pub max_vehicle_speed: f32,
    pub min_intersection_pass_time: f32,
    pub max_intersection_pass_time: f32,
    pub close_call_count: u32,
}

impl Metrics {
    pub fn update_intersection_time(&mut self, time: f32) {
        if self.min_intersection_pass_time == 0.0 || time < self.min_intersection_pass_time {
            self.min_intersection_pass_time = time;
        }
        if time > self.max_intersection_pass_time {
            self.max_intersection_pass_time = time;
        }
    }
}

lazy_static! {
    static ref METRICS: Mutex<Metrics> = Mutex::new(Metrics {
        vehicle_count: 0,
        min_vehicle_speed: 0.0,
        max_vehicle_speed: 0.0,
        min_intersection_pass_time: 0.0,
        max_intersection_pass_time: 0.0,
        close_call_count: 0,
    });
}

pub fn update_metrics(square: &Square) {
    let mut metrics = METRICS.lock().unwrap();
    metrics.vehicle_count += 1;
}

pub fn increment_close_call_count() {
    let mut metrics = METRICS.lock().unwrap();
    metrics.close_call_count += 1;
}

pub fn update_intersection_pass_time(time: f32) {
    let mut metrics = METRICS.lock().unwrap();
    metrics.update_intersection_time(time);
}

pub fn get_metrics() -> Metrics {
    let metrics = METRICS.lock().unwrap();
    Metrics {
        vehicle_count: metrics.vehicle_count,
        min_vehicle_speed: metrics.min_vehicle_speed,
        max_vehicle_speed: metrics.max_vehicle_speed,
        min_intersection_pass_time: metrics.min_intersection_pass_time,
        max_intersection_pass_time: metrics.max_intersection_pass_time,
        close_call_count: metrics.close_call_count,
    }
}
