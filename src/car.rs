use crate::{constants::*, increment_spawn_count};
use rand::Rng;
use sdl2::rect::Rect as SdlRect;
use sdl2::render::BlendMode;
use sdl2::render::{Texture, WindowCanvas};
use std::time::Instant;

/// A simple 2D vector for float values
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Vec2 {
    pub x: f32,
    pub y: f32,
}

impl Vec2 {
    pub fn new(x: f32, y: f32) -> Self {
        Self { x, y }
    }
    pub fn distance(&self, other: Vec2) -> f32 {
        let dx = other.x - self.x;
        let dy = other.y - self.y;
        (dx * dx + dy * dy).sqrt()
    }
}

/// A float-based rectangle for collision/drawing
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct FRect {
    pub x: f32,
    pub y: f32,
    pub w: f32,
    pub h: f32,
}

impl FRect {
    pub fn new(x: f32, y: f32, w: f32, h: f32) -> Self {
        Self { x, y, w, h }
    }

    /// Returns `Some(intersection)` if `self` intersects `other`, else `None`.
    pub fn intersect(&self, other: FRect) -> Option<FRect> {
        let rx = self.x.max(other.x);
        let ry = self.y.max(other.y);
        let rw = (self.x + self.w).min(other.x + other.w) - rx;
        let rh = (self.y + self.h).min(other.y + other.h) - ry;
        if rw > 0.0 && rh > 0.0 {
            Some(FRect::new(rx, ry, rw, rh))
        } else {
            None
        }
    }
}

pub const CAR_SIZE: Vec2 = Vec2 { x: 43., y: 33. };
pub const RADAR_SIZE: Vec2 = Vec2 { x: 43., y: 33. };

#[derive(Debug, PartialEq, Clone)]
pub struct Dimensions {
    pub long_edge: f32,
    pub short_edge: f32,
    pub delta_edge: f32,
}
#[derive(Debug, PartialEq, Clone)]
pub enum IntersectionState {
    Before,
    Inside,
    After,
}

#[derive(Clone, Debug, PartialEq)]
pub struct Car {
    pub id: u32, // Changed from uuid to simple integer id
    pub spawn_point: Vec2,
    pub lifetime: Instant,
    pub car_rect: FRect,
    pub current_direction: String,
    pub current_speed: f32,
    pub randomized_initial_speed: f32,
    pub radar: FRect,
    pub proximity: f32,
    pub has_turned: bool,
    pub behavior_code: String,
    pub waiting_flag: bool,
    pub car_size: Dimensions,
    pub radar_size: Dimensions,
    pub dest_point: Vec2,
    pub intersection_state: IntersectionState,
}

/// If so, move the car with the lower index back by 5 pixels.
pub fn check_perpendicular_and_move_back(cars: &mut Vec<Car>, i: usize, j: usize) {
    let car1 = &cars[i];
    let car2 = &cars[j];

    // Check if the cars are perpendicular
    let are_perpendicular = (car1.current_direction == "West" || car1.current_direction == "East")
        && (car2.current_direction == "North" || car2.current_direction == "South")
        || (car1.current_direction == "North" || car1.current_direction == "South")
            && (car2.current_direction == "West" || car2.current_direction == "East");

    // Check if both cars have a speed of 0
    let both_stopped = car1.current_speed == 0.0 && car2.current_speed == 0.0;
    let before_intersection = car1.intersection_state == IntersectionState::Before
        || car2.intersection_state == IntersectionState::Before;

    if are_perpendicular && both_stopped && !before_intersection {
        // Move the car with the lower index back by 5 pixels
        if i < j {
            match cars[i].current_direction.as_str() {
                "West" => cars[i].car_rect.x += 1.0,
                "East" => cars[i].car_rect.x -= 1.0,
                "North" => cars[i].car_rect.y += 1.0,
                "South" => cars[i].car_rect.y -= 1.0,
                _ => {}
            }
        } else {
            match cars[j].current_direction.as_str() {
                "West" => cars[j].car_rect.x += 1.0,
                "East" => cars[j].car_rect.x -= 1.0,
                "North" => cars[j].car_rect.y += 1.0,
                "South" => cars[j].car_rect.y -= 1.0,
                _ => {}
            }
        }
    }
}

