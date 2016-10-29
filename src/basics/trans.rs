use std;

use basics::*;

use na;
use na::{Diagonal, Norm};
use num::Zero;

#[derive(Copy, Clone, Debug)]
pub struct Trans
{
    pub transformation: Matrix,
    pub translation: Vector
}

impl Trans
{
    pub fn new(((m11, m21, m31), (m12, m22, m32), (m13, m23, m33)): ((Float, Float, Float), (Float, Float, Float), (Float, Float, Float)),
               (x, y, z): (Float, Float, Float)) -> Trans
    {
        Trans {
            transformation: Matrix::new(m11, m21, m31, m12, m22, m32, m13, m23, m33),
            translation: Vector::new(x, y, z),
        }
    }

    pub fn new_columnwise(((m11, m21, m31), (m12, m22, m32), (m13, m23, m33)): ((Float, Float, Float), (Float, Float, Float), (Float, Float, Float))) -> Trans
    {
        Trans {
            transformation: Matrix::new(m11, m21, m31, m12, m22, m32, m13, m23, m33),
            translation: Vector::zero(),
        }
    }

    pub fn new_rowwise(((m11, m12, m13), (m21, m22, m23), (m31, m32, m33)): ((Float, Float, Float), (Float, Float, Float), (Float, Float, Float))) -> Trans
    {
        Trans {
            transformation: Matrix::new(m11, m21, m31, m12, m22, m32, m13, m23, m33),
            translation: Vector::zero(),
        }
    }

    pub fn new_translation(x: Float, y: Float, z: Float) -> Trans
    {
        Trans::new_translation_vector(Vector::new(x, y, z))
    }

    pub fn new_translation_vector(v: Vector) -> Trans
    {
        let mut result = Trans::default();
        result.translation = v;
        return result;
    }

    pub fn new_from_orientation_and_sky(orientation: Vector, sky_vector: Vector) -> Trans
    {
        // The y-direction (forwards) of the camera is the vector from `position` to `look_at`,
        // normalized.
        let y_direction = orientation.normalize();

        // The x-direction (right) of the camera is the right-hand perpendicular of y and
        // `sky_vector`.
        let x_direction = na::cross(&y_direction, &sky_vector).normalize();

        // The z-direction (up) of the camera is x cross y, which is in the plane of y and
        // `sky_vector`.
        let z_direction = na::cross(&x_direction, &y_direction).normalize();

        let transformation = Matrix::new(x_direction.x, y_direction.x, z_direction.x,
                                         x_direction.y, y_direction.y, z_direction.y,
                                         x_direction.z, y_direction.z, z_direction.z);

        Trans {
            transformation: transformation,
            translation: Vector::zero(),
        }
    }

    pub fn from_diagonal(diagnonal: Vector) -> Trans
    {
        Trans {
            transformation: Matrix::from_diagonal(&diagnonal),
            translation: na::zero(),
        }
    }

    pub fn invert(self) -> Trans
    {
        // y = Ax + t
        // Ax = y - t
        // x = A \ (y - t) = A\y - A\t
        let inv = invert(self.transformation);
        Trans {
            transformation: inv,
            translation: -(inv * self.translation),
        }
    }

    pub fn transform_colour(self, rhs: Colour) -> Colour
    {
        (self * rhs.to_point()).to_vector()
    }
}

impl std::ops::Mul<Vector> for Trans
{
    type Output = Vector;

    fn mul(self, rhs: Vector) -> Vector
    {
        self.transformation * rhs
    }
}

impl std::ops::Mul<Point> for Trans
{
    type Output = Point;

    fn mul(self, rhs: Point) -> Point
    {
        self.transformation * rhs + self.translation
    }
}

impl std::ops::Mul<Ray> for Trans
{
    type Output = Ray;

    fn mul(self, rhs: Ray) -> Ray
    {
        Ray
        {
            origin: self * rhs.origin,
            direction: self * rhs.direction,
            start: rhs.start,
            stop: rhs.stop,
        }
    }
}

impl std::ops::Mul<Trans> for Trans
{
    type Output = Trans;

    fn mul(self, rhs: Trans) -> Trans
    {
        // T1 * T2 * v means
        // A1 * (A2 * v + t2) + t1
        // A1 * A2 * v + A1 * t2 + t1
        // So new transform is A1 * A2, and new translation is A1 * t2 + t1.
        Trans {
            transformation: self.transformation * rhs.transformation,
            translation: self.transformation * rhs.translation + self.translation,
        }
    }
}

impl Default for Trans
{
    fn default() -> Trans
    {
        Trans
        {
            transformation: na::Eye::new_identity(3),
            translation: na::zero(),
        }
    }
}

impl std::fmt::Display for Trans
{
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result
    {
        write!(f, "Trans::new((({:.4}, {:.4}, {:.4}), ({:.4}, {:.4}, {:.4}), ({:.4}, {:.4}, {:.4})), ({:.4}, {:.4}, {:.4}))",
               self.transformation.m11, self.transformation.m21, self.transformation.m31,
               self.transformation.m12, self.transformation.m22, self.transformation.m32,
               self.transformation.m13, self.transformation.m23, self.transformation.m33,
               self.translation.x, self.translation.y, self.translation.z
        )
    }
}
