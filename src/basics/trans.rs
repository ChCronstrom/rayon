use std;

use basics::{invert, Ray};

use na;
use na::{Mat3, Pnt3, Vec3};

#[derive(Copy, Clone, Debug)]
pub struct Trans
{
	pub transformation: Mat3<f32>,
	pub translation: Vec3<f32>
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

impl std::ops::Mul<Vec3<f32>> for Trans
{
	type Output = Vec3<f32>;

	fn mul(self, rhs: Vec3<f32>) -> Vec3<f32>
	{
		self.transformation * rhs
	}
}

impl std::ops::Mul<Pnt3<f32>> for Trans
{
	type Output = Pnt3<f32>;

	fn mul(self, rhs: Pnt3<f32>) -> Pnt3<f32>
	{
		self.transformation * rhs + self.translation
	}
}

impl std::ops::Mul<Ray> for Trans
{
	type Output = Ray;

	fn mul(self, rhs : Ray) -> Ray
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
