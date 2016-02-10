use std;

pub mod transformed;
pub mod plane;
pub mod sphere;
pub mod union;

use basics::{Ray};

use na::{Pnt3, Vec3};

#[derive(Copy, Clone, Debug)]
pub struct Intersection {
    pub t_value: f32,
	pub position: Pnt3<f32>,
    pub colour: Vec3<f32>,
}

pub trait Intersectable : std::fmt::Debug
{
    fn find_intersection(&self, ray : Ray) -> Option<Intersection>;
}
