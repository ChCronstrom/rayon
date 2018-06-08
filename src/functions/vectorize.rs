use basics::*;
use super::*;

use std::fmt::Debug;
use std::marker::PhantomData;

#[derive(Debug)]
pub struct Vectorize<From: Debug, F: Function<From, Float>>
{
    func: F,
    _phantom: PhantomData<Function<From, Float>>,
}

impl<From: Debug, F: Function<From, Float>> Vectorize<From, F>
{
    pub fn new(scalar_function: F) -> Vectorize<From, F>
    {
        Vectorize {
            func: scalar_function,
            _phantom: PhantomData,
        }
    }
}

impl<From: Debug, F: Function<From, Float>> Function<From, Vector> for Vectorize<From, F>
{
    fn evaluate(&self, f: From) -> Vector
    {
        let value = self.func.evaluate(f);
        Vector::new(value, value, value)
    }
}
