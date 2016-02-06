use std;

use basics::{Ray, Trans};

use na;
use na::{Mat3, Norm, Pnt3, Vec3};

#[derive(Copy, Clone, Debug, Default)]
pub struct Camera
{
    pub transformation: Trans,
}

impl Camera
{
    pub fn from_position(position: Pnt3<f32>, look_at: Pnt3<f32>) -> Camera
    {
        Camera::from_position_and_sky_vector(position, look_at, Vec3::new(0.0, 0.0, 1.0))
    }

    pub fn from_position_and_sky_vector(position: Pnt3<f32>, look_at: Pnt3<f32>, sky_vector: Vec3<f32>) -> Camera
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

        let transformation = Mat3::new(x_direction.x, y_direction.x, z_direction.x,
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
