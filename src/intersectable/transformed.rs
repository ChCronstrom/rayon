use basics::{Ray, Trans};
use intersectable::{Intersection, Intersectable};

#[derive(Debug)]
pub struct Transformed<T>
{
    pub inverse_transformation : Trans,
    pub primitive : T,
}

impl<T : Intersectable> Transformed<T>
{
    pub fn new(primitive : T, transformation: Trans) -> Transformed<T>
    {
        Transformed
        {
            inverse_transformation: transformation.invert(),
            primitive: primitive,
        }
    }
}

impl<T : Intersectable> Intersectable for Transformed<T>
{
    fn find_intersection(&self, ray : Ray) -> Option<Intersection>
    {
        self.primitive.find_intersection(self.inverse_transformation * ray)
    }
}
