mod ray;
mod solvers;
mod trans;

pub use self::trans::Trans;
pub use self::ray::Ray;
pub use self::solvers::solve_quadratic;

use std;
use image;
use na;
use na::{Inverse, Matrix3, Norm, Vector3, Point3};
use rand;
use rand::Rand;

pub type Float = f32;
pub type Vector = Vector3<Float>;
pub type Point = Point3<Float>;
pub type Matrix = Matrix3<Float>;
pub type Colour = Vector3<Float>;
pub type HdrImage = image::ImageBuffer<image::Rgb<Float>, Vec<Float>>;
pub type RandomSource = rand::Isaac64Rng;

pub use std::f32::EPSILON;
pub use std::f32::INFINITY;
pub use std::f32::NAN;
pub use std::f32::consts::PI;

pub fn rand_vector_in_sphere<R: rand::Rng>(rng: &mut R) -> Vector
{
    loop
    {
        let (x, y, z): (Float, Float, Float) = Rand::rand(rng);
        let result = Vector::new(2.0 * x - 1.0, 2.0 * y - 1.0, 2.0 * z - 1.0);
        if result.norm_squared() < 1.0
        {
            return result;
        }
    }
}

#[cfg(test)]
fn alternate_rand_vector_in_sphere<R: rand::Rng>(rng: &mut R) -> Vector
{
    let theta: Float = PI * (Rand::rand(rng) - 0.5);
    let phi: Float = 2.0 * PI * Rand::rand(rng);
    let r: Float = Rand::rand(rng).cbrt();

    Vector::new(
        r * theta.cos() * phi.sin(),
        r * theta.cos() * phi.cos(),
        r * theta.sin()
    )
}

pub fn rand_vector_in_half_sphere<R: rand::Rng>(rng: &mut R, direction: Vector) -> Vector
{
    // If dot(randvec, direction) is negative, it's pointing the wrong way. dot(randvec, direction) * normal
    // gives by how much, and so randvec - 2 * dot(randvec, direction) * direction will make it
    // point the right way. In the unlikely case that the dot product is exactly zero, we draw a
    // new random vector.

    loop
    {
        let rand_vector = rand_vector_in_sphere(rng);
        let dot_product = na::dot(&rand_vector, &direction);
        if dot_product < 0.0
        {
            return rand_vector + direction * (-2.0 * dot_product);
        }
        else if dot_product > 0.0
        {
            return rand_vector;
        }
    }
}

pub fn weighted_rand_vector_on_half_sphere<R: rand::Rng>(rng: &mut R, direction: Vector) -> Vector
{
    // FIXME: This is probably slow.
    loop
    {
        let rand_vector = rand_vector_in_half_sphere(rng, direction).normalize();
        let dot_product = na::dot(&rand_vector, &direction);
        let breakoff_point: Float = Rand::rand(rng);
        if dot_product >= breakoff_point
        {
            return rand_vector;
        }
    }
}

pub fn invert(m: Matrix) -> Matrix
{
    Inverse::inverse(&m).unwrap_or(Matrix::new(NAN, NAN, NAN, NAN, NAN, NAN, NAN, NAN, NAN))
}

pub fn calculate_projection_rejection(vector: Vector, project_onto: Vector) -> (Float, Vector, Vector)
{
    let dot = na::dot(&vector, &project_onto);
    let cosine = dot / na::norm(&vector);
    let projection = project_onto * cosine;
    let rejection = vector - projection;
    return (dot, projection, rejection);
}

/// Make a value live forever, returning a static reference to it.
///
/// The returned immutable reference has static lifetime, and will live for the remainder of the
/// program execution. If the last reference is dropped, the memory held by the value will leak.
pub fn live_forever<T>(value: T) -> &'static T
{
    let boxed = Box::new(value);
    let raw_pointer = Box::into_raw(boxed);
    unsafe { &*raw_pointer }
}

pub fn enumerate<T: IntoIterator>(iterator: T) -> std::iter::Enumerate<<T as IntoIterator>::IntoIter>
{
    iterator.into_iter().enumerate()
}

