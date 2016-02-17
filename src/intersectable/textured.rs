use basics::*;
use intersectable::{Intersectable, Intersection};
use texture::Texture;

#[derive(Debug)]
pub struct Textured<P, T>
{
    primitive: P,
    texture: T,
}

impl<P: Intersectable, T: Texture> Textured<P, T>
{
    pub fn new(primitive: P, texture: T) -> Textured<P, T>
    {
        Textured {
            primitive: primitive,
            texture: texture,
        }
    }
}

impl<P: Intersectable, T: Texture> Intersectable for Textured<P, T>
{
    fn find_intersection(&self, ray: Ray) -> Option<Intersection>
    {
        if let Some(mut intersection) = self.primitive.find_intersection(ray)
        {
            if intersection.texture.is_none()
            {
                intersection.texture = Some(self.texture.evaluate_texture(intersection.position))
            }
            Some(intersection)
        }
        else
        {
            None
        }
    }
}
