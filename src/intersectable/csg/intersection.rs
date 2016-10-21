use basics::*;
use std::iter::Iterator;

use intersectable::{Intersectable, Intersection, SolidIntersectable};

#[derive(Debug)]
pub struct CrossSection
{
    pub subobjects: Vec<Box<SolidIntersectable>>,
}

impl CrossSection
{
    pub fn new() -> CrossSection
    {
        CrossSection {
            subobjects: Vec::new(),
        }
    }
}

impl Intersectable for CrossSection
{
    fn find_intersection(&self, ray: Ray) -> Option<Intersection>
    {
        // TODO:
        // Allow for getting several intersections at once, for speed and accuracy
        // Allow for testing whether a point is inside an object
        // Possibly create a new trait for the above features (Solid?)
        // Then code this stuffs.

        let mut ray = ray;
        let mut result = None;

        for (i, subobj) in enumerate(&self.subobjects)
        {
            if let Some(intersection) = subobj.find_intersection(ray)
            {
                let is_inside_all_other_objects = enumerate(&self.subobjects).filter(|&(j, _)| i != j).all(|(_, other)| other.contains(intersection.position));
                if is_inside_all_other_objects
                {
                    ray.stop = intersection.t_value;
                    result = Some(intersection);
                }
            }
        }

        return result;
    }
}

impl SolidIntersectable for CrossSection
{
    fn contains(&self, point: Point) -> bool
    {
        self.subobjects.iter().all(|o| o.contains(point))
    }
}
