pub mod pigment;
mod chequered;
mod noise;

pub use self::chequered::Chequered;

use std;

// Use associated types instead?
pub trait Function<From, To>: std::fmt::Debug + Copy
{
    fn evaluate(&self, parameter: From) -> To;
}

// All compatible closures are valid functions too
impl<Args, Output, F: Fn(Args) -> Output + Copy + std::fmt::Debug> Function<Args, Output> for F
{
    fn evaluate(&self, parameter: Args) -> Output
    {
        (*self)(parameter)
    }
}
