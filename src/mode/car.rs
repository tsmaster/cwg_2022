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
	    default_deceleration: 0.25,
	    max_braking: 1.5,
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
	} else {
	    // slow down due to internal friction
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
