//! Ways of scattering/refracting incoming rays.

mod dielectric;
mod isotropic;
mod lambertian;
mod light_source;
mod metalic;

pub use dielectric::Dielectric;
pub use isotropic::Isotropic;
pub use lambertian::Lambertian;
pub use light_source::LightSource;
pub use metalic::Metalic;

use crate::hit::Hit;
use crate::ray::Ray;
use crate::V3;

use objekt_clonable::*; //allows to clone refs to trait objects
/// Each object that implements Material trait should be able to scatter incoming ray.
#[clonable]
pub trait Material: Send + Sync + Clone + MaterialClone {
    /// Ray - scattered ray vector from the surface of the object that scattered incoming ray, V3<f32> - color of scattered ray, Bool - is this object a light source.
    fn scatter<'a>(&self, ray: &'a Ray, hit: &Hit) -> Option<(Ray<'a>, V3<f32>, bool)>;
}

/// Needed to clone trait object, which are implementations of Material trait, into threads for parralelization.
pub trait MaterialClone {
    /// Clones Material trait object.
    fn box_clone(&self) -> Box<dyn Material + Send + Sync>;
}

impl<T> MaterialClone for T
where
    T: 'static + Material + Clone,
{
    fn box_clone(&self) -> Box<dyn Material + Send + Sync> {
        Box::new((*self).clone())
    }
}

impl Clone for Box<dyn Material + Send + Sync> {
    fn clone(&self) -> Box<dyn Material + Send + Sync> {
        self.box_clone()
    }
}

// Various functions needed to compute reflections, refractions etc.

/// Returns reflected, mirrored direction.
///
/// `v` - Incoming Ray.
///
/// `n` - Vector normal to the surface.
pub fn reflect(v: V3<f32>, n: V3<f32>) -> V3<f32> {
    v - n * v.dot(n) * 2.0
}

/// Refraction using Snell's law. Returns direction of the refracted Ray.
///
/// `uv` - Incoming Ray.
///
/// `n` - Vector normal to the surface.
///
/// `etai_over_etat` - Ratio of refractive indices between air and object's material.
pub fn refract(uv: V3<f32>, n: V3<f32>, etai_over_etat: f32) -> V3<f32> {
    let cos_theta = (-uv).dot(n).min(1.0);
    let r_out_perp = (uv + n * cos_theta) * etai_over_etat;
    let r_out_parallel = -n * ((1.0f32 - r_out_perp.len().powi(2)).abs().sqrt());
    r_out_perp + r_out_parallel
}

/// Schlick's approximation for reflectance
///
/// `cosine` - Cosine of the angle of incoming Ray.
///
/// `refractive_index` - Refractive index.
pub fn schlick(cosine: f32, refractive_index: f32) -> f32 {
    let r0 = ((1.0 - refractive_index) / (1.0 + refractive_index)).powi(2);
    r0 + (1.0 - r0) * (1.0 - cosine).powi(5)
}