impl Car {
    /// Create a new Car with some randomized behavior, direction, spawn points, etc.
    pub fn new(id: u32, randomized_behavior: &str, initial_direction: &str) -> Self {
        let mut rng = rand::thread_rng();
        let random_speed = rng.gen_range(0.8..2.0);

        // Calculate lane center offset (half of LINE_SPACING minus half of car width/height)
        let lane_center_h = (LINE_SPACING as f32 / 2.0) - (CAR_SIZE.y / 2.75);
        let lane_center_v = (LINE_SPACING as f32 / 2.0) - (CAR_SIZE.x / 2.75);

        // Update spawn points to match window size using LINE_SPACING
        let spawning = match randomized_behavior {
            // Right side spawns (x = window edge + 50, y = different lanes)
            "RU" => Vec2::new(
                (WINDOW_SIZE + 50) as f32,
                (LINE_SPACING * 4) as f32 + lane_center_h,
            ),
            "RL" => Vec2::new(
                (WINDOW_SIZE + 50) as f32,
                (LINE_SPACING * 5) as f32 + lane_center_h,
            ),
            "RD" => Vec2::new(
                (WINDOW_SIZE + 50) as f32,
                (LINE_SPACING * 6) as f32 + lane_center_h,
            ),

            // Down side spawns (x = different lanes, y = window edge + 50)
            "DU" => Vec2::new(
                (LINE_SPACING * 8) as f32 + lane_center_v,
                (WINDOW_SIZE + 50) as f32,
            ),
            "DL" => Vec2::new(
                (LINE_SPACING * 7) as f32 + lane_center_v,
                (WINDOW_SIZE + 50) as f32,
            ),
            "DR" => Vec2::new(
                (LINE_SPACING * 9) as f32 + lane_center_v,
                (WINDOW_SIZE + 50) as f32,
            ),

            // Left side spawns (x = -50, y = different lanes)
            "LU" => Vec2::new(-50., (LINE_SPACING * 7) as f32 + lane_center_h),
            "LR" => Vec2::new(-50., (LINE_SPACING * 8) as f32 + lane_center_h),
            "LD" => Vec2::new(-50., (LINE_SPACING * 9) as f32 + lane_center_h),

            // Up side spawns (x = different lanes, y = -50)
            "UD" => Vec2::new((LINE_SPACING * 5) as f32 + lane_center_v, -50.),
            "UR" => Vec2::new((LINE_SPACING * 6) as f32 + lane_center_v, -50.),
            "UL" => Vec2::new((LINE_SPACING * 4) as f32 + lane_center_v, -50.),
            _ => panic!("Unexpected lane"),
        };

        // Determine the car rectangle dimensions depending on direction
        let car_rect = if initial_direction == "West" || initial_direction == "East" {
            FRect::new(spawning.x, spawning.y, CAR_SIZE.x, CAR_SIZE.y)
        } else {
            FRect::new(spawning.x, spawning.y, CAR_SIZE.y, CAR_SIZE.x)
        };

        // Radar rect (initially behind or ahead of the car, depending on direction)
        let radar = FRect::new(
            spawning.x - RADAR_SIZE.x,
            spawning.y,
            RADAR_SIZE.x,
            RADAR_SIZE.y,
        );

        // Destination point based on behavior - using LINE_SPACING for consistency
        let dest_point = match randomized_behavior {
            "RU" => Vec2::new((LINE_SPACING * 7) as f32, -50.),
            "RL" => Vec2::new(-50., (LINE_SPACING * 5) as f32 + lane_center_h),
            "RD" => Vec2::new((LINE_SPACING * 7) as f32, WINDOW_SIZE as f32),
            "DU" => Vec2::new((LINE_SPACING * 8) as f32 + lane_center_v, -50.),
            "DL" => Vec2::new(-50., (LINE_SPACING * 7) as f32 + lane_center_h),
            "DR" => Vec2::new(
                WINDOW_SIZE as f32,
                (LINE_SPACING * 7) as f32 + lane_center_h,
            ),
            "LU" => Vec2::new((LINE_SPACING * 7) as f32, -50.),
            "LR" => Vec2::new(
                WINDOW_SIZE as f32,
                (LINE_SPACING * 7) as f32 + lane_center_h,
            ),
            "LD" => Vec2::new((LINE_SPACING * 7) as f32, WINDOW_SIZE as f32),
            "UD" => Vec2::new(
                (LINE_SPACING * 5) as f32 + lane_center_v,
                WINDOW_SIZE as f32,
            ),
            "UR" => Vec2::new(
                WINDOW_SIZE as f32,
                (LINE_SPACING * 5) as f32 + lane_center_h,
            ),
            "UL" => Vec2::new(-50., (LINE_SPACING * 5) as f32 + lane_center_h),
            _ => panic!("Unexpected lane"),
        };

        Car {
            id,
            lifetime: Instant::now(),
            spawn_point: spawning,
            car_rect,
            current_direction: initial_direction.to_string(),
            current_speed: random_speed,
            randomized_initial_speed: random_speed,
            radar,
            proximity: RADAR_SIZE.x,
            has_turned: false,
            behavior_code: randomized_behavior.to_string(),
            waiting_flag: false,

            car_size: Dimensions {
                long_edge: 43.,
                short_edge: 33.,
                delta_edge: CAR_SIZE.x - CAR_SIZE.y,
            },
            radar_size: Dimensions {
                long_edge: 43.,
                short_edge: 33.,
                delta_edge: CAR_SIZE.x - CAR_SIZE.y,
            },
            dest_point,
            intersection_state: IntersectionState::Before,
        }
    }

