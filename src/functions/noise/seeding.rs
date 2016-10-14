use super::*;
use basics::*;
use std::collections::BTreeMap;
use std::ops::DerefMut;
use std::sync::{Mutex, MutexGuard};
use noise::Seed;
use rand::{Rng, SeedableRng, XorShiftRng};

lazy_static! {
    static ref PERMUTATION_TABLES: Mutex<BTreeMap<[u32; 4], &'static Seed>> = {
        Mutex::new(BTreeMap::new())
    };
}

pub fn get_permutation_table(seed: [u32; 4]) -> &'static Seed
{
    // Lock the global list of cached permutation tables.
    let mut permutation_table_map = PERMUTATION_TABLES.lock().expect("Mutex has been poisoned.");

    // If there exists a cached permutation table for this seed, return that.
    match permutation_table_map.get(&seed) {
        Some(reference) => { return reference },
        None => { }
    }

    // Otherwise, generate a new permutation table using the provided seed in a XorShiftRng.
    let mut rng: XorShiftRng = SeedableRng::from_seed(seed);
    let permutation_table = rng.gen();

    // Make this permutation table live forever, so that we can keep static references to it.
    let reference = live_forever(permutation_table);

    // Return the static reference, and also keep it cached for future reference.
    permutation_table_map.insert(seed, reference);
    return reference;
}
