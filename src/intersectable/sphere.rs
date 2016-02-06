use std;

use basics::{Ray};
use intersectable::{Intersection, Intersectable};

use na;
use na::Vec3;

#[derive(Debug)]
pub struct Sphere;

impl Intersectable for Sphere
{
    fn find_intersection(&self, ray : Ray) -> Option<Intersection>
    {
        // The basic sphere has the equation x² + y² + z² = 1.
        // The ray has equation (xyz) = origin + t * direction, for t in (start, stop).

        // Σ ((origin_i + t direction_i)²) = 1
        // Σ (origin_i² + 2 origin_i t direction_i + t² direction_i²) = 1
        // Σ (origin_i²) + 2t Σ (origin_i direction_i) + t² Σ (direction_i²) = 1
        //     [ where Σ (origin_i²) = norm²(origin) ]
        //     [ where Σ (origin_i direction_i) = dot(origin, direction) ]
        //     [ where Σ (direction_i²) = norm²(direction) ]
        // norm²(origin) + 2t dot(origin, direction) + t² norm²(direction) = 1
        // t² + 2t dot(origin, direction)/norm²(direction) + norm²(origin)/norm²(direction) - 1 = 0
        // t = - dot(origin, direction)/norm²(direction)
        //     ± sqrt( (dot(origin, direction)/norm²(direction))² - norm²(origin)/norm²(direction) + 1 )
        //     [ call these a ± sqrt(b) ]

        let norm_direction_sq = na::sqnorm(&ray.direction);
        let norm_origin_sq = na::sqnorm(ray.origin.as_vec());
        let origin_dot_direction = na::dot(&ray.direction, ray.origin.as_vec());

        // b = (dot(origin, direction)/norm²(direction))² - norm²(origin)/norm²(direction) + 1
        //   = dot(origin, direction)² / norm²(direction)² - norm²(origin)/norm²(direction) + 1
        //   = ( dot(origin, direction)² / norm²(direction) - norm²(origin) ) / norm²(direction) + 1
        let b = (origin_dot_direction * origin_dot_direction / norm_direction_sq - norm_origin_sq) / norm_direction_sq + 1.0;

        // This value will be positive for two intersections, zero or negative for no
        // intersections. It will be +inf, -inf, or NaN for dumb cases, which should return no
        // intersection. We test `!(b > 0)` instead of `b <= 0` to catch NaNs.
        if !(b > 0.0) || b == std::f32::INFINITY
        {
            return None;
        }

        let a = -origin_dot_direction / norm_direction_sq;
        let sqrt_b = b.sqrt();

        let (t1, t2) = (a - sqrt_b, a + sqrt_b);
        let t;
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

        return Some(Intersection
        {
            t_value: t,
            position: ray.evaluate(t),
            colour: Vec3::new(0.8, 0.0, 0.2),
        });
    }
}
