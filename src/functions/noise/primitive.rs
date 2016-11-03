use basics::*;
use noise;
use std;

use super::seeding;

const NR_OF_OCTAVES: usize = 4;

#[derive(Clone, Copy)]
pub struct ElementalNoise
{
    // TODO: Find a better solution for seeding.
    // TODO: When scenes have resources, these permutation tables might live there instead.
    permutation_tables: [&'static noise::Seed; NR_OF_OCTAVES],
    user_seed: u32,
    channel_seed: u32,
    omega: Float,
}

impl ElementalNoise
{
    pub fn new(user_seed: u32, channel_seed: u32, omega: Float) -> ElementalNoise
    {
        assert!(omega > 0.0);
        assert!(omega <= 1.0);

        // "White" constants to reduce early bias in the RNG.
        // π = 0x3.243F6A88_85A308D3_13198A2E_03707344
        const A: u32 = 0x243F6A88_u32;
        const B: u32 = 0x85A308D3_u32;
        const C: u32 = 0x13198A2E_u32;
        const D: u32 = 0x03707344_u32;

        // TODO: Refactor this into a safe interface
        let mut permutation_tables: [&'static noise::Seed; NR_OF_OCTAVES] = unsafe { std::mem::zeroed() };
        for i in 0..4 {
            let seed_list = [A, B.wrapping_add(i), C.wrapping_add(channel_seed), D.wrapping_add(user_seed)];
            let permutation_table = seeding::get_permutation_table(seed_list);
            unsafe { std::ptr::write(&mut permutation_tables[i as usize], permutation_table); }
        }

        ElementalNoise {
            permutation_tables: permutation_tables,
            user_seed: user_seed,
            channel_seed: channel_seed,
            omega: omega,
        }
    }

    pub fn evaluate(&self, parameter: Point) -> Float
    {
        // Every octave has twice the frequency of the octave below, and omega times the amplitude.
        let mut amplitude: Float = 1.0;
        let mut point = parameter;
        let mut result: Float = 0.0;

        for i in 0..NR_OF_OCTAVES
        {
            let seed = self.permutation_tables[i];//.expect("Failed to find the permutation table.");
            result += amplitude * noise::open_simplex3(seed, point.as_ref());
            point = point * 2.0;
            amplitude *= self.omega;
        }

        result
    }
}

impl std::fmt::Debug for ElementalNoise
{
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> Result<(), std::fmt::Error>
    {
        // Unfortunately, ::noise::Seed doesn't implement Debug, so we must do this manually.
        fmt.debug_struct("ElementalNoise")
           .field("omega", &self.omega)
           .field("user_seed", &self.user_seed)
           .field("channel_seed", &self.channel_seed)
           .finish()
    }
}
