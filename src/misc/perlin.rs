use crate::V3;

use arr_macro::arr;
use permutation_iterator::Permutor;

//#[macro_use]
use itertools::*;
/// Perlin noise.
#[derive(Clone)]
pub struct Perlin {
    perlin: [V3<f32>; 256],
    permutate: ([u32; 256], [u32; 256], [u32; 256]),
}

impl Perlin {
    /// Returns new perlin n
    pub fn new() -> Perlin {
        let mut hashed_iter_x = Permutor::new(256 as u64);
        let mut hashed_iter_y = Permutor::new(256 as u64);
        let mut hashed_iter_z = Permutor::new(256 as u64);

        Perlin {
            perlin: arr![V3::random(); 256],
            permutate: (
                arr![hashed_iter_x.next().unwrap() as u32; 256],
                arr![hashed_iter_y.next().unwrap() as u32; 256],
                arr![hashed_iter_z.next().unwrap() as u32; 256],
            ),
        }
    }

    /// Returns grayscale value of noise at specific point.
    ///
    /// `point` - Point from which noise value is taken.
    pub fn noise(&self, point: V3<f32>) -> f32 {
        let point4 = point * 4.0;
        let x = (point4.x as i32) & 255;
        let y = (point4.y as i32) & 255;
        let z = (point4.z as i32) & 255;
        self.perlin[((self.permutate.0)[x as usize]
            ^ (self.permutate.1)[y as usize]
            ^ (self.permutate.2)[z as usize]) as usize]
            .dot(V3::new(1.0, 1.0, 1.0))
    }

    /// Returns grayscale value of noise at specific point with linear interpolation for smoothing.
    ///
    /// `point` - Point from which noise value is taken.
    pub fn noise_with_trilinear_interpolating(&self, point: V3<f32>) -> f32 {
        let x = point.x.floor() as i32;
        let y = point.y.floor() as i32;
        let z = point.z.floor() as i32;

        let _uvww = point - point.floor();

        let u = point.x - point.x.floor();
        let v = point.y - point.y.floor();
        let w = point.z - point.z.floor();

        let uu = u * u * (3.0 - 2.0 * u);
        let vv = v * v * (3.0 - 2.0 * v);
        let ww = w * w * (3.0 - 2.0 * w);

        iproduct!(0..2, 0..2, 0..2).fold(0.0, |acc, p| {
            acc + (((p.0 as f32) * uu + ((1 - p.0) as f32) * (1.0 - uu))
                * ((p.1 as f32) * vv + ((1 - p.1) as f32) * (1.0 - vv))
                * ((p.2 as f32) * ww + ((1 - p.2) as f32) * (1.0 - ww))
                * ((self.perlin[((self.permutate.0)[((x + p.0) & 255) as usize]
                    ^ (self.permutate.1)[((y + p.1) & 255) as usize]
                    ^ (self.permutate.2)[((z + p.2) & 255) as usize])
                    as usize])
                    .dot(V3::new(
                        u - (p.0 as f32),
                        v - (p.1 as f32),
                        w - (p.2 as f32),
                    ))))
        })
    }
    /// Composite noise that has multiple summed frequencies.
    ///
    /// `point` - Point on the object.
    ///
    /// `depth` - How many calls to noise method is used.
    pub fn turbulence(&self, point: V3<f32>, depth: i32) -> f32 {
        (0..depth)
            .map(|x| 2.0_f32.powi(x))
            .fold(0.0, |acc, x| {
                acc + (1.0 / x) * self.noise_with_trilinear_interpolating(point * x)
            })
            .abs()
    }
}
