use basics::*;
use intersectable::{Intersection, Intersectable};

use na;
use na::{Norm};

#[derive(Debug)]
pub struct Cylinder;

impl Intersectable for Cylinder
{
    fn find_intersection(&self, ray: Ray) -> Option<Intersection>
    {
        // The basic cylinder has equation x² + y² = 1, or x'Ax = 1 where A = diag(1, 1, 0).
        // The basic ray has equation x = o + dt.
        // The two equations together become
        // (o+dt)'A(o+dt) = 1, which expands to
        // o'Ao + 2 o'Adt + dt'Adt = 1
        // (d'Ad) t² + (2 o'Ad) t + (o'Ao - 1) = 0
        // Ax is just x with the z-component zeroed.
        // Thus
        let d_mod = Vector::new(ray.direction.x, ray.direction.y, 0.0);
        let o_mod = Vector::new(ray.origin.x, ray.origin.y, 0.0);

        let d_norm_sq = na::sqnorm(&d_mod);
        let o_norm_sq = na::sqnorm(&o_mod);
        let o_dot_d = na::dot(&o_mod, &d_mod);
        let p = (2.0 * o_dot_d) / d_norm_sq;
        let q = (o_norm_sq - 1.0) / d_norm_sq;

        let t;
        if let Some((t1, t2)) = solve_quadratic(p, q)
        {
            if (t1 > ray.start) && (t1 < ray.stop)
            {
                t = t1;
            }
            else if (t2 > ray.start) && (t2 < ray.stop)
            {
                t = t2;
            }
            else
            {
                return None;
            }
        }
        else
        {
            return None;
        }

        let position = ray.evaluate(t);
        let normal = {
            let mut normal = *position.as_vec();
            normal.z = 0.0;
            normal.normalize()
        };

        return Some(Intersection::new(t, position, normal));
    }
}
