use basics::*;
use functions::*;

struct Gradient<T>
{
    source_function: T,
    steps: Box<[(Float, Colour)]>,
}

// impl<T> Function<T, Colour> for Gradient<T>
//     where T: Function<T, Float>
// {
//
// }