    /// Attempt to spawn a new Car if it doesn't intersect with any existing Car.
    pub fn spawn_if_can(
        cars_ref: &mut Vec<Car>,
        next_id: u32,
        randomized_behavior: &str,
        initial_direction: &str,
    ) {
        let possible_new_car = Car::new(next_id, randomized_behavior, initial_direction);
        // If none intersects and we don't exceed capacity
        if !cars_ref.iter().any(|other_car| {
            possible_new_car
                .car_rect
                .intersect(other_car.car_rect)
                .is_some()
        }) && cars_ref.len() < 9999
        {
            cars_ref.push(possible_new_car);
            increment_spawn_count();
        }
    }

    /// If approaching an intersection, check if this Car needs to wait
    /// based on other cars' positions/behaviors.
    pub fn communicate_with_intersection(
        &mut self,
        cars_ref: &Vec<Car>,
        core_intersection: &FRect,
    ) {
        let mut temp_cars = cars_ref.clone();
        temp_cars.retain(|car| car.id != self.id);

        if self.behavior_code == "LR"
            && self.radar.intersect(*core_intersection).is_some()
            && self.car_rect.intersect(*core_intersection).is_none()
        {
            self.waiting_flag = false;
            let left_turning_cars = temp_cars
                .iter()
                .filter(|car| {
                    car.car_rect.intersect(*core_intersection).is_some()
                        && (car.behavior_code == "LR"
                            || car.behavior_code == "UR"
                            || car.behavior_code == "DR")
                })
                .count();

            if left_turning_cars >= 3
                || temp_cars.iter().any(|car| {
                    car.behavior_code == "LR"
                        && car.car_rect.intersect(*core_intersection).is_some()
                })
            {
                self.waiting_flag = true;
            }
        }
        if self.behavior_code == "LU"
            && self.radar.intersect(*core_intersection).is_some()
            && self.car_rect.intersect(*core_intersection).is_none()
        {
            self.waiting_flag = false;
            if temp_cars.iter().any(|car| {
                car.behavior_code == "LU" && car.car_rect.intersect(*core_intersection).is_some()
            }) {
                self.waiting_flag = true;
                // println!("Car {} is waiting before the intersection (LU)", self.id);
            }
        }
        if self.behavior_code == "RD"
            && self.radar.intersect(*core_intersection).is_some()
            && self.car_rect.intersect(*core_intersection).is_none()
        {
            self.waiting_flag = false;
            if temp_cars.iter().any(|car| {
                car.behavior_code == "RD" && car.car_rect.intersect(*core_intersection).is_some()
            }) {
                self.waiting_flag = true;
                // println!("Car {} is waiting before the intersection (RD)", self.id);
            }
        }
        if self.behavior_code == "RL"
            && self.radar.intersect(*core_intersection).is_some()
            && self.car_rect.intersect(*core_intersection).is_none()
        {
            self.waiting_flag = false;
            if temp_cars.iter().any(|car| {
                car.behavior_code == "RL" && car.car_rect.intersect(*core_intersection).is_some()
            }) {
                self.waiting_flag = true;
                // println!("Car {} is waiting before the intersection (RL)", self.id);
            }
        }

        if self.behavior_code == "UR"
            && self.radar.intersect(*core_intersection).is_some()
            && self.car_rect.intersect(*core_intersection).is_none()
        {
            self.waiting_flag = false;
            let left_turning_cars = temp_cars
                .iter()
                .filter(|car| {
                    car.car_rect.intersect(*core_intersection).is_some()
                        && (car.behavior_code == "LR"
                            || car.behavior_code == "UR"
                            || car.behavior_code == "DR")
                })
                .count();

            if left_turning_cars >= 3
                || temp_cars.iter().any(|car| {
                    (car.behavior_code == "UR" || car.behavior_code == "RL")
                        && car.car_rect.intersect(*core_intersection).is_some()
                })
            {
                self.waiting_flag = true;
            }
        }
        if self.behavior_code == "UD"
            && self.radar.intersect(*core_intersection).is_some()
            && self.car_rect.intersect(*core_intersection).is_none()
        {
            self.waiting_flag = false;
            if temp_cars.iter().any(|car| {
                (car.behavior_code == "UD" || car.behavior_code == "RL")
                    && car.car_rect.intersect(*core_intersection).is_some()
            }) {
                self.waiting_flag = true;
                // println!("Car {} is waiting before the intersection (UD)", self.id);
            }
        }

        if self.behavior_code == "DL"
            && self.radar.intersect(*core_intersection).is_some()
            && self.car_rect.intersect(*core_intersection).is_none()
        {
            self.waiting_flag = false;
            let left_turning_cars = temp_cars
                .iter()
                .filter(|car| {
                    car.car_rect.intersect(*core_intersection).is_some()
                        && (car.behavior_code == "LR"
                            || car.behavior_code == "UR"
                            || car.behavior_code == "DR")
                })
                .count();

            if left_turning_cars >= 3
                || temp_cars.iter().any(|car| {
                    (car.behavior_code == "DL" || car.behavior_code == "UR")
                        && car.car_rect.intersect(*core_intersection).is_some()
                })
            {
                self.waiting_flag = true;
            }
        }
        if self.behavior_code == "DU"
            && self.radar.intersect(*core_intersection).is_some()
            && self.car_rect.intersect(*core_intersection).is_none()
        {
            self.waiting_flag = false;
            if temp_cars.iter().any(|car| {
                (car.behavior_code == "DU" || car.behavior_code == "LR")
                    && car.car_rect.intersect(*core_intersection).is_some()
            }) {
                self.waiting_flag = true;
                // println!("Car {} is waiting before the intersection (DU)", self.id);
            }
        }
    }

