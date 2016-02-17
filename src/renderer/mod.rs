extern crate image;

use basics::*;
use intersectable::Intersectable;
use scene::Scene;

use image::Rgb;
use na;
use num::traits::Zero;

use rand::Rng;

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
            if (y % 100) == 0
            {
                println!("Rendering line {} of {}.", y, self.height);
            }

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
        let mut accumulated_colour = Colour::zero();
        for _ in 0..self.supersamples
        {
            let (x, y) = (pixel_x as f32 + self.rng.next_f32() - 0.5, pixel_y as f32 + self.rng.next_f32() - 0.5);
            let (x, y) = self.pixel_to_coord(x, y);
            accumulated_colour = accumulated_colour + self.render_sample(x, y);
        }
        let colour = accumulated_colour / self.supersamples as Float;
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
        // If the ray intersects something in the scene
        if let Some(intersection) = self.scene.objects.find_intersection(ray)
        {
            // If there was a texture at the intersection
            if let Some(texture) = intersection.texture
            {
                let interaction = texture.evaluate_texture_point(&mut self.rng, ray.direction, intersection.normal);

                // If the colour transformation matrix is non-zero, we spawn a child ray.
                // TODO: Implement better ray cancellation criteria
                if !na::is_zero(&interaction.colour_matrix.transformation)
                {
                    let child_ray_colour = self.render_ray(Ray::new(intersection.position, interaction.child_ray));
                    return interaction.colour_matrix.transform_colour(child_ray_colour);
                }

                // Otherwise we just return the emissive component
                else
                {
                    return interaction.colour_matrix.translation;
                }
            }

            // If the intersection is untextured, I don't know, black I guess. Or panic!() works too.
            else
            {
                return Colour::new(0.0, 0.0, 0.0);
            }
        }

        // If the ray doesn't intersect anything, return the background colour.
        else
        {
            // TODO: Implement procedural backgrounds in the Scene object.
            return Colour::new(0.1, 0.1, 0.1);
        }
    }
}
