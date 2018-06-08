use basics::*;

use std;
use super::primitive::ElementalNoise;
use ::functions::Function;

#[derive(Clone, Copy)]
pub struct ScalarNoise
{
    noise: ElementalNoise,
    user_seed: u32,
    omega: Float,
}

impl ScalarNoise
{
    pub fn new(user_seed: u32, omega: Float) -> ScalarNoise
    {
        let noise = ElementalNoise::new(user_seed, 0xFFFFFFFFu32, omega);

        ScalarNoise {
            noise: noise,
            user_seed: user_seed,
            omega: omega,
        }
    }
}

impl Function<Point, Float> for ScalarNoise
{
    fn evaluate(&self, parameter: Point) -> Float
    {
        self.noise.evaluate(parameter)
    }
}

impl std::fmt::Debug for ScalarNoise
{
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> Result<(), std::fmt::Error>
    {
        fmt.debug_struct("ScalarNoise")
           .field("omega", &self.omega)
           .field("user_seed", &self.user_seed)
           .finish()
    }
}