    /// Move one step in the current direction if it doesn't cause a collision.
    pub fn move_one_step_if_no_collide(&mut self, temp_cars: &mut Vec<Car>, intersection: &FRect) {
        // Copy so we can test a hypothetical move
        let mut temp_self_car = self.clone();
        temp_cars.retain(|car| temp_self_car.id != car.id);

        if self.waiting_flag {
            // println!("Car {} is waiting and not moving", self.id);
            return;
        }

        match self.current_direction.as_str() {
            "West" => {
                temp_self_car.car_rect.x -= temp_self_car.current_speed;
                if temp_cars
                    .iter()
                    .all(|car| temp_self_car.car_rect.intersect(car.car_rect).is_none())
                {
                    self.car_rect.x -= self.current_speed;
                }
            }
            "North" => {
                temp_self_car.car_rect.y -= temp_self_car.current_speed;
                if temp_cars
                    .iter()
                    .all(|car| temp_self_car.car_rect.intersect(car.car_rect).is_none())
                {
                    self.car_rect.y -= self.current_speed;
                }
            }
            "South" => {
                temp_self_car.car_rect.y += temp_self_car.current_speed;
                if temp_cars
                    .iter()
                    .all(|car| temp_self_car.car_rect.intersect(car.car_rect).is_none())
                {
                    self.car_rect.y += self.current_speed;
                }
            }
            "East" => {
                temp_self_car.car_rect.x += self.current_speed;
                if temp_cars
                    .iter()
                    .all(|car| temp_self_car.car_rect.intersect(car.car_rect).is_none())
                {
                    self.car_rect.x += self.current_speed;
                }
            }
            _ => {}
        }

        // Update intersection state if the car is inside the intersection
        if self.car_rect.intersect(*intersection).is_some() {
            self.intersection_state = IntersectionState::Inside;
        }
    }

