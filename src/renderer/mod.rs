extern crate image;

use super::HdrImage;
use super::scene::Scene;

pub struct Renderer;

impl Renderer
{
	pub fn new() -> Renderer
	{
		Renderer
	}

	pub fn render(&self, scene : &Scene) -> HdrImage
	{
		HdrImage::from_pixel(800, 600, image::Rgb { data : [ 0.0, 0.2, 0.9 ] })
	}
}
