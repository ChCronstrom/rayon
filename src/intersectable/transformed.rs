use basics::{Ray, Trans};
use intersectable::{Intersection, Intersectable};

use na::{Norm, Transpose};

#[derive(Debug)]
pub struct Transformed<T>
{
    primitive: T,
    transformation: Trans,
    inverse_transformation: Trans,
}

impl<T: Intersectable> Transformed<T>
{
    pub fn new(primitive: T, transformation: Trans) -> Transformed<T>
    {
        Transformed
        {
            transformation: transformation,
            inverse_transformation: transformation.invert(),
            primitive: primitive,
        }
    }
}

impl<T: Intersectable> Intersectable for Transformed<T>
{
    fn find_intersection(&self, ray: Ray) -> Option<Intersection>
    {
        if let Some(mut intersection) = self.primitive.find_intersection(self.inverse_transformation * ray)
        {
            intersection.position = self.transformation * intersection.position;
            intersection.normal = (self.inverse_transformation.transformation.transpose() * intersection.normal).normalize();
            Some(intersection)
        }
        else
        {
            None
        }
    }
}