    // Remove unused check_for_collision method
    // pub fn check_for_collision(&self, temp_cars: &mut Vec<Car>) -> bool {
    //     temp_cars.retain(|temp_car| temp_car.id != self.id);
    //     temp_cars
    //         .iter()
    //         .any(|temp_car| temp_car.car_rect.intersect(self.car_rect).is_some())
    // }

    /// Update the 'radar' rectangle based on our current direction and nearby cars.
    pub fn update_radar(&mut self, car_index: usize, temp_cars: &Vec<Car>) {
        match self.current_direction.as_str() {
            "West" => {
                self.radar.x = self.car_rect.x - self.radar_size.long_edge;
                self.radar.y = self.car_rect.y;
                self.radar.w = self.radar_size.long_edge;
                self.radar.h = self.radar_size.short_edge;

                for (other_index, other_car) in temp_cars.iter().enumerate() {
                    if car_index != other_index
                        && self.radar.intersect(other_car.car_rect).is_some()
                    {
                        self.radar.x = other_car.car_rect.x + other_car.car_rect.w;
                    }
                    self.radar.w = (self.car_rect.x - self.radar.x).abs().min(43.);
                }
            }
            "North" => {
                self.radar.x = self.car_rect.x;
                self.radar.y = self.car_rect.y - self.radar_size.long_edge;
                for (other_index, other_car) in temp_cars.iter().enumerate() {
                    if car_index != other_index
                        && self.radar.intersect(other_car.car_rect).is_some()
                    {
                        self.radar.y = other_car.car_rect.y + other_car.car_rect.h;
                    }
                    self.radar.h = (self.car_rect.y - self.radar.y).abs().min(43.);
                    self.radar.w = 35.;
                }
            }
            "South" => {
                self.radar.x = self.car_rect.x;
                self.radar.y = self.car_rect.y + self.radar_size.long_edge;
                self.radar.w = self.radar_size.short_edge;
                self.radar.h = self.radar_size.long_edge;
                for (other_index, other_car) in temp_cars.iter().enumerate() {
                    if car_index != other_index
                        && self.radar.intersect(other_car.car_rect).is_some()
                    {
                        self.radar.h =
                            other_car.car_rect.y - (self.car_rect.y + self.car_size.long_edge);
                    }
                }
            }
            "East" => {
                self.radar.x = self.car_rect.x + self.car_rect.w;
                self.radar.y = self.car_rect.y;
                self.radar.w = self.radar_size.long_edge;
                self.radar.h = self.radar_size.short_edge;

                for (other_index, other_car) in temp_cars.iter().enumerate() {
                    if car_index != other_index
                        && self.radar.intersect(other_car.car_rect).is_some()
                    {
                        self.radar.w = other_car.car_rect.x - (self.car_rect.x + self.car_rect.w);
                    }
                    // Additional logic if you have radar-radar checks, etc.
                }
            }
            _ => {}
        }
    }

