use image;
use na::{Pnt3, Vec3};

pub type HdrImage = image::ImageBuffer<image::Rgb<f32>, Vec<f32>>;

#[derive(Copy, Clone)]
pub struct Ray {
	pub origin: Pnt3<f32>,
	pub direction: Vec3<f32>,
	pub start: f32,
	pub stop: f32
}

#[derive(Copy, Clone)]
pub struct Intersection {
	pub position: Vec3<f32>,
}
