use super::*;

use std::ops::DerefMut;
use std::sync::Mutex;

use basics::*;
use intersectable::Intersectable;
use medium::Medium;

use image;
use image::Rgb;
use na;
use na::{Diagonal, Norm};
use num::traits::Zero;
use rand::{Rng, SeedableRng};

pub struct RenderTask<'a>
{
    settings: RenderSettings<'a>,
    range: RenderBlock,
    rng: RandomSource,
}

impl<'a> RenderTask<'a>
{
    pub fn new(settings: RenderSettings<'a>, range: RenderBlock, random_seed: u64) -> RenderTask<'a>
    {
        RenderTask {
            settings: settings,
            range: range,
            rng: RandomSource::from_seed(&[random_seed]),
        }
    }

    pub fn render(mut self, destination: &Mutex<HdrImage>)
    {
        let RenderBlock((xstart, xcount), (ystart, ycount)) = self.range;

        let mut result = HdrImage::new(xcount, ycount);

        for y in 0..ycount
        {
            for x in 0..xcount
            {
                let colour = self.render_pixel(xstart + x, ystart + y);
                result.put_pixel(x, y, colour);
            }
        }

        let mut destination = destination.lock().expect("Mutex was poisoned.");
        image::imageops::replace(destination.deref_mut(), &result, xstart, ystart);
    }

    fn render_pixel(&mut self, pixel_x: u32, pixel_y: u32) -> Rgb<f32>
    {
        let mut accumulated_colour = Colour::zero();
        for _ in 0..self.settings.supersamples
        {
            let (x, y) = (pixel_x as f32 + self.rng.next_f32() - 0.5, pixel_y as f32 + self.rng.next_f32() - 0.5);
            let (x, y) = self.pixel_to_coord(x, y);
            accumulated_colour = accumulated_colour + self.render_sample(x, y);
        }
        let colour = accumulated_colour / self.settings.supersamples as Float;
        return Rgb { data: *colour.as_ref() };
    }

    fn pixel_to_coord(&mut self, pixel_x: f32, pixel_y: f32) -> (f32, f32)
    {
        let centre_x = self.settings.width as f32 / 2.0;
        let centre_y = self.settings.height as f32 / 2.0;
        let unit_circle = centre_x.min(centre_y);

        return ((pixel_x - centre_x) / unit_circle,
                (centre_y - pixel_y) / unit_circle);
    }

    fn render_sample(&mut self, x: f32, y: f32) -> Colour
    {
        let mut ray = self.settings.scene.camera.compute_ray(x, y);
        let mut medium = Medium::default(); // TODO: Implement atmospheric media
        let mut colour_matrix = Trans::default();
        let mut reflection_count = 0u32;

        loop
        {
            // If the ray intersects something in the scene
            if let Some(intersection) = self.settings.scene.objects.find_intersection(ray)
            {
                // First process the emissive and absorptive media that we've travelled through
                match (medium.emission, medium.absorption)
                {
                    (Some(emission), Some(absorption)) => {
                        let distance_travelled = ray.direction.norm() * intersection.t_value;
                        // c := (c + e/k) * exp(kx) - e/k
                        // c := c * exp(kx) + e/k * exp(kx) - e/k
                        // Transformation is exp(kx) diagonal matrix
                        // Translation is (exp(kx) - 1) * e/k if k ≠ 0, e otherwise. We try to keep
                        // as much as possible of this operation vectorized.
                        let kx = absorption * distance_travelled;
                        let exp_kx = Vector::new(kx.x.exp(), kx.y.exp(), kx.z.exp());
                        let translation = (exp_kx + Vector::new(-1.0, -1.0, -1.0)) * (emission / absorption);
                        let translation = Vector::new(
                            if absorption.x == 0.0 { emission.x } else { translation.x },
                            if absorption.y == 0.0 { emission.y } else { translation.y },
                            if absorption.z == 0.0 { emission.z } else { translation.z },
                        );

                        colour_matrix = colour_matrix * Trans {
                            transformation: Matrix::from_diagonal(&exp_kx),
                            translation: translation,
                        };
                    },
                    (Some(emission), None) => {
                        let distance_travelled = ray.direction.norm() * intersection.t_value;
                        let colour_to_add = emission * distance_travelled;
                        // TODO: Optimization possible; use the fact that only the translation
                        // component changes. t := A * emission + t
                        colour_matrix = colour_matrix * Trans::new_translation_vector(colour_to_add);
                    },
                    (None, Some(absorption)) => {
                        let distance_travelled = ray.direction.norm() * intersection.t_value;
                        // c := exp(kx) * c
                        let kx = absorption * distance_travelled;
                        let exp_kx = Vector::new(kx.x.exp(), kx.y.exp(), kx.z.exp());
                        colour_matrix = colour_matrix * Trans::from_diagonal(exp_kx);
                    },
                    (None, None) => ()
                }

                // If there was a texture at the intersection
                if let Some(texture) = intersection.texture
                {
                    let interaction = texture.evaluate_texture(&mut self.rng, intersection.texture_point, ray.direction, intersection.normal);

                    // Accumulated colour matrix
                    colour_matrix = colour_matrix * interaction.colour_matrix;

                    // Stopping criteria:
                    // 1. If the aggregate colour transformation matrix is zero, we can stop immediately.
                    // 2. If we have surpassed 100 iterations, we can also stop.
                    // TODO: Implement more ray cancellation criteria
                    if na::is_zero(&colour_matrix.transformation) || reflection_count > 100
                    {
                        return colour_matrix.translation;
                    }

                    // Otherwise we spawn a child ray
                    ray = Ray::new(intersection.position, interaction.child_ray);
                    medium = if na::dot(&interaction.child_ray, &intersection.normal) > 0.0
                    {
                        intersection.outside
                    }
                    else
                    {
                        intersection.inside
                    };
                }

                // If there was no texture at the intersection just keep going in the same direction.
                else
                {
                    ray = Ray::new(intersection.position, ray.direction);
                }

                reflection_count += 1;
            }

            // If the ray doesn't intersect anything, return the background colour.
            else
            {
                // TODO: Implement procedural backgrounds in the Scene object.
                return colour_matrix.transform_colour(Colour::new(0.1, 0.1, 0.1));
                //return colour_matrix.transform_colour(Colour::new(0.0, 0.0, 0.0));
            }
        }
    }
}
