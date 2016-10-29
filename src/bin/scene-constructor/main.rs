extern crate rayon;

use std::fmt::{Display, Formatter, Result};

use rayon::{Colour, Float, PI, Trans, Vector};

#[derive(Debug)]
pub enum Intersectable
{
    CrossSection(Vec<Intersectable>),
    Cylinder,
    Plane,
    Sphere,
    Textured(Box<Intersectable>, Colour),
    Transformed(Box<Intersectable>, Trans),
    Union(Vec<Intersectable>),
}

struct VecPrinter<'a, T: 'a + Display>(&'a [T]);

use Intersectable::*;

impl Intersectable
{
    fn transform(self, trans: Trans) -> Intersectable
    {
        match self
        {
            Transformed(i, t) => Transformed(i, trans * t),
            x => Transformed(Box::new(x), trans),
        }
    }

    fn texture(self, texture: Colour) -> Intersectable
    {
        match self
        {
            t @ Textured(..) => t,
            //t @ Transformed(box Textured(..), _) => t,
            t => Textured(Box::new(t), texture),
        }
    }
}

impl Display for Intersectable
{
    fn fmt(&self, f: &mut Formatter) -> Result
    {
        use Intersectable::*;

        match self
        {
            &CrossSection(ref items) => write!(f, "CrossSection {{ subobjects: {} }}", VecPrinter(items)),
            &Cylinder => write!(f, "Cylinder"),
            &Plane => write!(f, "Plane"),
            &Sphere => write!(f, "Sphere"),
            &Transformed(ref i, ref t) => write!(f, "Transformed::new({}, {})", i, t),
            &Textured(ref i, ref t) => write!(f, "Textured::new({}, {})", i, t),
            &Union(ref items) => write!(f, "Union {{ subobjects: {} }}", VecPrinter(items)),
            //_ => unimplemented!()
        }
    }
}

impl<'a, T: Display> Display for VecPrinter<'a, T>
{
    fn fmt(&self, f: &mut Formatter) -> Result
    {
        try!(write!(f, "vec!["));

        for x in self.0
        {
            try!(write!(f, "Box::new({}), ", x));
        }

        write!(f, "]")
    }
}

pub fn scale(factor: Float) -> Trans
{
    scale_xyz(factor, factor, factor)
}

pub fn scale_xyz(x: Float, y: Float, z: Float) -> Trans
{
    Trans::from_diagonal(Vector::new(x, y, z))
}

pub fn translation<V: Into<Vector>>(v: V) -> Trans
{
    Trans::new_translation_vector(v.into())
}

pub fn rotation_x(angle: Float) -> Trans
{
    let cosa = angle.cos();
    let sina = angle.sin();

    Trans::new_rowwise(
        ((1.0, 0.0, 0.0),
         (0.0, cosa, -sina),
         (0.0, sina, cosa)))
}

pub fn rotation_y(angle: Float) -> Trans
{
    let cosa = angle.cos();
    let sina = angle.sin();

    Trans::new_rowwise(
        ((cosa, 0.0, sina),
         (0.0, 1.0, 0.0),
         (-sina, 0.0, cosa)))
}

pub fn rotation_z(angle: Float) -> Trans
{
    let cosa = angle.cos();
    let sina = angle.sin();

    Trans::new_rowwise(
        ((cosa, sina, 0.0),
         (-sina, cosa, 0.0),
         (0.0, 0.0, 1.0)))
}

fn cuboid() -> Intersectable
{
    let x_pos = Plane.transform(rotation_y(0.5 * PI)).transform(translation(&[1.0, 0.0, 0.0]));
    let x_neg = Plane.transform(rotation_y(-0.5 * PI)).transform(translation(&[-1.0, 0.0, 0.0]));

    let y_pos = Plane.transform(rotation_x(-0.5 * PI)).transform(translation(&[0.0, 1.0, 0.0]));
    let y_neg = Plane.transform(rotation_x(0.5 * PI)).transform(translation(&[0.0, -1.0, 0.0]));

    let z_pos = Plane.transform(translation(&[0.0, 0.0, 1.0]));
    let z_neg = Plane.transform(rotation_x(PI)).transform(translation(&[0.0, 0.0, -1.0]));

    CrossSection(vec![x_pos, x_neg, y_pos, y_neg, z_pos, z_neg])
}

fn main()
{
    //let b = cuboid().transform(rotation_z(0.25 * PI)).transform(rotation_x(0.25 * PI));
    let b = Plane.transform(rotation_z(0.1));
    println!("{}", b);
}
