mod trans;
mod ray;

pub use basics::trans::Trans;
pub use basics::ray::Ray;

use image;
use na;
use na::{Mat3, Norm, Vec3, Pnt3};
use rand;
use rand::Rand;

pub type Float = f32;
pub type Vector = Vec3<Float>;
pub type Point = Pnt3<Float>;
pub type Matrix = Mat3<Float>;
pub type Colour = Vec3<Float>;
pub type HdrImage = image::ImageBuffer<image::Rgb<Float>, Vec<Float>>;
pub type RandomSource = rand::Isaac64Rng;

pub fn rand_vector_in_sphere<R: rand::Rng>(rng: &mut R) -> Vector
{
	loop
	{
		let (x, y, z): (Float, Float, Float) = Rand::rand(rng);
		let result = Vector::new(2.0 * x - 1.0, 2.0 * y - 1.0, 2.0 * z - 1.0);
		if result.sqnorm() < 1.0
		{
			return result;
		}
	}
}

pub fn rand_vector_in_half_sphere<R: rand::Rng>(rng: &mut R, direction: Vector) -> Vector
{
	let rand_vector = rand_vector_in_sphere(rng);

	// If dot(randvec, direction) is negative, it's pointing the wrong way. dot(randvec, direction) * normal
	// gives by how much, and so randvec - 2 * dot(randvec, direction) * direction will make it
	// point the right way.

	let dot_product = na::dot(&rand_vector, &direction);
	if dot_product < 0.0
	{
		rand_vector + direction * (-2.0 * dot_product)
	}
	else
	{
		rand_vector
	}
}

pub fn invert(m: Matrix) -> Matrix
{
	let determinant = na::det(&m);
	let inv_det = 1.0 / determinant;

	Mat3::new(inv_det * (m.m22 * m.m33 - m.m23 * m.m32),
              inv_det * (m.m13 * m.m32 - m.m12 * m.m33),
	          inv_det * (m.m12 * m.m23 - m.m13 * m.m22),

	          inv_det * (m.m23 * m.m31 - m.m21 * m.m33),
			  inv_det * (m.m11 * m.m33 - m.m13 * m.m31),
			  inv_det * (m.m13 * m.m21 - m.m11 * m.m23),

	          inv_det * (m.m21 * m.m32 - m.m22 * m.m31),
			  inv_det * (m.m12 * m.m31 - m.m11 * m.m32),
	          inv_det * (m.m11 * m.m22 - m.m12 * m.m21))
}

#[cfg(test)]
mod tests
{
	use super::*;

	use na;
	use na::*;
	use rand;

	#[test]
	fn test_rand_vector_in_sphere()
	{
		let mut randomizer = rand::thread_rng();
		for i in 0..100
		{
			let random_vector = rand_vector_in_sphere(&mut randomizer);
			assert!(random_vector.norm() < 1.0);
		}
	}

	#[test]
	fn test_rand_vector_in_half_sphere()
	{
		let mut randomizer = rand::thread_rng();

		let direction = Vector::new(1.0, 0.0, 0.0);
		for i in 0..100
		{
			let random_vector = rand_vector_in_half_sphere(&mut randomizer, direction);
			assert!(random_vector.norm() < 1.0);
			assert!(random_vector.x > 0.0);
		}

		let direction = Vector::new(-1.0, 0.0, 0.0);
		for i in 0..100
		{
			let random_vector = rand_vector_in_half_sphere(&mut randomizer, direction);
			assert!(random_vector.norm() < 1.0);
			assert!(random_vector.x < 0.0);
		}

		let direction = Vector::new(0.0, 1.0, 0.0);
		for i in 0..100
		{
			let random_vector = rand_vector_in_half_sphere(&mut randomizer, direction);
			assert!(random_vector.norm() < 1.0);
			assert!(random_vector.y > 0.0);
		}

		let direction = Vector::new(0.0, -1.0, 0.0);
		for i in 0..100
		{
			let random_vector = rand_vector_in_half_sphere(&mut randomizer, direction);
			assert!(random_vector.norm() < 1.0);
			assert!(random_vector.y < 0.0);
		}

		let direction = Vector::new(0.0, 0.0, 1.0);
		for i in 0..100
		{
			let random_vector = rand_vector_in_half_sphere(&mut randomizer, direction);
			assert!(random_vector.norm() < 1.0);
			assert!(random_vector.z > 0.0);
		}

		let direction = Vector::new(0.0, 0.0, -1.0);
		for i in 0..100
		{
			let random_vector = rand_vector_in_half_sphere(&mut randomizer, direction);
			assert!(random_vector.norm() < 1.0);
			assert!(random_vector.z < 0.0);
		}
	}
}
