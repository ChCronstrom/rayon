use basics::*;

pub fn solve_quadratic(p: Float, q: Float) -> Option<(Float, Float)>
{
    // x = -p/2 ± sqrt(p²/4 - q)
    //   =    a ± sqrt(b)
    let a = p / -2.0;
    let b = a * a - q;

    if !(b > 0.0) || b == INFINITY
    {
        None
    }
    else
    {
        let sqrt_b = b.sqrt();
        Some((a - sqrt_b, a + sqrt_b))
    }
}
