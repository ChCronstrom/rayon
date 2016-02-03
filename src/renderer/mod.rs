extern crate std;
extern crate image;

use basics::HdrImage;
use scene::Scene;

use na::Vec3;
use image::Rgb;

pub struct Renderer
{
	pub width: u32,
	pub height: u32,
	pub supersamples: u32
}

impl Renderer
{
	pub fn new() -> Renderer
	{
		Renderer { width: 800, height: 600, supersamples: 10 }
	}

	pub fn render(&self, scene : &Scene) -> HdrImage
	{
		let mut result = HdrImage::new(self.width, self.height);

		for y in 0..self.height
		{
			for x in 0..self.width
			{
				let colour = self.render_pixel(x, y);
				result.put_pixel(x, y, colour);
			}
		}

		result
	}

	fn render_pixel(&self, pixel_x: u32, pixel_y: u32) -> Rgb<f32>
	{
		let (x, y) = self.pixel_to_coord(pixel_x as f32, pixel_y as f32);
		let colour = self.render_sample(x, y);
		return Rgb { data: *colour.as_ref() };
	}

	fn pixel_to_coord(&self, pixel_x: f32, pixel_y: f32) -> (f32, f32)
	{
		let centre_x = self.width as f32 / 2.0;
		let centre_y = self.height as f32 / 2.0;
		let unit_circle = centre_x.min(centre_y);

		return ((pixel_x - centre_x) / unit_circle,
		        (pixel_y - centre_y) / unit_circle);
	}

	fn render_sample(&self, x: f32, y: f32) -> Vec3<f32>
	{
		Vec3::new(0.0, 0.2, 0.8)
	}
}
