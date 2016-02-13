use std;

pub mod transformed;
pub mod plane;
pub mod sphere;
pub mod union;

use basics::*;
use texture::TexturePoint;

#[derive(Debug)]
pub struct Intersection
{
    pub t_value: f32,
	pub position: Point,
    pub normal: Vector,
    pub texture: Box<TexturePoint>,
}

pub trait Intersectable: std::fmt::Debug
{
    fn find_intersection(&self, ray: Ray) -> Option<Intersection>;
}
