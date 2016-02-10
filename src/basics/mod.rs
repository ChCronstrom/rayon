mod trans;
mod ray;

pub use basics::trans::Trans;
pub use basics::ray::Ray;

use image;
use na::{det, Mat3};

pub type HdrImage = image::ImageBuffer<image::Rgb<f32>, Vec<f32>>;

pub fn invert(m: Mat3<f32>) -> Mat3<f32>
{
	let determinant = det(&m);
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
