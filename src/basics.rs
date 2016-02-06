use std;
use image;
use na;
use na::{Mat3, Pnt3, Vec3};

pub type HdrImage = image::ImageBuffer<image::Rgb<f32>, Vec<f32>>;

#[derive(Copy, Clone, Debug)]
pub struct Ray {
	pub origin: Pnt3<f32>,
	pub direction: Vec3<f32>,
	pub start: f32,
	pub stop: f32
}

impl Ray
{
	pub fn evaluate(self, t: f32) -> Pnt3<f32>
	{
		self.origin + self.direction * t
	}
}

#[derive(Copy, Clone, Debug)]
pub struct Trans
{
	pub transformation: Mat3<f32>,
	pub translation: Vec3<f32>
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
