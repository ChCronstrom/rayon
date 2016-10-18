mod emissive;
mod lambertian;
mod glass;

use std;

use basics::*;

pub use functions::pigment::Pigment;
pub use texture::emissive::Emissive;
pub use texture::lambertian::Lambertian;
pub use texture::glass::Glass;

#[derive(Copy, Clone, Debug)]
pub struct LightInteraction
{
    pub colour_matrix: Trans,
    pub child_ray: Vector,
}

impl LightInteraction
{
    pub fn new_uncoloured(child_ray: Vector) -> LightInteraction
    {
        LightInteraction {
            colour_matrix: Trans::default(),
            child_ray: child_ray,
        }
    }
}

pub trait Texture: std::fmt::Debug
{
    fn evaluate_texture(&self, rng: &mut RandomSource, location: Point, incidence: Vector, normal: Vector) -> LightInteraction;
}