#[cfg(test)]
mod tests
{
    use super::*;

    use na::ApproxEq;
    use rand;
    use rand::Rng;

    #[test]
    fn test_rand_vector_in_sphere()
    {
        let mut randomizer = rand::thread_rng();
        for _ in 0..100
        {
            let random_vector = rand_vector_in_sphere(&mut randomizer);
            assert!(random_vector.norm() < 1.0);
        }
    }

    #[test]
    fn test_rand_vector_in_half_sphere()
    {
        let mut randomizer = rand::thread_rng();

        let direction = Vector::new(1.0, 0.0, 0.0);
        for _ in 0..100
        {
            let random_vector = rand_vector_in_half_sphere(&mut randomizer, direction);
            assert!(random_vector.norm() < 1.0);
            assert!(random_vector.x > 0.0);
        }

        let direction = Vector::new(-1.0, 0.0, 0.0);
        for _ in 0..100
        {
            let random_vector = rand_vector_in_half_sphere(&mut randomizer, direction);
            assert!(random_vector.norm() < 1.0);
            assert!(random_vector.x < 0.0);
        }

        let direction = Vector::new(0.0, 1.0, 0.0);
        for _ in 0..100
        {
            let random_vector = rand_vector_in_half_sphere(&mut randomizer, direction);
            assert!(random_vector.norm() < 1.0);
            assert!(random_vector.y > 0.0);
        }

        let direction = Vector::new(0.0, -1.0, 0.0);
        for _ in 0..100
        {
            let random_vector = rand_vector_in_half_sphere(&mut randomizer, direction);
            assert!(random_vector.norm() < 1.0);
            assert!(random_vector.y < 0.0);
        }

        let direction = Vector::new(0.0, 0.0, 1.0);
        for _ in 0..100
        {
            let random_vector = rand_vector_in_half_sphere(&mut randomizer, direction);
            assert!(random_vector.norm() < 1.0);
            assert!(random_vector.z > 0.0);
        }

        let direction = Vector::new(0.0, 0.0, -1.0);
        for _ in 0..100
        {
            let random_vector = rand_vector_in_half_sphere(&mut randomizer, direction);
            assert!(random_vector.norm() < 1.0);
            assert!(random_vector.z < 0.0);
        }
    }

    #[test]
    fn test_invert()
    {
        let mut randomizer = rand::thread_rng();

        for _ in 0..100
        {
            let random_matrix = Matrix::new(2.0 * randomizer.next_f32() - 1.0,
                                            2.0 * randomizer.next_f32() - 1.0,
                                            2.0 * randomizer.next_f32() - 1.0,
                                            2.0 * randomizer.next_f32() - 1.0,
                                            2.0 * randomizer.next_f32() - 1.0,
                                            2.0 * randomizer.next_f32() - 1.0,
                                            2.0 * randomizer.next_f32() - 1.0,
                                            2.0 * randomizer.next_f32() - 1.0,
                                            2.0 * randomizer.next_f32() - 1.0);

            let determinant = na::determinant(&random_matrix);
            if determinant < 0.001 && determinant > -0.001
            {
                continue;
            }

            let inverse = invert(random_matrix);
            let identity = random_matrix * inverse;

            assert!(identity.m11.approx_eq_eps(&1.0, &1.0e-4));
            assert!(identity.m22.approx_eq_eps(&1.0, &1.0e-4));
            assert!(identity.m33.approx_eq_eps(&1.0, &1.0e-4));

            assert!(identity.m12.approx_eq_eps(&0.0, &1.0e-4));
            assert!(identity.m13.approx_eq_eps(&0.0, &1.0e-4));
            assert!(identity.m21.approx_eq_eps(&0.0, &1.0e-4));
            assert!(identity.m23.approx_eq_eps(&0.0, &1.0e-4));
            assert!(identity.m31.approx_eq_eps(&0.0, &1.0e-4));
            assert!(identity.m32.approx_eq_eps(&0.0, &1.0e-4));
        }
    }
}
