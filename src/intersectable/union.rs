use basics::Ray;
use intersectable::{Intersectable, Intersection};

#[derive(Debug)]
pub struct Union
{
    pub subobjects: Vec<Box<Intersectable>>
}

impl Union
{
    pub fn new() -> Union
    {
        Union
        {
            subobjects : Vec::new(),
        }
    }
}

impl Intersectable for Union
{
    fn find_intersection(&self, ray : Ray) -> Option<Intersection>
    {
        let mut ray = ray;
        let mut result = None;
        for object in self.subobjects.iter()
        {
            if let Some(intersection) = object.find_intersection(ray)
            {
                result = Some(intersection);
                ray.stop = intersection.t_value;
            }
        }
        return result;
    }
}
