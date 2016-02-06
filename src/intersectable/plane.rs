use basics::{Ray};
use intersectable::{Intersection, Intersectable};

use na::Vec3;

#[derive(Debug)]
pub struct Plane;

impl Intersectable for Plane
{
    fn find_intersection(&self, ray: Ray) -> Option<Intersection>
    {
        // The basic plane has the equation z = 0.
        // The ray has equation (xyz) = origin + t * direction, for t in (start, stop).
        // origin_z + t * direction_z = 0 => t = - origin_z / direction_z

        // If ray.direction.z is exactly zero, t_solution will be either -inf, +inf, or NaN. In
        // this case one or both of the comparisons will always return false, which is what we
        // want. Parallel rays never intersect, even if it runs exactly along the plane.
        let t_solution = -ray.origin.z / ray.direction.z;
        if (t_solution > ray.start) && (t_solution < ray.stop)
        {
            Some(Intersection
            {
                t_value: t_solution,
                position: ray.evaluate(t_solution),
                colour: Vec3::new(0.0, 0.2, 0.8)
            })
        }
        else
        {
            None
        }
    }
}
