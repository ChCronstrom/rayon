use std;

mod plane;
mod sphere;
mod textured;
mod transformed;
mod union;

pub use self::plane::Plane;
pub use self::sphere::Sphere;
pub use self::textured::Textured;
pub use self::transformed::Transformed;
pub use self::union::Union;

use basics::*;
use texture::TexturePoint;

#[derive(Debug)]
pub struct Intersection
{
    pub t_value: Float,
    pub position: Point,
    pub normal: Vector,
    pub texture: Option<Box<TexturePoint>>,
}

pub trait Intersectable: std::fmt::Debug
{
    fn find_intersection(&self, ray: Ray) -> Option<Intersection>;
}
