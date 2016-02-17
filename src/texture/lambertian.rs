use basics::*;
use super::*;

use na::{Diag};

#[derive(Clone, Copy, Debug)]
pub struct Lambertian<P: Pigment>
{
    pub pigment: P,
}

#[derive(Clone, Copy, Debug)]
struct LambertianPoint<P: Pigment>
{
    pub pigment: P,
    pub location: Point,
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

impl<P: Pigment + 'static> Texture for Lambertian<P>
{
    fn evaluate_texture(&self, location: Point) -> Box<TexturePoint>
    {
        Box::new(LambertianPoint { pigment: self.pigment, location: location })
    }
}

impl<P: Pigment> TexturePoint for LambertianPoint<P>
{
    fn evaluate_texture_point(&self, rng: &mut RandomSource, incidence: Vector, normal: Vector) -> LightInteraction
    {
        let _ = incidence;
        let reflection_direction = weighted_rand_vector_on_half_sphere(rng, normal);
        let colour_filter = Matrix::from_diag(&self.pigment.evaluate(self.location));
        LightInteraction {
            colour_matrix: Trans {
                transformation: colour_filter,
                translation: Vector::new(0.0, 0.0, 0.0),
            },
            child_ray: reflection_direction,
        }
    }
}