    /// Adjust the Car's current speed based on radar distance.
    pub fn adjust_current_speed(&mut self) {
        if self.current_direction == "West" || self.current_direction == "East" {
            match self.radar.w {
                w if w <= 3.0 => {
                    self.current_speed = 0.0;
                }
                w if w <= 30.0 => {
                    self.current_speed = self.randomized_initial_speed * 0.25;
                }
                w if w <= 39.0 => {
                    self.current_speed = self.randomized_initial_speed * 0.50;
                }
                _ => self.current_speed = self.randomized_initial_speed,
            }
        } else if self.current_direction == "North" || self.current_direction == "South" {
            match self.radar.h {
                h if h <= 3.0 => {
                    self.current_speed = 0.0;
                }
                h if h <= 20.0 => {
                    self.current_speed = self.randomized_initial_speed * 0.25;
                }
                h if h <= 39.0 => {
                    self.current_speed = self.randomized_initial_speed * 0.50;
                }
                _ => self.current_speed = self.randomized_initial_speed,
            }
        }

        // Clamp final speed to ensure it never exceeds the initial base
        self.current_speed = self.current_speed.clamp(0.0, self.randomized_initial_speed);
    }

    /// Turn the Car if the conditions for turning (based on behavior_code) are met.
    pub fn turn_if_can(&mut self, temp_cars: &Vec<Car>) {
        // Add complete turning logic for all cases
        match self.behavior_code.as_str() {
            "RU" => self.turn_right_up(temp_cars),
            "RD" => self.turn_right_down(temp_cars),
            "LU" => self.turn_left_up(temp_cars),
            "LD" => self.turn_left_down(temp_cars),
            "UR" => self.turn_up_right(temp_cars),
            "UL" => self.turn_up_left(temp_cars),
            "DR" => self.turn_down_right(temp_cars),
            "DL" => self.turn_down_left(temp_cars),
            _ => {} // Straight paths don't turn
        }
    }

    /// From: Right lane moving West
    /// To: Upper lane moving North
    fn turn_right_up(&mut self, temp_cars: &Vec<Car>) {
        // Change turning point to use LINE_SPACING * 9
        if !self.has_turned && self.car_rect.x <= (LINE_SPACING * 9) as f32 {
            self.waiting_flag = true;
            let mut clear_to_turn = true;

            let temp_rect = FRect::new(
                (LINE_SPACING * 9) as f32,
                self.car_rect.y - (self.car_rect.w - self.car_rect.h).abs(),
                self.car_rect.h,
                self.car_rect.w,
            );

            for other_car in temp_cars {
                if self.id != other_car.id && temp_rect.intersect(other_car.car_rect).is_some() {
                    clear_to_turn = false;
                }
            }
            if clear_to_turn {
                self.car_rect = temp_rect;
                self.waiting_flag = false;
                self.current_direction = "North".to_string();
                self.has_turned = true;
            }
        }
    }

    /// From: Right lane moving West
    /// To: Lower lane moving South
    fn turn_right_down(&mut self, temp_cars: &Vec<Car>) {
        // Use same LINE_SPACING * 9 for consistency
        if !self.has_turned && self.car_rect.x <= (LINE_SPACING * 6) as f32 + OFFSET {
            self.waiting_flag = true;
            let mut clear_to_turn = true;

            let temp_rect = FRect::new(
                (LINE_SPACING * 6) as f32 + OFFSET,
                self.car_rect.y,
                self.car_rect.h,
                self.car_rect.w,
            );

            for other_car in temp_cars {
                if self.id != other_car.id && temp_rect.intersect(other_car.car_rect).is_some() {
                    clear_to_turn = false;
                }
            }
            if clear_to_turn {
                self.car_rect = temp_rect;
                self.waiting_flag = false;
                self.current_direction = "South".to_string();
                self.has_turned = true;
            }
        }
    }

