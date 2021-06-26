//! Struct representing probability density function

pub use crate::hit::*;
pub use crate::misc::Onb;
pub use crate::primitive::*;
pub use crate::V3;
use objekt_clonable::*;

/// Represents probability density function
//#[clonable]
#[enum_dispatch(Primitive)]
pub trait Pdf {
    /// Gives value of this Pdf for given direction, or "probability" of given direction to sample from this Pdf.
    fn value(&self, origin: V3<f32>, direction: V3<f32>) -> f32;

    /// Generate, or sample, new direction from this Pdf.
    fn generate(&self, origin: V3<f32>) -> V3<f32>;
}
/// Dirac delta function density
pub struct ZeroPdf;
impl Pdf for ZeroPdf {
    fn value(&self, _origin: V3<f32>, _direction: V3<f32>) -> f32 {
        0.0
    }
    // Should not be used
    fn generate(&self, _origin: V3<f32>) -> V3<f32> {
        V3::zero()
    }
}

/// Uniform Pdf on a sphere.
pub struct UniformPdf;
impl Pdf for UniformPdf {
    fn value(&self, _origin: V3<f32>, _direction: V3<f32>) -> f32 {
        1.0 / 4.0 * std::f32::consts::PI
    }

    fn generate(&self, _origin: V3<f32>) -> V3<f32> {
        V3::get_point_on_sphere()
    }
}

/// Cosine density
pub struct CosinePdf {
    /// Vector representing direction of distribution, points sample around that vector, usually it should be normal to the surface.
    uvw: Onb,
}

impl CosinePdf {
    /// Returns new cosine PDF.
    pub fn new(w: &V3<f32>) -> Self {
        CosinePdf {
            uvw: Onb::build_from_w(w),
        }
    }
}

impl Pdf for CosinePdf {
    fn value(&self, _origin: V3<f32>, direction: V3<f32>) -> f32 {
        let cosine = direction.norm().dot(self.uvw.w());
        if cosine <= 0.0 {
            0.0
        } else {
            cosine / std::f32::consts::PI
        }
    }
    fn generate(&self, _origin: V3<f32>) -> V3<f32> {
        self.uvw.local_from_vec(&V3::random_cosine_direction())
    }
}

/// MixturePdf represents weighted average of two probability density functions, which average is also PDF.
pub struct MixturePdf<'a, 'b, T, P>
where
    T: 'a + Pdf + ?Sized,
    P: 'b + Pdf + ?Sized,
{
    p0: &'a T,
    p1: &'b P,
}

impl<'a, 'b, T, P> MixturePdf<'a, 'b, T, P>
where
    T: Pdf + ?Sized,
    P: Pdf + ?Sized,
{
    /// Returns new MixturePdf given two PDFs.
    pub fn new(p0: &'a T, p1: &'b P) -> Self {
        MixturePdf { p0, p1 }
    }
}

impl<'a, 'b, T, P> Pdf for MixturePdf<'a, 'b, T, P>
where
    T: Pdf + ?Sized,
    P: Pdf + ?Sized,
{
    fn value(&self, origin: V3<f32>, direction: V3<f32>) -> f32 {
        0.5 * self.p0.value(origin, direction) + 0.5 * self.p1.value(origin, direction)
    }

    fn generate(&self, origin: V3<f32>) -> V3<f32> {
        use rand::Rng;
        if rand::thread_rng().gen::<f32>() < 0.5f32 {
            self.p0.generate(origin)
        } else {
            self.p1.generate(origin)
        }
    }
}

use enum_dispatch::enum_dispatch;
#[clonable]
/// HittablePdf represents structs that have PDF property and can be hit by ray.
#[enum_dispatch(Primitive)]
pub trait HittablePdf: Hittable + Pdf + Send + Sync + Clone {}
