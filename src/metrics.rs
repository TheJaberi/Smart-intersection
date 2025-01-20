use lazy_static::lazy_static;
use std::sync::Mutex;

#[derive(Clone, Copy)]
pub struct Metrics {
    pub vehicle_count: u32,
    pub cars_spawned: u32,  // New field to track total spawns
    pub min_vehicle_speed: f32,
    pub max_vehicle_speed: f32,
    pub min_intersection_pass_time: f32,
    pub max_intersection_pass_time: f32,
    pub close_call_count: u32,
}

lazy_static! {
    static ref METRICS: Mutex<Metrics> = Mutex::new(Metrics {
        vehicle_count: 0,
        cars_spawned: 0,
        min_vehicle_speed: f32::MAX,
        max_vehicle_speed: 0.0,  // Changed from MIN to track actual speeds
        min_intersection_pass_time: f32::MAX,
        max_intersection_pass_time: 0.0,  // Changed from MIN to track actual times
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

pub fn increment_spawn_count() {
    let mut metrics = METRICS.lock().unwrap();
    metrics.cars_spawned += 1;
}

// Remove unused get_metrics function
// pub fn get_metrics() -> Metrics {
//     *METRICS.lock().unwrap()
// }

pub fn get_metrics_display() -> (String, Vec<String>) {
    let metrics = METRICS.lock().unwrap();
    
    let title = if metrics.cars_spawned == 0 {
        "No Cars Were Spawned".to_string()
    } else {
        "Simulation Stopped".to_string()
    };

    let mut stats = Vec::new();
    stats.push(format!("Total Cars Spawned: {}", metrics.cars_spawned));
    stats.push(format!("Cars Completed Journey: {}", metrics.vehicle_count));
    
    // Speed metrics
    if metrics.cars_spawned > 0 {
        if metrics.max_vehicle_speed > 0.0 {
            stats.push(format!("Max Vehicle Velocity: {:.2}", metrics.max_vehicle_speed));
            stats.push(format!("Min Vehicle Velocity: {:.2}", metrics.min_vehicle_speed));
        } else {
            stats.push("Max Vehicle Velocity: None".to_string());
            stats.push("Min Vehicle Velocity: None".to_string());
        }
    }

    // Time metrics
    if metrics.vehicle_count > 0 {
        stats.push(format!("Max Time to Pass: {:.2}s", metrics.max_intersection_pass_time));
        stats.push(format!("Min Time to Pass: {:.2}s", metrics.min_intersection_pass_time));
    } else {
        stats.push("Max Time to Pass: No completions".to_string());
        stats.push("Min Time to Pass: No completions".to_string());
    }

    stats.push(format!("Close Calls: {}", metrics.close_call_count));

    (title, stats)
}
