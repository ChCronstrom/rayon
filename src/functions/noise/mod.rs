/*

    What to do with the 3D OpenSimplex noise primitive, N(̄x).

    Seeds are [u32; 4], giving us four dimensions of seeding. One of these dimensions can be used
    for choosing an octave in multi-octave noise, one can be used to pick colour channel in multicolour
    noise, and one can be user-picked. All seed values cannot be simultaneously zero, so the fourth
    component will be kept at a non-zero constant.

    I'd really like the noise function to be parameterized by the number of octaves of noise it has.
    This could be done using typenum, but will probably be nicer in the future when Rust gets
    proper type-level integers. For now, we'll only use 4-octave noise.

    I'd really like the noise function to be able to operate on scalar, 2-vector, and 3-vector
    inputs at some point, but at the moment we keep it at 3-vectors for simplicity.

    I'd really like the noise function to be able to return scalars and 3-vectors.

    I'd really like a Pigment wrapper that generates colours.

*/

mod primitive;
mod seeding;
mod vector;

pub use self::vector::VectorNoise;
