use basics::*;
use super::*;

use std;
use std::fmt::Debug;

#[derive(Clone, Copy, Debug)]
pub struct Checker<T>
{
    even: T,
    odd: T,
}

impl<T> Checker<T>
    where T: Copy + Debug
{
    pub fn new(even: T, odd: T) -> Checker<T>
    {
        Checker {
            even: even,
            odd: odd,
        }
    }
}

impl<T> Function<Point, T> for Checker<T>
    where T: Copy + Debug
{
    fn evaluate(&self, parameter: Point) -> T
    {
        let x_is_odd = parameter.x.floor() as i64 % 2 != 0;
        let y_is_odd = parameter.y.floor() as i64 % 2 != 0;
        let z_is_odd = (parameter.z + 1E-6).floor() as i64 % 2 != 0;

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
