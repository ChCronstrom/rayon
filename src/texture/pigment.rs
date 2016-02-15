use basics::*;

use std;

pub trait Pigment: std::fmt::Debug + Copy
{
    fn evaluate(&self, point: Point) -> Colour;
}

impl Pigment for Colour
{
    fn evaluate(&self, point: Point) -> Colour
    {
        let _ = point;

        *self
    }
}
