mod lambertian;

use std;

use basics::*;

pub use texture::lambertian::Lambertian;

#[derive(Copy, Clone, Debug)]
pub struct LightInteraction
{
    pub colour_matrix: Trans,
    pub child_ray: Vector,
}

pub trait TexturePoint: std::fmt::Debug
{
    fn evaluate_texture(&self, rng: &mut RandomSource, incidence: Vector, normal: Vector) -> LightInteraction;
}
