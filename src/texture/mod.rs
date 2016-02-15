mod emissive;
mod lambertian;
mod pigment;

use std;

use basics::*;

pub use texture::emissive::Emissive;
pub use texture::lambertian::Lambertian;
pub use texture::pigment::Pigment;

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

pub trait Texture: std::fmt::Debug
{
    fn evaluate_texture_point(&self, location: Point) -> Box<TexturePoint>;
}
