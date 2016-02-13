use std;

use basics::*;

use na::{Pnt3, Vec3};

#[derive(Copy, Clone, Debug)]
pub struct Ray {
	pub origin: Point,
	pub direction: Vector,
	pub start: f32,
	pub stop: f32
}

impl Ray
{
	pub fn new(origin: Point, direction: Vector) -> Ray
	{
		Ray {
			origin: origin,
			direction: direction,
			start: std::f32::EPSILON,
			stop: std::f32::INFINITY,
		}
	}

	pub fn evaluate(self, t: f32) -> Point
	{
		self.origin + self.direction * t
	}
}
