use num_traits::Float;
use rand::Rng;
use std::ops::{Add, Div, Mul, Neg, Sub};

/// Simple 3d vector library.
#[derive(Copy, Debug, PartialEq, PartialOrd, Clone, Default)]
pub struct V3<T> {
    /// X component of vector.
    pub x: T,
    /// Y component of vector.
    pub y: T,
    /// Z component of vector.
    pub z: T,
}

use std::ops::Index;
impl<T> Index<u32> for V3<T> {
    type Output = T;

    fn index(&self, index: u32) -> &Self::Output {
        match index {
            0 => &self.x,
            1 => &self.y,
            2 => &self.z,
            _ => panic!("V3 is out of bound"),
        }
    }
}

impl<T: Copy> IntoIterator for V3<T> {
    type Item = T;
    type IntoIter = V3IntoIterator<T>;

    fn into_iter(self) -> Self::IntoIter {
        V3IntoIterator { v3: self, index: 0 }
    }
}
/// V3 made into Interator.
#[derive(Clone)]
pub struct V3IntoIterator<T> {
    v3: V3<T>,
    index: usize,
}

impl<T: Copy> Iterator for V3IntoIterator<T> {
    type Item = T;
    fn next(&mut self) -> Option<T> {
        let result = match self.index {
            0 => self.v3.x,
            1 => self.v3.y,
            2 => self.v3.z,
            _ => return None,
        };
        self.index += 1;
        Some(result)
    }
}

use std::iter::FromIterator;
impl<T: Default> FromIterator<T> for V3<T> {
    fn from_iter<I: IntoIterator<Item = T>>(iter: I) -> Self {
        let mut v3 = V3::default();

        for (index, i) in iter.into_iter().enumerate() {
            match index {
                0 => v3.x = i,
                1 => v3.y = i,
                2 => v3.z = i,
                _ => {}
            }
        }

        v3
    }
}

impl<T> V3<T> {
    /// Returns new Vector
    #[inline]
    pub fn new(x: T, y: T, z: T) -> V3<T> {
        V3 { x, y, z }
    }
}

impl<T: Add<U, Output = V>, U, V> Add<V3<U>> for V3<T> {
    type Output = V3<V>;
    fn add(self, rhs: V3<U>) -> V3<V> {
        V3::new(self.x + rhs.x, self.y + rhs.y, self.z + rhs.z)
    }
}

impl<T: Sub<U, Output = V>, U, V> Sub<V3<U>> for V3<T> {
    type Output = V3<V>;
    fn sub(self, rhs: V3<U>) -> V3<V> {
        V3::new(self.x - rhs.x, self.y - rhs.y, self.z - rhs.z)
    }
}

impl<T: Neg<Output = T>> Neg for V3<T> {
    type Output = V3<T>;
    fn neg(self) -> V3<T> {
        V3::new(-self.x, -self.y, -self.z)
    }
}

impl<T: Mul<U, Output = V>, U: Copy, V> Mul<U> for V3<T> {
    type Output = V3<V>;
    fn mul(self, rhs: U) -> V3<V> {
        V3::new(self.x * rhs, self.y * rhs, self.z * rhs)
    }
}

impl<T: Div<U, Output = V>, U: Copy, V> Div<U> for V3<T> {
    type Output = V3<V>;
    fn div(self, rhs: U) -> V3<V> {
        V3::new(self.x / rhs, self.y / rhs, self.z / rhs)
    }
}

impl<
        T: Add<T, Output = T>
            + Sub<T, Output = T>
            + Mul<T, Output = T>
            + Div<T, Output = T>
            + Neg<Output = T>
            + Copy
            + Clone,
    > V3<T>
{
    /// Multiply Vector by another Vector element-wise.
    pub fn mul(self, other: V3<T>) -> V3<T> {
        V3::new(self.x * other.x, self.y * other.y, self.z * other.z)
    }
    /// Divide Vector by another Vector element-wise.
    pub fn div(self, other: V3<T>) -> V3<T> {
        V3::new(self.x / other.x, self.y / other.y, self.z / other.z)
    }
    /// Dot product of two Vectors.
    pub fn dot(self, other: V3<T>) -> T {
        self.x * other.x + self.y * other.y + self.z * other.z
    }
    /// Cross product of two Vectors.
    pub fn cross(self, other: V3<T>) -> V3<T> {
        V3::new(
            self.y * other.z - self.z * other.y,
            self.z * other.x - self.x * other.z,
            self.x * other.y - self.y * other.x,
        )
    }
}

