mod chequered;
pub mod noise;
pub mod pigment;

pub use self::chequered::Chequered;

use std;

// Use associated types instead?
pub trait Function<From, To>: std::fmt::Debug
{
    fn evaluate(&self, parameter: From) -> To;
}

pub struct ClosureFunction<From, To, Closure: Fn(From) -> To>
{
    closure: Closure,
    _phantom: std::marker::PhantomData<(From, To)>,
}

impl<From, To, Closure: Fn(From) -> To> ClosureFunction<From, To, Closure>
{
    pub fn new(closure: Closure) -> ClosureFunction<From, To, Closure>
    {
        ClosureFunction {
            closure: closure,
            _phantom: std::marker::PhantomData,
        }
    }
}

impl<From, To, Closure: Fn(From) -> To> Function<From, To> for ClosureFunction<From, To, Closure>
{
    fn evaluate(&self, parameter: From) -> To
    {
        (self.closure)(parameter)
    }
}

impl<From, To, Closure: Fn(From) -> To> std::fmt::Debug for ClosureFunction<From, To, Closure>
{
    fn fmt(&self, f: &mut std::fmt::Formatter) ->std::fmt::Result
    {
        f.debug_struct("ClosureFunction")
         .finish()
    }
}
