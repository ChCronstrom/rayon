use basics::*;
use super::{LightInteraction, TexturePoint};

use na;
use na::{Diag, Norm};
use rand;
use rand::Rand;

#[derive(Debug)]
pub struct Lambertian
{
    pub pigment: Colour,
}

impl Lambertian
{
    pub fn new(colour: Colour) -> Lambertian
    {
        Lambertian {
            pigment: colour,
        }
    }
}

impl TexturePoint for Lambertian
{
    fn evaluate_texture(&self, rng: &mut RandomSource, incidence: Vector, normal: Vector) -> LightInteraction
    {
        let reflection_direction = rand_vector_in_half_sphere(rng, normal).normalize();
        let colour_filter = Matrix::from_diag(&self.pigment);
        LightInteraction {
            colour_matrix: Trans {
                transformation: colour_filter,
                translation: Vector::new(0.0, 0.0, 0.0),
            },
            child_ray: reflection_direction,
        }
    }
}
