use basics::*;
use super::*;

use num::Zero;

#[derive(Clone, Copy, Debug)]
pub struct Emissive<P: Pigment>
{
    pub colour: P,
}

#[derive(Clone, Copy, Debug)]
struct EmissivePoint<P: Pigment>
{
    pub colour: P,
    pub location: Point,
}

impl<P: Pigment + 'static> Texture for Emissive<P>
{
    fn evaluate_texture_point(&self, location: Point) -> Box<TexturePoint>
    {
        Box::new(EmissivePoint { colour: self.colour, location: location })
    }
}

impl<P: Pigment> TexturePoint for EmissivePoint<P>
{
    fn evaluate_texture(&self, rng: &mut RandomSource, incidence: Vector, normal: Vector) -> LightInteraction
    {
        let _ = rng;
        let _ = incidence;
        let _ = normal;

        LightInteraction {
            colour_matrix: Trans {
                transformation: Matrix::zero(),
                translation: self.colour.evaluate(self.location),
            },
            child_ray: Vector::zero(),
        }
    }
}
