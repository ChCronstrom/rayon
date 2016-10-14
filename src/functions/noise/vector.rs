use super::*;
use basics::*;

use std;
use super::primitive::ElementalNoise;
use ::functions::Function;

#[derive(Clone, Copy)]
pub struct VectorNoise
{
    noise: [ElementalNoise; 3],
    user_seed: u32,
    omega: Float,
}

impl VectorNoise
{
    pub fn new(user_seed: u32, omega: Float) -> VectorNoise
    {
        let mut elemental_noises: [ElementalNoise; 3] = unsafe { std::mem::zeroed() };
        for i in 0..3
        {
            // TODO: Refactor this into a safe interface
            let noise = ElementalNoise::new(user_seed, i, omega);
            unsafe { std::ptr::write(&mut elemental_noises[i as usize], noise); }
        }

        VectorNoise {
            noise: elemental_noises,
            user_seed: user_seed,
            omega: omega,
        }
    }
}

impl Function<Point, Vector> for VectorNoise
{
    fn evaluate(&self, parameter: Point) -> Vector
    {
        Vector {
            x: self.noise[0].evaluate(parameter),
            y: self.noise[1].evaluate(parameter),
            z: self.noise[2].evaluate(parameter),
        }
    }
}

impl std::fmt::Debug for VectorNoise
{
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> Result<(), std::fmt::Error>
    {
        fmt.debug_struct("VectorNoise")
           .field("omega", &self.omega)
           .field("user_seed", &self.user_seed)
           .finish()
    }
}
