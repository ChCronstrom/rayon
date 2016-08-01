use std::hash::{Hash, Hasher, SipHasher};

use basics::*;

use functions::Function;

#[derive(Debug)]
struct SimplexNoise
{
    seed: u64,
}

const STRETCH_CONSTANT_3D: Float = -1.0 / 6.0; //(1/Math.sqrt(3+1)-1)/3;
const SQUISH_CONSTANT_3D: Float = 1.0 / 3.0;   //(Math.sqrt(3+1)-1)/3;
const NORM_CONSTANT_3D: Float = 103.0;

impl SimplexNoise
{
    pub fn new(seed: u64) -> SimplexNoise
    {
        SimplexNoise {
            seed: seed,
        }
    }

    fn extrapolate(xsb: i32, ysb: i32, zsb: i32, dx: Float, dy: Float, dz: Float) -> Float
    {
        let index = permGradIndex3D[(perm[(perm[xsb & 0xFF] + ysb) & 0xFF] + zsb) & 0xFF];
        return gradients3D[index] * dx
            + gradients3D[index + 1] * dy
            + gradients3D[index + 2] * dz;
    }

    fn vertex_value(&self, points: &[u64]) -> u64
    {
        let mut hasher = SipHasher::new_with_keys(self.seed, 0);
        points.hash(&mut hasher);
        hasher.finish()
    }
}

