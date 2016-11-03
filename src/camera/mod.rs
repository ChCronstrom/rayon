use basics::*;

use na::{Norm};

#[derive(Copy, Clone, Debug, Default)]
pub struct Camera
{
    pub transformation: Trans,
}

impl Camera
{
    pub fn from_position(position: Point, look_at: Point) -> Camera
    {
        Camera::from_position_and_sky_vector(position, look_at, Vector::new(0.0, 0.0, 1.0))
    }

    pub fn from_position_and_sky_vector(position: Point, look_at: Point, sky_vector: Vector) -> Camera
    {
        let transformation = Trans::new_from_orientation_and_sky(look_at - position, sky_vector).transformation;

        Camera
        {
            transformation: Trans
            {
                transformation: transformation,
                translation: position.to_vector(),
            }
        }
    }

    pub fn compute_ray(&self, x: f32, y: f32) -> Ray
    {
        Ray
        {
            origin: self.transformation.translation.to_point(),
            direction: (self.transformation * Vector::new(x, 1.0, y)).normalize(),
            start: EPSILON,
            stop: INFINITY,
        }
    }
}
