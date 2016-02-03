pub mod plane;

use basics::{Intersection, Ray};

trait Intersectable
{
    fn find_intersection(&self, ray : Ray) -> Option<Intersection>;
}