impl Function<Point, Float> for SimplexNoise
{
    fn evaluate(&self, point: Point) -> Float
    {
        let x = point.x;
        let y = point.y;
        let z = point.z;

        //Place input coordinates on simplectic honeycomb.
        let stretchOffset = (x + y + z) * STRETCH_CONSTANT_3D;
        let xs = x + stretchOffset;
        let ys = y + stretchOffset;
        let zs = z + stretchOffset;

        //Floor to get simplectic honeycomb coordinates of rhombohedron (stretched cube) super-cell origin.
        let xsb = xs.floor();
        let ysb = ys.floor();
        let zsb = zs.floor();

        //Skew out to get actual coordinates of rhombohedron origin. We'll need these later.
        let squishOffset = (xsb + ysb + zsb) * SQUISH_CONSTANT_3D;
        let xb = xsb + squishOffset;
        let yb = ysb + squishOffset;
        let zb = zsb + squishOffset;

        //Compute simplectic honeycomb coordinates relative to rhombohedral origin.
        let xins = xs - xsb;
        let yins = ys - ysb;
        let zins = zs - zsb;

        //Sum those together to get a value that determines which region we're in.
        let inSum = xins + yins + zins;

        //Positions relative to origin point.
        let dx0 = x - xb;
        let dy0 = y - yb;
        let dz0 = z - zb;

        //We'll be defining these inside the next block and using them afterwards.
        let dx_ext0;
        let dy_ext0;
        let dz_ext0;
        let dx_ext1;
        let dy_ext1;
        let dz_ext1;
        let xsv_ext0;
        let ysv_ext0;
        let zsv_ext0;
        let xsv_ext1;
        let ysv_ext1;
        let zsv_ext1;

        let mut value: Float = 0.0;

        if inSum <= 1 // We're inside the tetrahedron (3-Simplex) at (0,0,0)
        {
			//Determine which two of (0,0,1), (0,1,0), (1,0,0) are closest.
			let mut aPoint = 0x01;
			let mut aScore = xins;
			let mut bPoint = 0x02;
			let mut bScore = yins;
			if aScore >= bScore && zins > bScore {
				bScore = zins;
				bPoint = 0x04;
			} else if aScore < bScore && zins > aScore {
				aScore = zins;
				aPoint = 0x04;
			}

			//Now we determine the two lattice points not part of the tetrahedron that may contribute.
			//This depends on the closest two tetrahedral vertices, including (0,0,0)
			let wins = 1.0 - inSum;
			if wins > aScore || wins > bScore { //(0,0,0) is one of the closest two tetrahedral vertices.
				let c = if bScore > aScore { bPoint } else { aPoint }; //Our other closest vertex is the closest out of a and b.

				if (c & 0x01) == 0 {
					xsv_ext0 = xsb - 1;
					xsv_ext1 = xsb;
					dx_ext0 = dx0 + 1;
					dx_ext1 = dx0;
				} else {
					xsv_ext0 = xsv_ext1 = xsb + 1;
					dx_ext0 = dx_ext1 = dx0 - 1;
				}

				if (c & 0x02) == 0 {
					ysv_ext0 = ysv_ext1 = ysb;
					dy_ext0 = dy_ext1 = dy0;
					if (c & 0x01) == 0 {
						ysv_ext1 -= 1;
						dy_ext1 += 1;
					} else {
						ysv_ext0 -= 1;
						dy_ext0 += 1;
					}
				} else {
					ysv_ext0 = ysv_ext1 = ysb + 1;
					dy_ext0 = dy_ext1 = dy0 - 1;
				}

				if (c & 0x04) == 0 {
					zsv_ext0 = zsb;
					zsv_ext1 = zsb - 1;
					dz_ext0 = dz0;
					dz_ext1 = dz0 + 1;
				} else {
					zsv_ext0 = zsv_ext1 = zsb + 1;
					dz_ext0 = dz_ext1 = dz0 - 1;
				}
			} else { //(0,0,0) is not one of the closest two tetrahedral vertices.
				let c = aPoint | bPoint; //Our two extra vertices are determined by the closest two.

				if (c & 0x01) == 0 {
					xsv_ext0 = xsb;
					xsv_ext1 = xsb - 1;
					dx_ext0 = dx0 - 2 * SQUISH_CONSTANT_3D;
					dx_ext1 = dx0 + 1 - SQUISH_CONSTANT_3D;
				} else {
					xsv_ext0 = xsv_ext1 = xsb + 1;
					dx_ext0 = dx0 - 1 - 2 * SQUISH_CONSTANT_3D;
					dx_ext1 = dx0 - 1 - SQUISH_CONSTANT_3D;
				}

				if (c & 0x02) == 0 {
					ysv_ext0 = ysb;
					ysv_ext1 = ysb - 1;
					dy_ext0 = dy0 - 2 * SQUISH_CONSTANT_3D;
					dy_ext1 = dy0 + 1 - SQUISH_CONSTANT_3D;
				} else {
					ysv_ext0 = ysv_ext1 = ysb + 1;
					dy_ext0 = dy0 - 1 - 2 * SQUISH_CONSTANT_3D;
					dy_ext1 = dy0 - 1 - SQUISH_CONSTANT_3D;
				}

				if (c & 0x04) == 0 {
					zsv_ext0 = zsb;
					zsv_ext1 = zsb - 1;
					dz_ext0 = dz0 - 2 * SQUISH_CONSTANT_3D;
					dz_ext1 = dz0 + 1 - SQUISH_CONSTANT_3D;
				} else {
					zsv_ext0 = zsv_ext1 = zsb + 1;
					dz_ext0 = dz0 - 1 - 2 * SQUISH_CONSTANT_3D;
					dz_ext1 = dz0 - 1 - SQUISH_CONSTANT_3D;
				}
			}

			//Contribution (0,0,0)
			let attn0 = 2.0 - dx0 * dx0 - dy0 * dy0 - dz0 * dz0;
			if attn0 > 0.0 {
				attn0 *= attn0;
				value += attn0 * attn0 * extrapolate(xsb + 0, ysb + 0, zsb + 0, dx0, dy0, dz0);
			}

			//Contribution (1,0,0)
			let dx1 = dx0 - 1.0 - SQUISH_CONSTANT_3D;
			let dy1 = dy0 - 0.0 - SQUISH_CONSTANT_3D;
			let dz1 = dz0 - 0.0 - SQUISH_CONSTANT_3D;
			let attn1 = 2.0 - dx1 * dx1 - dy1 * dy1 - dz1 * dz1;
			if attn1 > 0.0 {
				attn1 *= attn1;
				value += attn1 * attn1 * extrapolate(xsb + 1, ysb + 0, zsb + 0, dx1, dy1, dz1);
			}

			//Contribution (0,1,0)
			let dx2 = dx0 - 0.0 - SQUISH_CONSTANT_3D;
			let dy2 = dy0 - 1.0 - SQUISH_CONSTANT_3D;
			let dz2 = dz1;
			let attn2 = 2.0 - dx2 * dx2 - dy2 * dy2 - dz2 * dz2;
			if attn2 > 0.0 {
				attn2 *= attn2;
				value += attn2 * attn2 * extrapolate(xsb + 0, ysb + 1, zsb + 0, dx2, dy2, dz2);
			}

			//Contribution (0,0,1)
			let dx3 = dx2;
			let dy3 = dy1;
			let dz3 = dz0 - 1.0 - SQUISH_CONSTANT_3D;
			let attn3 = 2.0 - dx3 * dx3 - dy3 * dy3 - dz3 * dz3;
			if attn3 > 0.0 {
				attn3 *= attn3;
				value += attn3 * attn3 * extrapolate(xsb + 0, ysb + 0, zsb + 1, dx3, dy3, dz3);
			}
		} else if inSum >= 2 { //We're inside the tetrahedron (3-Simplex) at (1,1,1)

			//Determine which two tetrahedral vertices are the closest, out of (1,1,0), (1,0,1), (0,1,1) but not (1,1,1).
			let aPoint = 0x06;
			let aScore = xins;
			let bPoint = 0x05;
			let bScore = yins;
			if aScore <= bScore && zins < bScore {
				bScore = zins;
				bPoint = 0x03;
			} else if aScore > bScore && zins < aScore {
				aScore = zins;
				aPoint = 0x03;
			}

			//Now we determine the two lattice points not part of the tetrahedron that may contribute.
			//This depends on the closest two tetrahedral vertices, including (1,1,1)
			let wins = 3.0 - inSum;
			if wins < aScore || wins < bScore { //(1,1,1) is one of the closest two tetrahedral vertices.
				let c = if bScore < aScore { bPoint } else { aPoint }; //Our other closest vertex is the closest out of a and b.

				if (c & 0x01) != 0 {
					xsv_ext0 = xsb + 2;
					xsv_ext1 = xsb + 1;
					dx_ext0 = dx0 - 2 - 3 * SQUISH_CONSTANT_3D;
					dx_ext1 = dx0 - 1 - 3 * SQUISH_CONSTANT_3D;
				} else {
					xsv_ext0 = xsv_ext1 = xsb;
					dx_ext0 = dx_ext1 = dx0 - 3 * SQUISH_CONSTANT_3D;
				}

				if (c & 0x02) != 0 {
					ysv_ext0 = ysv_ext1 = ysb + 1;
					dy_ext0 = dy_ext1 = dy0 - 1 - 3 * SQUISH_CONSTANT_3D;
					if (c & 0x01) != 0 {
						ysv_ext1 += 1;
						dy_ext1 -= 1;
					} else {
						ysv_ext0 += 1;
						dy_ext0 -= 1;
					}
				} else {
					ysv_ext0 = ysv_ext1 = ysb;
					dy_ext0 = dy_ext1 = dy0 - 3 * SQUISH_CONSTANT_3D;
				}

				if (c & 0x04) != 0 {
					zsv_ext0 = zsb + 1;
					zsv_ext1 = zsb + 2;
					dz_ext0 = dz0 - 1 - 3 * SQUISH_CONSTANT_3D;
					dz_ext1 = dz0 - 2 - 3 * SQUISH_CONSTANT_3D;
				} else {
					zsv_ext0 = zsv_ext1 = zsb;
					dz_ext0 = dz_ext1 = dz0 - 3 * SQUISH_CONSTANT_3D;
				}
			} else { //(1,1,1) is not one of the closest two tetrahedral vertices.
				let c = aPoint & bPoint; //Our two extra vertices are determined by the closest two.

				if (c & 0x01) != 0 {
					xsv_ext0 = xsb + 1;
					xsv_ext1 = xsb + 2;
					dx_ext0 = dx0 - 1 - SQUISH_CONSTANT_3D;
					dx_ext1 = dx0 - 2 - 2 * SQUISH_CONSTANT_3D;
				} else {
					xsv_ext0 = xsv_ext1 = xsb;
					dx_ext0 = dx0 - SQUISH_CONSTANT_3D;
					dx_ext1 = dx0 - 2 * SQUISH_CONSTANT_3D;
				}

				if (c & 0x02) != 0 {
					ysv_ext0 = ysb + 1;
					ysv_ext1 = ysb + 2;
					dy_ext0 = dy0 - 1 - SQUISH_CONSTANT_3D;
					dy_ext1 = dy0 - 2 - 2 * SQUISH_CONSTANT_3D;
				} else {
					ysv_ext0 = ysv_ext1 = ysb;
					dy_ext0 = dy0 - SQUISH_CONSTANT_3D;
					dy_ext1 = dy0 - 2 * SQUISH_CONSTANT_3D;
				}

				if (c & 0x04) != 0 {
					zsv_ext0 = zsb + 1;
					zsv_ext1 = zsb + 2;
					dz_ext0 = dz0 - 1 - SQUISH_CONSTANT_3D;
					dz_ext1 = dz0 - 2 - 2 * SQUISH_CONSTANT_3D;
				} else {
					zsv_ext0 = zsv_ext1 = zsb;
					dz_ext0 = dz0 - SQUISH_CONSTANT_3D;
					dz_ext1 = dz0 - 2 * SQUISH_CONSTANT_3D;
				}
			}

			//Contribution (1,1,0)
			let dx3 = dx0 - 1 - 2 * SQUISH_CONSTANT_3D;
			let dy3 = dy0 - 1 - 2 * SQUISH_CONSTANT_3D;
			let dz3 = dz0 - 0 - 2 * SQUISH_CONSTANT_3D;
			let attn3 = 2 - dx3 * dx3 - dy3 * dy3 - dz3 * dz3;
			if attn3 > 0 {
				attn3 *= attn3;
				value += attn3 * attn3 * extrapolate(xsb + 1, ysb + 1, zsb + 0, dx3, dy3, dz3);
			}

			//Contribution (1,0,1)
			let dx2 = dx3;
			let dy2 = dy0 - 0 - 2 * SQUISH_CONSTANT_3D;
			let dz2 = dz0 - 1 - 2 * SQUISH_CONSTANT_3D;
			let attn2 = 2 - dx2 * dx2 - dy2 * dy2 - dz2 * dz2;
			if attn2 > 0 {
				attn2 *= attn2;
				value += attn2 * attn2 * extrapolate(xsb + 1, ysb + 0, zsb + 1, dx2, dy2, dz2);
			}

			//Contribution (0,1,1)
			let dx1 = dx0 - 0 - 2 * SQUISH_CONSTANT_3D;
			let dy1 = dy3;
			let dz1 = dz2;
			let attn1 = 2 - dx1 * dx1 - dy1 * dy1 - dz1 * dz1;
			if attn1 > 0 {
				attn1 *= attn1;
				value += attn1 * attn1 * extrapolate(xsb + 0, ysb + 1, zsb + 1, dx1, dy1, dz1);
			}

			//Contribution (1,1,1)
			dx0 = dx0 - 1 - 3 * SQUISH_CONSTANT_3D;
			dy0 = dy0 - 1 - 3 * SQUISH_CONSTANT_3D;
			dz0 = dz0 - 1 - 3 * SQUISH_CONSTANT_3D;
			let attn0 = 2 - dx0 * dx0 - dy0 * dy0 - dz0 * dz0;
			if attn0 > 0 {
				attn0 *= attn0;
				value += attn0 * attn0 * extrapolate(xsb + 1, ysb + 1, zsb + 1, dx0, dy0, dz0);
			}
		} else { //We're inside the octahedron (Rectified 3-Simplex) in between.
			let mut aScore;
			let mut aPoint;
			let mut aIsFurtherSide;
			let mut bScore;
			let mut bPoint;
			let mut bIsFurtherSide;

			//Decide between point (0,0,1) and (1,1,0) as closest
			let p1 = xins + yins;
			if p1 > 1 {
				aScore = p1 - 1;
				aPoint = 0x03;
				aIsFurtherSide = true;
			} else {
				aScore = 1 - p1;
				aPoint = 0x04;
				aIsFurtherSide = false;
			}

			//Decide between point (0,1,0) and (1,0,1) as closest
			let p2 = xins + zins;
			if p2 > 1 {
				bScore = p2 - 1;
				bPoint = 0x05;
				bIsFurtherSide = true;
			} else {
				bScore = 1 - p2;
				bPoint = 0x02;
				bIsFurtherSide = false;
			}

			//The closest out of the two (1,0,0) and (0,1,1) will replace the furthest out of the two decided above, if closer.
			let p3 = yins + zins;
			if p3 > 1 {
				let score = p3 - 1;
				if aScore <= bScore && aScore < score {
					aScore = score;
					aPoint = 0x06;
					aIsFurtherSide = true;
				} else if aScore > bScore && bScore < score {
					bScore = score;
					bPoint = 0x06;
					bIsFurtherSide = true;
				}
			} else {
				let score = 1 - p3;
				if aScore <= bScore && aScore < score {
					aScore = score;
					aPoint = 0x01;
					aIsFurtherSide = false;
				} else if aScore > bScore && bScore < score {
					bScore = score;
					bPoint = 0x01;
					bIsFurtherSide = false;
				}
			}

			//Where each of the two closest points are determines how the extra two vertices are calculated.
			if aIsFurtherSide == bIsFurtherSide {
				if aIsFurtherSide { //Both closest points on (1,1,1) side

					//One of the two extra points is (1,1,1)
					dx_ext0 = dx0 - 1 - 3 * SQUISH_CONSTANT_3D;
					dy_ext0 = dy0 - 1 - 3 * SQUISH_CONSTANT_3D;
					dz_ext0 = dz0 - 1 - 3 * SQUISH_CONSTANT_3D;
					xsv_ext0 = xsb + 1;
					ysv_ext0 = ysb + 1;
					zsv_ext0 = zsb + 1;

					//Other extra point is based on the shared axis.
					let c = aPoint & bPoint;
					if (c & 0x01) != 0 {
						dx_ext1 = dx0 - 2 - 2 * SQUISH_CONSTANT_3D;
						dy_ext1 = dy0 - 2 * SQUISH_CONSTANT_3D;
						dz_ext1 = dz0 - 2 * SQUISH_CONSTANT_3D;
						xsv_ext1 = xsb + 2;
						ysv_ext1 = ysb;
						zsv_ext1 = zsb;
					} else if (c & 0x02) != 0 {
						dx_ext1 = dx0 - 2 * SQUISH_CONSTANT_3D;
						dy_ext1 = dy0 - 2 - 2 * SQUISH_CONSTANT_3D;
						dz_ext1 = dz0 - 2 * SQUISH_CONSTANT_3D;
						xsv_ext1 = xsb;
						ysv_ext1 = ysb + 2;
						zsv_ext1 = zsb;
					} else {
						dx_ext1 = dx0 - 2 * SQUISH_CONSTANT_3D;
						dy_ext1 = dy0 - 2 * SQUISH_CONSTANT_3D;
						dz_ext1 = dz0 - 2 - 2 * SQUISH_CONSTANT_3D;
						xsv_ext1 = xsb;
						ysv_ext1 = ysb;
						zsv_ext1 = zsb + 2;
					}
				} else {//Both closest points on (0,0,0) side

					//One of the two extra points is (0,0,0)
					dx_ext0 = dx0;
					dy_ext0 = dy0;
					dz_ext0 = dz0;
					xsv_ext0 = xsb;
					ysv_ext0 = ysb;
					zsv_ext0 = zsb;

					//Other extra point is based on the omitted axis.
					let c = aPoint | bPoint;
					if (c & 0x01) == 0 {
						dx_ext1 = dx0 + 1 - SQUISH_CONSTANT_3D;
						dy_ext1 = dy0 - 1 - SQUISH_CONSTANT_3D;
						dz_ext1 = dz0 - 1 - SQUISH_CONSTANT_3D;
						xsv_ext1 = xsb - 1;
						ysv_ext1 = ysb + 1;
						zsv_ext1 = zsb + 1;
					} else if (c & 0x02) == 0 {
						dx_ext1 = dx0 - 1 - SQUISH_CONSTANT_3D;
						dy_ext1 = dy0 + 1 - SQUISH_CONSTANT_3D;
						dz_ext1 = dz0 - 1 - SQUISH_CONSTANT_3D;
						xsv_ext1 = xsb + 1;
						ysv_ext1 = ysb - 1;
						zsv_ext1 = zsb + 1;
					} else {
						dx_ext1 = dx0 - 1 - SQUISH_CONSTANT_3D;
						dy_ext1 = dy0 - 1 - SQUISH_CONSTANT_3D;
						dz_ext1 = dz0 + 1 - SQUISH_CONSTANT_3D;
						xsv_ext1 = xsb + 1;
						ysv_ext1 = ysb + 1;
						zsv_ext1 = zsb - 1;
					}
				}
			} else { //One point on (0,0,0) side, one point on (1,1,1) side
				let c1;
                let c2;
				if aIsFurtherSide {
					c1 = aPoint;
					c2 = bPoint;
				} else {
					c1 = bPoint;
					c2 = aPoint;
				}

				//One contribution is a permutation of (1,1,-1)
				if (c1 & 0x01) == 0 {
					dx_ext0 = dx0 + 1 - SQUISH_CONSTANT_3D;
					dy_ext0 = dy0 - 1 - SQUISH_CONSTANT_3D;
					dz_ext0 = dz0 - 1 - SQUISH_CONSTANT_3D;
					xsv_ext0 = xsb - 1;
					ysv_ext0 = ysb + 1;
					zsv_ext0 = zsb + 1;
				} else if (c1 & 0x02) == 0 {
					dx_ext0 = dx0 - 1 - SQUISH_CONSTANT_3D;
					dy_ext0 = dy0 + 1 - SQUISH_CONSTANT_3D;
					dz_ext0 = dz0 - 1 - SQUISH_CONSTANT_3D;
					xsv_ext0 = xsb + 1;
					ysv_ext0 = ysb - 1;
					zsv_ext0 = zsb + 1;
				} else {
					dx_ext0 = dx0 - 1 - SQUISH_CONSTANT_3D;
					dy_ext0 = dy0 - 1 - SQUISH_CONSTANT_3D;
					dz_ext0 = dz0 + 1 - SQUISH_CONSTANT_3D;
					xsv_ext0 = xsb + 1;
					ysv_ext0 = ysb + 1;
					zsv_ext0 = zsb - 1;
				}

				//One contribution is a permutation of (0,0,2)
				dx_ext1 = dx0 - 2 * SQUISH_CONSTANT_3D;
				dy_ext1 = dy0 - 2 * SQUISH_CONSTANT_3D;
				dz_ext1 = dz0 - 2 * SQUISH_CONSTANT_3D;
				xsv_ext1 = xsb;
				ysv_ext1 = ysb;
				zsv_ext1 = zsb;
				if (c2 & 0x01) != 0 {
					dx_ext1 -= 2;
					xsv_ext1 += 2;
				} else if (c2 & 0x02) != 0 {
					dy_ext1 -= 2;
					ysv_ext1 += 2;
				} else {
					dz_ext1 -= 2;
					zsv_ext1 += 2;
				}
			}

			//Contribution (1,0,0)
			let dx1 = dx0 - 1 - SQUISH_CONSTANT_3D;
			let dy1 = dy0 - 0 - SQUISH_CONSTANT_3D;
			let dz1 = dz0 - 0 - SQUISH_CONSTANT_3D;
			let attn1 = 2 - dx1 * dx1 - dy1 * dy1 - dz1 * dz1;
			if attn1 > 0 {
				attn1 *= attn1;
				value += attn1 * attn1 * extrapolate(xsb + 1, ysb + 0, zsb + 0, dx1, dy1, dz1);
			}

			//Contribution (0,1,0)
			let dx2 = dx0 - 0 - SQUISH_CONSTANT_3D;
			let dy2 = dy0 - 1 - SQUISH_CONSTANT_3D;
			let dz2 = dz1;
			let attn2 = 2 - dx2 * dx2 - dy2 * dy2 - dz2 * dz2;
			if attn2 > 0 {
				attn2 *= attn2;
				value += attn2 * attn2 * extrapolate(xsb + 0, ysb + 1, zsb + 0, dx2, dy2, dz2);
			}

			//Contribution (0,0,1)
			let dx3 = dx2;
			let dy3 = dy1;
			let dz3 = dz0 - 1 - SQUISH_CONSTANT_3D;
			let attn3 = 2 - dx3 * dx3 - dy3 * dy3 - dz3 * dz3;
			if attn3 > 0 {
				attn3 *= attn3;
				value += attn3 * attn3 * extrapolate(xsb + 0, ysb + 0, zsb + 1, dx3, dy3, dz3);
			}

			//Contribution (1,1,0)
			let dx4 = dx0 - 1 - 2 * SQUISH_CONSTANT_3D;
			let dy4 = dy0 - 1 - 2 * SQUISH_CONSTANT_3D;
			let dz4 = dz0 - 0 - 2 * SQUISH_CONSTANT_3D;
			let attn4 = 2 - dx4 * dx4 - dy4 * dy4 - dz4 * dz4;
			if attn4 > 0 {
				attn4 *= attn4;
				value += attn4 * attn4 * extrapolate(xsb + 1, ysb + 1, zsb + 0, dx4, dy4, dz4);
			}

			//Contribution (1,0,1)
			let dx5 = dx4;
			let dy5 = dy0 - 0 - 2 * SQUISH_CONSTANT_3D;
			let dz5 = dz0 - 1 - 2 * SQUISH_CONSTANT_3D;
			let attn5 = 2 - dx5 * dx5 - dy5 * dy5 - dz5 * dz5;
			if attn5 > 0 {
				attn5 *= attn5;
				value += attn5 * attn5 * extrapolate(xsb + 1, ysb + 0, zsb + 1, dx5, dy5, dz5);
			}

			//Contribution (0,1,1)
			let dx6 = dx0 - 0 - 2 * SQUISH_CONSTANT_3D;
			let dy6 = dy4;
			let dz6 = dz5;
			let attn6 = 2 - dx6 * dx6 - dy6 * dy6 - dz6 * dz6;
			if attn6 > 0 {
				attn6 *= attn6;
				value += attn6 * attn6 * extrapolate(xsb + 0, ysb + 1, zsb + 1, dx6, dy6, dz6);
			}
		}

		//First extra vertex
		let attn_ext0 = 2 - dx_ext0 * dx_ext0 - dy_ext0 * dy_ext0 - dz_ext0 * dz_ext0;
		if attn_ext0 > 0
		{
			attn_ext0 *= attn_ext0;
			value += attn_ext0 * attn_ext0 * extrapolate(xsv_ext0, ysv_ext0, zsv_ext0, dx_ext0, dy_ext0, dz_ext0);
		}

		//Second extra vertex
		let attn_ext1 = 2 - dx_ext1 * dx_ext1 - dy_ext1 * dy_ext1 - dz_ext1 * dz_ext1;
		if attn_ext1 > 0
		{
			attn_ext1 *= attn_ext1;
			value += attn_ext1 * attn_ext1 * extrapolate(xsv_ext1, ysv_ext1, zsv_ext1, dx_ext1, dy_ext1, dz_ext1);
		}

		return value / NORM_CONSTANT_3D;

    }
}
