use std;

use basics::*;

use na;
use na::Diag;

#[derive(Copy, Clone, Debug)]
pub struct Trans
{
    pub transformation: Matrix,
    pub translation: Vector
}

impl Trans
{
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

    pub fn from_diagonal(diagnonal: Vector) -> Trans
    {
        Trans {
            transformation: Matrix::from_diag(&diagnonal),
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
        (self * rhs.to_pnt()).to_vec()
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
