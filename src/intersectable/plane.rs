use intersectable::{Intersectable};
use basics::{Intersection, Ray};

pub struct Plane;

impl Intersectable for Plane
{
    fn find_intersection(&self, ray: Ray) -> Option<Intersection>
    {
        None
    }
}
