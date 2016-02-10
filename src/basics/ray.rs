use na::{Pnt3, Vec3};

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
