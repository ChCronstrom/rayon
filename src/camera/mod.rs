use std;

use basics::*;

use na;
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
        // The y-direction (forwards) of the camera is the vector from `position` to `look_at`,
        // normalized.
        let y_direction = (look_at - position).normalize();

        // The x-direction (right) of the camera is the right-hand perpendicular of y and
        // `sky_vector`.
        let x_direction = na::cross(&y_direction, &sky_vector).normalize();

        // The z-direction (up) of the camera is x cross y, which is in the plane of y and
        // `sky_vector`.
        let z_direction = na::cross(&x_direction, &y_direction).normalize();

        let transformation = Matrix::new(x_direction.x, y_direction.x, z_direction.x,
                                         x_direction.y, y_direction.y, z_direction.y,
                                         x_direction.z, y_direction.z, z_direction.z);

        Camera
        {
            transformation: Trans
            {
                transformation: transformation,
                translation: position.to_vec(),
            }
        }
    }

    pub fn compute_ray(&self, x: f32, y: f32) -> Ray
    {
        Ray
        {
            origin: self.transformation.translation.to_pnt(),
            direction: (self.transformation * na::Vec3::new(x, 1.0, y)).normalize(),
            start: std::f32::EPSILON,
            stop: std::f32::INFINITY,
        }
    }
}
