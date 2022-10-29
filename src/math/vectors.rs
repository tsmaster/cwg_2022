// math/vectors.rs

use std::ops::Add;
use std::ops::Mul;

#[derive(Debug, Clone, Copy)]
pub struct Vec2f {
    pub x: f32,
    pub y: f32,
}

impl Add for Vec2f {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
	Self {
	    x: self.x + rhs.x,
	    y: self.y + rhs.y
	}
    }
}

impl Mul<f32> for Vec2f {
    type Output = Self;

    fn mul(self, rhs: f32) -> Self::Output {
	Self {
	    x: self.x * rhs,
	    y: self.y * rhs
	}
    }
}

impl Vec2f {
    pub fn new(x: f32, y: f32) -> Vec2f {
	Vec2f {
	    x: x,
	    y: y
	}
    }

    pub fn make_angle_mag(angle_rad: f32, mag: f32) -> Vec2f {
	Vec2f {
	    x: f32::cos(angle_rad) * mag,
	    y: f32::sin(angle_rad) * mag
	}
    }

    pub const ZERO:Vec2f = Vec2f{x: 0.0, y:0.0};

    pub fn magnitude(&self) -> f32 {
	f32::sqrt(self.x * self.x + self.y * self.y)
    }

    pub fn normalized(&self) -> Vec2f {
	let m = self.magnitude();
	Vec2f {
	    x: self.x / m,
	    y: self.y / m
	}
    }
}