    /// From: Left lane moving East
    /// To: Upper lane moving North
    fn turn_left_up(&mut self, temp_cars: &Vec<Car>) {
        if !self.has_turned && self.car_rect.x >= (LINE_SPACING * 7) as f32 + OFFSET {
            self.waiting_flag = true;
            let mut clear_to_turn = true;

            let temp_rect = FRect::new(
                (LINE_SPACING * 7) as f32 + OFFSET,
                self.car_rect.y - (self.car_rect.w - self.car_rect.h).abs(),
                self.car_rect.h,
                self.car_rect.w,
            );

            for other_car in temp_cars {
                if self.id != other_car.id && temp_rect.intersect(other_car.car_rect).is_some() {
                    clear_to_turn = false;
                }
            }
            if clear_to_turn {
                self.car_rect = temp_rect;
                self.waiting_flag = false;
                self.current_direction = "North".to_string();
                self.has_turned = true;
            }
        }
    }

    /// From: Left lane moving East
    /// To: Lower lane moving South
    fn turn_left_down(&mut self, temp_cars: &Vec<Car>) {
        if !self.has_turned && self.car_rect.x >= (LINE_SPACING * 4) as f32 + OFFSET {
            self.waiting_flag = true;
            let mut clear_to_turn = true;

            let temp_rect = FRect::new(
                (LINE_SPACING * 4) as f32 + OFFSET,
                self.car_rect.y,
                self.car_rect.h,
                self.car_rect.w,
            );

            for other_car in temp_cars {
                if self.id != other_car.id && temp_rect.intersect(other_car.car_rect).is_some() {
                    clear_to_turn = false;
                }
            }
            if clear_to_turn {
                self.car_rect = temp_rect;
                self.waiting_flag = false;
                self.current_direction = "South".to_string();
                self.has_turned = true;
            }
        }
    }

    /// From: Upper lane moving South
    /// To: Right lane moving East
    fn turn_up_right(&mut self, temp_cars: &Vec<Car>) {
        if !self.has_turned && self.car_rect.y >= (LINE_SPACING * 7) as f32 + OFFSET {
            self.waiting_flag = true;
            let mut clear_to_turn = true;

            let temp_rect = FRect::new(
                self.car_rect.x,
                (LINE_SPACING * 7) as f32 + OFFSET,
                self.car_rect.h,
                self.car_rect.w,
            );

            for other_car in temp_cars {
                if self.id != other_car.id && temp_rect.intersect(other_car.car_rect).is_some() {
                    clear_to_turn = false;
                }
            }
            if clear_to_turn {
                self.car_rect = temp_rect;
                self.waiting_flag = false;
                self.current_direction = "East".to_string();
                self.has_turned = true;
            }
        }
    }

    /// From: Upper lane moving South
    /// To: Left lane moving West
    fn turn_up_left(&mut self, temp_cars: &Vec<Car>) {
        if !self.has_turned && self.car_rect.y >= (LINE_SPACING * 4) as f32 + OFFSET {
            self.waiting_flag = true;
            let mut clear_to_turn = true;

            let temp_rect = FRect::new(
                self.car_rect.x - (self.car_rect.w - self.car_rect.h).abs(),
                (LINE_SPACING * 4) as f32 + OFFSET,
                self.car_rect.h,
                self.car_rect.w,
            );

            for other_car in temp_cars {
                if self.id != other_car.id && temp_rect.intersect(other_car.car_rect).is_some() {
                    clear_to_turn = false;
                }
            }
            if clear_to_turn {
                self.car_rect = temp_rect;
                self.waiting_flag = false;
                self.current_direction = "West".to_string();
                self.has_turned = true;
            }
        }
    }

    /// From: Lower lane moving North
    /// To: Right lane moving East
    fn turn_down_right(&mut self, temp_cars: &Vec<Car>) {
        if !self.has_turned && self.car_rect.y <= (LINE_SPACING * 9) as f32 + OFFSET {
            self.waiting_flag = true;
            let mut clear_to_turn = true;

            let temp_rect = FRect::new(
                self.car_rect.x,
                (LINE_SPACING * 9) as f32 + OFFSET,
                self.car_rect.h,
                self.car_rect.w,
            );

            for other_car in temp_cars {
                if self.id != other_car.id && temp_rect.intersect(other_car.car_rect).is_some() {
                    clear_to_turn = false;
                }
            }
            if clear_to_turn {
                self.car_rect = temp_rect;
                self.waiting_flag = false;
                self.current_direction = "East".to_string();
                self.has_turned = true;
            }
        }
    }

