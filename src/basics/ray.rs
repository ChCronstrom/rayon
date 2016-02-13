use basics::*;

#[derive(Copy, Clone, Debug)]
pub struct Ray {
	pub origin: Point,
	pub direction: Vector,
	pub start: Float,
	pub stop: Float
}

impl Ray
{
	pub fn new(origin: Point, direction: Vector) -> Ray
	{
		Ray {
			origin: origin,
			direction: direction,
			start: EPSILON,
			stop: INFINITY,
		}
	}

	pub fn evaluate(self, t: Float) -> Point
	{
		self.origin + self.direction * t
	}
}
