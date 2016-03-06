use std;

mod plane;
mod sphere;
mod textured;
mod transformed;
mod union;
mod cylinder;

pub use self::cylinder::Cylinder;
pub use self::plane::Plane;
pub use self::sphere::Sphere;
pub use self::textured::Textured;
pub use self::transformed::Transformed;
pub use self::union::Union;

use basics::*;
use medium::Medium;
use texture::TexturePoint;

#[derive(Debug)]
pub struct Intersection
{
    pub t_value: Float,
    pub position: Point,
    pub normal: Vector,
    pub texture: Option<Box<TexturePoint>>,
    pub outside: Medium,
    pub inside: Medium,
}

impl Intersection
{
    pub fn new(t_value: Float, position: Point, normal: Vector) -> Intersection
    {
        Intersection {
            t_value: t_value,
            position: position,
            normal: normal,
            texture: Default::default(),
            outside: Default::default(),
            inside: Default::default(),
        }
    }
}

pub trait Intersectable: std::fmt::Debug
{
    fn find_intersection(&self, ray: Ray) -> Option<Intersection>;
}
