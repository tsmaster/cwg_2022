// mode/car.rs
//
// this represents a realtime 2d model of a car, with kinematics and collision
// simple arcade physics from http://kidscancode.org/godot_recipes/3.x/2d/car_steering/

use crate::math::vectors::Vec2f;
use crate::math::utils::degrees_to_radians;
use crate::math::utils::radians_to_degrees;

pub struct Car {
    // current state of car
    pub position: Vec2f, // center
    pub velocity: Vec2f,

    pub heading_degrees: f32,

    // intrinsic car parameters
    pub wheel_base: f32, // m, dist from front to rear wheel
    
    pub max_acceleration: f32, // m/s^2
    pub top_speed: f32, // m/s
    pub top_speed_reverse: f32, // m/s

    pub default_deceleration: f32, // m/s^2
    pub max_braking: f32, // m/s^2

    pub drift_deceleration: f32, // m/s^2

    pub steering_angle_degrees: f32, // maximum angle of wheels

    pub friction: f32,
    pub drag: f32,
}

impl Car {
    pub fn new(posn: &Vec2f, heading: f32) -> Car {
	Car {
	    position: *posn,
	    velocity: Vec2f::ZERO,
	    heading_degrees: heading,

	    wheel_base: 50.0,
	    max_acceleration: 20.0,
	    top_speed: 60.0,
	    top_speed_reverse: 45.0,
	    default_deceleration: 2.5,
	    drift_deceleration: 10.0,
	    max_braking: 10.0,
	    steering_angle_degrees: 30.0,

	    friction: -0.25, 
	    drag: -0.0015,
	}
    }

    pub fn update(&mut self, dt: f32, input: &CarControls) {

	// apply friction/drag
	if self.velocity.magnitude() < 0.05 && input.acceleration == 0.0 && input.brake == 0.0 {
	    self.velocity = Vec2f::ZERO;
	}
	let friction_force = self.velocity * self.friction;
	let drag_force = self.velocity * self.velocity.magnitude() * self.drag;

	let drag_friction_accel = drag_force + friction_force;
	
	let accel = Vec2f::make_angle_mag(degrees_to_radians(self.heading_degrees), self.max_acceleration * input.acceleration - self.max_braking * input.brake) + drag_friction_accel;

	self.velocity += accel * dt;
	if self.velocity.magnitude() > self.top_speed {
	    self.velocity = self.velocity.normalized() * self.top_speed;
	}

	let forward_unit = Vec2f::make_angle_mag(degrees_to_radians(self.heading_degrees), 1.0);

	let mut rear_wheel_posn = self.position + (forward_unit * self.wheel_base * -0.5);
	let mut front_wheel_posn = self.position + (forward_unit * self.wheel_base * 0.5);

	let steer = input.steering * self.steering_angle_degrees;
	let wheel_angle_degrees = self.heading_degrees + steer;
	let wheel_forward_unit = Vec2f::make_angle_mag(degrees_to_radians(wheel_angle_degrees), 1.0);
	
	rear_wheel_posn += self.velocity * dt;
	front_wheel_posn += self.velocity.rotated(steer) * dt;
	
	self.position = (front_wheel_posn + rear_wheel_posn) * 0.5;

	// note that "new heading" may be in reverse
	let new_heading_unit = (front_wheel_posn - rear_wheel_posn).normalized();
	let d = new_heading_unit.dot(&self.velocity);
	if d > 0.0 {
	    self.velocity = new_heading_unit * self.velocity.magnitude();
	} else {
	    self.velocity = new_heading_unit * -1.0 * f32::min(self.velocity.magnitude(), self.top_speed_reverse);
	}

	self.heading_degrees = radians_to_degrees(new_heading_unit.angle());
    }
}
    
pub struct CarControls {
    pub steering: f32, // -1 (left) to 0 (straight) to 1 (right)
    pub acceleration: f32, // 0 (no acceleration) to 1 (max acceleration)
    pub brake: f32, // 0 (no brake) to 1 (max braking)
}
