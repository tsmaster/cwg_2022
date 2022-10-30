// mode/car.rs
//
// this represents a realtime 2d model of a car, with kinematics and collision

use crate::math::vectors::Vec2f;
use crate::math::utils::degrees_to_radians;

pub struct Car {
    pub position: Vec2f,
    pub velocity: Vec2f,

    pub heading_degrees: f32,

    pub max_acceleration: f32, // m/s^2
    pub top_speed: f32, // m/s

    pub default_deceleration: f32, // m/s^2
    pub max_braking: f32, // m/s^2

    pub drift_deceleration: f32,

    pub turning_circle_radius: f32, // m
}

impl Car {
    pub fn new(posn: &Vec2f, heading: f32) -> Car {
	Car {
	    position: *posn,
	    velocity: Vec2f::ZERO,
	    heading_degrees: heading,
	    max_acceleration: 20.0,
	    top_speed: 60.0,
	    default_deceleration: 2.5,
	    drift_deceleration: 10.0,
	    max_braking: 10.0,
	    turning_circle_radius: 10.0,
	}
    }

    pub fn update(&mut self, dt: f32, input: &CarControls) {
	// update position
	self.position = self.position + self.velocity * dt;

	// update heading
	let max_turn = 30.0; // degrees per second
	let turn = max_turn * input.steering * dt;
	self.heading_degrees += turn;
	while self.heading_degrees >= 360.0 {
	    self.heading_degrees -= 360.0;
	}
	while self.heading_degrees < 0.0 {
	    self.heading_degrees += 360.0;
	}

	// update velocity
	if input.acceleration > 0.0 {
	    let heading_rad = degrees_to_radians(self.heading_degrees);
	    
	    let delta_velocity = Vec2f::make_angle_mag(heading_rad,
						      input.acceleration * self.max_acceleration * dt);
	    self.velocity = self.velocity + delta_velocity;
	} else if input.brake > 0.0 {
	    // slow down due to braking
	    let old_mag = self.velocity.magnitude();
	    if old_mag > 0.0 {
		let delta_velocity = self.velocity.normalized() * -1.0 * input.brake * self.max_braking * dt;
		if delta_velocity.magnitude() >= self.velocity.magnitude() {
		    self.velocity = Vec2f::ZERO;
		} else {
		    self.velocity = self.velocity + delta_velocity;
		}
	    }
	} else {
	    // slow down due to internal friction
	    let old_mag = self.velocity.magnitude();
	    if old_mag > 0.0 {
		let delta_velocity = self.velocity.normalized() * -1.0 * self.default_deceleration * dt;
		if delta_velocity.magnitude() >= self.velocity.magnitude() {
		    self.velocity = Vec2f::ZERO;
		} else {
		    self.velocity = self.velocity + delta_velocity;
		}
	    }
	}

	// slow down due to drifting
	let speed = self.velocity.magnitude();
	if speed > 0.0 {
	    let vel_hat = self.velocity.normalized();
	    let heading_rad = degrees_to_radians(self.heading_degrees);
	    let heading_hat = Vec2f::make_angle_mag(heading_rad, 1.0);
	    let drift_angle_rad = f32::acos(vel_hat.dot(&heading_hat));
	    let delta_velocity = vel_hat * -1.0 * f32::sin(drift_angle_rad) * dt * self.drift_deceleration;
	    if delta_velocity.magnitude() >= speed {
		self.velocity = Vec2f::ZERO;
	    } else {
		self.velocity = self.velocity + delta_velocity;
	    }
	}

	if self.velocity.magnitude() > self.top_speed {
	    self.velocity = self.velocity.normalized() * self.top_speed;
	}
    }
}
    
pub struct CarControls {
    pub steering: f32, // 1 (left) to 0 (straight) to -1 (right)
    pub acceleration: f32, // 0 (no acceleration) to 1 (max acceleration)
    pub brake: f32, // 0 (no brake) to 1 (max braking)
}