impl<
        T: Add<T, Output = T>
            + Sub<T, Output = T>
            + Mul<T, Output = T>
            + Div<T, Output = T>
            + Neg
            + Float,
    > V3<T>
{
    /// Returns length of the vector.
    pub fn length(self) -> T {
        (self.x * self.x + self.y * self.y + self.z * self.z).sqrt()
    }
    /// Returns normalized vector, which length equals 1.
    pub fn norm(self) -> V3<T> {
        self / V3::length(self)
    }

    /// Returns vector rotated around Y axis by some angle.
    pub fn rot_y(self, sin: T, cos: T) -> V3<T> {
        V3::new(
            self.x * cos + self.z * sin,
            self.y,
            -sin * self.x + cos * self.z,
        )
    }
    /// Returns new vector, combined from two vectors, with smaller elemenets.
    pub fn min(self, other: V3<T>) -> V3<T> {
        V3::new(
            self.x.min(other.x),
            self.y.min(other.y),
            self.z.min(other.z),
        )
    }
    /// Returns new vector, combined from two vectors, with bigger elemenets.
    pub fn max(self, other: V3<T>) -> V3<T> {
        V3::new(
            self.x.max(other.x),
            self.y.max(other.y),
            self.z.max(other.z),
        )
    }

    /// Returns the smallest component from vector.
    pub fn min_component(self) -> T {
        self.x.min(self.y.min(self.z))
    }

    /// Returns the biggest component from vector.
    pub fn max_component(self) -> T {
        self.x.max(self.y.max(self.z))
    }
}

impl V3<f32> {
    /// Returns vector with components floored to the whole.
    pub fn floor(self) -> V3<f32> {
        V3::new(self.x.floor(), self.y.floor(), self.z.floor())
    }

    /// Returns random vector, whose lenght is smaller than 1.
    pub fn get_point_in_sphere() -> V3<f32> {
        let mut rng = rand::thread_rng();
        let mut random_point = V3::new(1.0, 1.0, 1.0);
        while random_point.length() > 1.0 {
            random_point = V3::new(
                (rng.gen::<f32>() * 2.0) - 1.0,
                (rng.gen::<f32>() * 2.0) - 1.0,
                (rng.gen::<f32>() * 2.0) - 1.0,
            );
        }
        random_point
    }

    /// Returns random vector. whose lenght equals 1.
    pub fn get_point_on_sphere() -> V3<f32> {
        V3::get_point_in_sphere().norm()
    }

    /// Returns random vector, whose elements are from range <-1, 1>.
    pub fn random() -> V3<f32> {
        let mut rng = rand::thread_rng();
        V3::new(
            (rng.gen::<f32>() * 2.0) - 1.0,
            (rng.gen::<f32>() * 2.0) - 1.0,
            (rng.gen::<f32>() * 2.0) - 1.0,
        )
        .norm()
    }

    /// Random cosine direction, with pdf proportional to cos(theta)/pi, used in lambertian scattering
    pub fn random_cosine_direction() -> V3<f32> {
        let mut rng = rand::thread_rng();

        let r1 = rng.gen::<f32>();
        let r2 = rng.gen::<f32>();

        let z = (1.0 - r2).sqrt();

        let phi = 2.0 * std::f32::consts::PI * r1;

        let sqr = r2.sqrt();

        let x = phi.cos() * sqr;
        let y = phi.sin() * sqr;

        V3::new(x, y, z)
    }

    /**
    Checks if vector's length is near zero.
    */
    pub fn near_zero(&self) -> bool {
        const EPSILON: f32 = 1e-8;
        (self.x.abs() < EPSILON) && (self.y.abs() < EPSILON) && (self.z.abs()) < EPSILON
    }

    /// function to rotate v3<T> point around axis, sin and cos arguments are precomputed, and they represent angle/2 because of algorithm
    pub fn rot(self, axis: V3<f32>, sin: f32, cos: f32) -> V3<f32> {
        //point to rotate
        let V3 {
            x: xp,
            y: yp,
            z: zp,
        } = self;

        // real component of quaternion
        let s = cos;
        // imaginary component of quaternion
        let V3 { x, y, z } = axis * (sin / axis.length());
        let s2 = s.powi(2);

        V3::new(
            (2.0 * (s2 + x.powi(2)) - 1.0) * xp
                + (2.0 * (x * y - s * z)) * yp
                + (2.0 * (s * y + x * z)) * zp,
            (2.0 * (s * z + x * y)) * xp
                + (2.0 * (s2 + y.powi(2)) - 1.0) * yp
                + (2.0 * (y * z - s * x)) * zp,
            (2.0 * (x * z - s * y)) * xp
                + (2.0 * (s * x + y * z)) * yp
                + (2.0 * (s2 + z.powi(2)) - 1.0) * zp,
        )
    }
}

use std::fmt;
impl fmt::Display for V3<f32> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "({}, {}, {})", self.x, self.y, self.z)
    }
}
