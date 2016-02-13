extern crate image;

use basics::*;
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
	rng: RandomSource,
}

impl<'a> Renderer<'a>
{
	pub fn new(scene: &Scene) -> Renderer
	{
		Renderer {
			scene: scene,
			width: 800,
			height: 600,
			supersamples: 10,
			rng: RandomSource::new_unseeded(),
		}
	}

	pub fn render(&mut self) -> HdrImage
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

	fn render_pixel(&mut self, pixel_x: u32, pixel_y: u32) -> Rgb<f32>
	{
		let (x, y) = self.pixel_to_coord(pixel_x as f32, pixel_y as f32);
		let colour = self.render_sample(x, y);
		return Rgb { data: *colour.as_ref() };
	}

	fn pixel_to_coord(&mut self, pixel_x: f32, pixel_y: f32) -> (f32, f32)
	{
		let centre_x = self.width as f32 / 2.0;
		let centre_y = self.height as f32 / 2.0;
		let unit_circle = centre_x.min(centre_y);

		return ((pixel_x - centre_x) / unit_circle,
		        (centre_y - pixel_y) / unit_circle);
	}

	fn render_sample(&mut self, x: f32, y: f32) -> Colour
	{
		let ray = self.scene.camera.compute_ray(x, y);
		return self.render_ray(ray);
	}

	fn render_ray(&mut self, ray: Ray) -> Colour
	{
		if let Some(intersection) = self.scene.objects.find_intersection(ray)
		{
			let interaction = intersection.texture.evaluate_texture(&mut self.rng, ray.direction, intersection.normal);
			let child_ray_colour = self.render_ray(Ray::new(intersection.position, interaction.child_ray));
			return interaction.colour_matrix * child_ray_colour;
		}
		else
		{
			return Colour::new(1.0, 1.0, 1.0);
		}
	}
}
