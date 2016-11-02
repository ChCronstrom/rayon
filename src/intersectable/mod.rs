use std;

pub mod csg;
mod cylinder;
mod plane;
mod sphere;
mod textured;
mod transformed;
mod union;

pub use self::cylinder::Cylinder;
pub use self::plane::Plane;
pub use self::sphere::Sphere;
pub use self::textured::Textured;
pub use self::transformed::Transformed;
pub use self::union::Union;
pub use self::csg::CrossSection;

use basics::*;
use medium::Medium;
use texture::Texture;

#[derive(Debug)]
pub struct Intersection<'a>
{
    pub t_value: Float,
    pub position: Point,
    pub normal: Vector,
    pub texture: Option<&'a Texture>,
    pub texture_point: Point,
    pub outside: Medium,
    pub inside: Medium,
}

impl<'a> Intersection<'a>
{
    pub fn new(t_value: Float, position: Point, normal: Vector) -> Intersection<'a>
    {
        Intersection {
            t_value: t_value,
            position: position,
            normal: normal,
            texture: Default::default(),
            texture_point: position,
            outside: Default::default(),
            inside: Default::default(),
        }
    }
}

pub trait Intersectable: std::fmt::Debug + Sync
{
    fn find_intersection(&self, ray: Ray) -> Option<Intersection>;
}

pub trait SolidIntersectable: Intersectable
{
    fn contains(&self, point: Point) -> bool;
}
