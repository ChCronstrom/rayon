use basics::*;
use intersectable::{Intersectable, Intersection, SolidIntersectable};
use medium::Medium;
use texture::Texture;

#[derive(Debug)]
pub struct Textured<P, T>
{
    primitive: P,
    texture: T,
    inside: Medium,
    outside: Medium,
}

impl<P: Intersectable, T: Texture> Textured<P, T>
{
    pub fn new(primitive: P, texture: T) -> Textured<P, T>
    {
        Textured {
            primitive: primitive,
            texture: texture,
            inside: Default::default(),
            outside: Default::default(),
        }
    }

    pub fn new_with_media(primitive: P, texture: T, inside: Medium, outside: Medium) -> Textured<P, T>
    {
        Textured {
            primitive: primitive,
            texture: texture,
            inside: inside,
            outside: outside,
        }
    }
}

impl<P: Intersectable, T: Texture> Intersectable for Textured<P, T>
{
    fn find_intersection(&self, ray: Ray) -> Option<Intersection>
    {
        if let Some(mut intersection) = self.primitive.find_intersection(ray)
        {
            intersection.inside = self.inside;
            intersection.outside = self.outside;
            if intersection.texture.is_none()
            {
                intersection.texture = Some(&self.texture)
            }
            Some(intersection)
        }
        else
        {
            None
        }
    }
}

impl<P: SolidIntersectable, T: Texture> SolidIntersectable for Textured<P, T>
{
    fn contains(&self, point: Point) -> bool
    {
        self.primitive.contains(point)
    }
}
