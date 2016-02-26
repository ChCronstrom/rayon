use super::*;
use basics::*;

use std;

pub trait Pigment: Function<Point, Colour> { }

impl<T: Function<Point, Colour>> Pigment for T { }

impl Function<Point, Colour> for Colour
{
    fn evaluate(&self, point: Point) -> Colour
    {
        let _ = point;

        *self
    }
}
