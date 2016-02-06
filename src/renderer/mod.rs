extern crate std;
extern crate image;

use basics::HdrImage;
use intersectable::Intersectable;
use scene::Scene;

use na;
use na::Vec3;
use image::Rgb;

pub struct Renderer<'a>
{
	pub width: u32,
	pub height: u32,
	pub supersamples: u32,
	scene: &'a Scene,
}

impl<'a> Renderer<'a>
{
	pub fn new(scene : &Scene) -> Renderer
	{
		Renderer { scene: scene, width: 800, height: 600, supersamples: 10 }
	}

	pub fn render(&self) -> HdrImage
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
		        (centre_y - pixel_y) / unit_circle);
	}

	fn render_sample(&self, x: f32, y: f32) -> Vec3<f32>
	{
		let ray = self.scene.camera.compute_ray(x, y);
		let intersection = self.scene.objects.find_intersection(ray);

		return intersection.map_or(na::zero(), |i| i.colour);
	}
}
