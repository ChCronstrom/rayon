use basics::*;
use super::*;

use na::{Diag};

#[derive(Clone, Copy, Debug)]
pub struct Lambertian<P: Pigment>
{
    pub pigment: P,
}

impl<P: Pigment> Lambertian<P>
{
    pub fn new(pigment: P) -> Lambertian<P>
    {
        Lambertian {
            pigment: pigment,
        }
    }
}

impl<P: Pigment> Texture for Lambertian<P>
{
    fn evaluate_texture(&self, rng: &mut RandomSource, location: Point, incidence: Vector, normal: Vector) -> LightInteraction
    {
        let _ = incidence;
        let reflection_direction = weighted_rand_vector_on_half_sphere(rng, normal);
        let colour_filter = Matrix::from_diag(&self.pigment.evaluate(location));
        LightInteraction {
            colour_matrix: Trans {
                transformation: colour_filter,
                translation: Vector::new(0.0, 0.0, 0.0),
            },
            child_ray: reflection_direction,
        }
    }
}