    /// From: Lower lane moving North
    /// To: Left lane moving West
    fn turn_down_left(&mut self, temp_cars: &Vec<Car>) {
        if !self.has_turned && self.car_rect.y <= (LINE_SPACING * 6) as f32 + OFFSET {
            self.waiting_flag = true;
            let mut clear_to_turn = true;

            let temp_rect = FRect::new(
                self.car_rect.x - (self.car_rect.w - self.car_rect.h).abs(),
                (LINE_SPACING * 6) as f32 + OFFSET,
                self.car_rect.h,
                self.car_rect.w,
            );

            for other_car in temp_cars {
                if self.id != other_car.id && temp_rect.intersect(other_car.car_rect).is_some() {
                    clear_to_turn = false;
                }
            }
            if clear_to_turn {
                self.car_rect = temp_rect;
                self.waiting_flag = false;
                self.current_direction = "West".to_string();
                self.has_turned = true;
            }
        }
    }

    /// Draw the car, radar, and/or debugging overlay using SDL2.
    ///
    /// * `canvas`      - the SDL2 rendering canvas
    /// * `car_texture` - the pre-loaded texture for the car sprite
    /// * `debug`       - whether to draw the radar/car rect for debugging
    pub fn draw_all_components(
        &self,
        canvas: &mut WindowCanvas,
        car_texture: &Texture,
        debug: bool,
    ) -> Result<(), String> {
        // Make the car visible by default
        canvas.set_draw_color(sdl2::pixels::Color::RGB(255, 255, 255));
        let car_rect = SdlRect::new(
            self.car_rect.x as i32,
            self.car_rect.y as i32,
            self.car_rect.w as u32,
            self.car_rect.h as u32,
        );
        canvas.fill_rect(car_rect)?;

        // If debug, draw the radar rect and car rect with partial alpha
        if debug {
            // Enable blending so alpha is visible
            canvas.set_blend_mode(BlendMode::Blend);

            // Draw Radar with low alpha (like macroquad's 0.1)
            canvas.set_draw_color(sdl2::pixels::Color::RGBA(255, 0, 0, 25));
            let radar_rect = SdlRect::new(
                self.radar.x as i32,
                self.radar.y as i32,
                self.radar.w as u32,
                self.radar.h as u32,
            );
            canvas.fill_rect(radar_rect)?;

            // Draw Car rect with alpha (like macroquad's 0.3)
            canvas.set_draw_color(sdl2::pixels::Color::RGBA(0, 255, 0, 77));
            let car_rect = SdlRect::new(
                self.car_rect.x as i32,
                self.car_rect.y as i32,
                self.car_rect.w as u32,
                self.car_rect.h as u32,
            );
            canvas.fill_rect(car_rect)?;
        }

        // Draw the car image with rotation based on direction
        let (angle, offset_x, offset_y) = match self.current_direction.as_str() {
            "West" => (0.0, 1.5, 1.5),
            "North" => (90.0, -3.0, 7.0),
            "South" => (270.0, -3.0, 7.0),
            "East" => (180.0, 2.0, 2.0),
            _ => (0.0, 0.0, 0.0),
        };

        // We'll draw the car at a fixed 40x30 area (like your macroquad code),
        // offset by (offset_x, offset_y) to match your original.
        let dest_rect = SdlRect::new(
            (self.car_rect.x + offset_x) as i32,
            (self.car_rect.y + offset_y) as i32,
            40,
            30,
        );

        // Render with rotation. `copy_ex` uses degrees, so we pass `angle` directly.
        //
        // - `src` is None => use the entire texture
        // - `dest_rect` is the output rectangle
        // - `angle` in degrees
        // - `center` is None => rotation around top-left
        // - `flip_horizontal/flip_vertical` are false
        canvas.copy_ex(car_texture, None, dest_rect, angle, None, false, false)?;

        Ok(())
    }
}
