use basics::*;
use super::*;

use num::Zero;

#[derive(Clone, Debug)]
pub struct Emissive<P: Pigment>
{
    pub colour: P,
}

impl<P: Pigment> Texture for Emissive<P>
{
    fn evaluate_texture(&self, rng: &mut RandomSource, location: Point, incidence: Vector, normal: Vector) -> LightInteraction
    {
        let _ = rng;
        let _ = incidence;
        let _ = normal;

        LightInteraction {
            colour_matrix: Trans {
                transformation: Matrix::zero(),
                translation: self.colour.evaluate(location),
            },
            child_ray: Vector::zero(),
        }
    }
}
