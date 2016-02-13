use std;

use basics::*;

use na;
use na::{Mat3, Pnt3, Vec3};

#[derive(Copy, Clone, Debug)]
pub struct Trans
{
	pub transformation: Matrix,
	pub translation: Vector
}

impl Trans
{
	pub fn new_translation(x: f32, y: f32, z: f32) -> Trans
	{
		let mut result = Trans::default();
		result.translation = Vec3::new(x, y, z);
		return result;
	}

    pub fn invert(self) -> Trans
    {
        // y = Ax + t
        // Ax = y - t
        // x = A \ (y - t) = A\y - A\t
        let inv = invert(self.transformation);
        Trans {
            transformation: inv,
            translation: -(inv * self.translation),
        }
    }
}

impl std::ops::Mul<Vector> for Trans
{
	type Output = Vector;

	fn mul(self, rhs: Vector) -> Vector
	{
		self.transformation * rhs
	}
}

impl std::ops::Mul<Point> for Trans
{
	type Output = Point;

	fn mul(self, rhs: Point) -> Point
	{
		self.transformation * rhs + self.translation
	}
}

impl std::ops::Mul<Ray> for Trans
{
	type Output = Ray;

	fn mul(self, rhs: Ray) -> Ray
	{
		Ray
		{
			origin: self * rhs.origin,
            direction: self * rhs.direction,
            start: rhs.start,
            stop: rhs.stop,
		}
	}
}

impl Default for Trans
{
	fn default() -> Trans
	{
		Trans
		{
			transformation: na::Eye::new_identity(3),
			translation: na::zero(),
		}
	}
}
