use basics::*;
use super::*;

use std::fmt::Debug;

#[derive(Clone, Copy, Debug)]
pub struct Chequered<T>
{
    even: T,
    odd: T,
}

impl<T> Chequered<T>
    where T: Copy + Debug
{
    pub fn new(even: T, odd: T) -> Chequered<T>
    {
        Chequered {
            even: even,
            odd: odd,
        }
    }
}

impl<T> Function<Point, T> for Chequered<T>
    where T: Copy + Debug
{
    fn evaluate(&self, parameter: Point) -> T
    {
        let x_is_odd = parameter.x.round() as i64 % 2 != 0;
        let y_is_odd = parameter.y.round() as i64 % 2 != 0;
        let z_is_odd = parameter.z.round() as i64 % 2 != 0;

        if x_is_odd ^ y_is_odd ^ z_is_odd
        {
            self.odd
        }
        else
        {
            self.even
        }
    }
}
