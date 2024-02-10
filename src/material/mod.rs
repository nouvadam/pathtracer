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
use crate::misc::Pdf;
use crate::ray::Ray;
use crate::V3;

/// Each object that implements Material trait should be able to scatter incoming ray.

pub trait MaterialTrait: Send + Sync {
    /// Returns scattered ray from the surface of the object with given material.
    /// Takes:
    /// `&self` - Reference to material to handle different materials' scattering properties.
    /// `ray` - Ray that hit the object.
    /// `hit` - Informations about hit evenet of the ray.
    /// Gives:
    /// Ray - scattered ray vector from the surface of the object that scattered incoming ray, V3<f32> - color of scattered ray, f32 - value of Pdf for generated ray.
    fn scatter<'a>(&self, ray_in: &'a Ray, hit: &Hit) -> Option<ScatterRecord<'a>>;

    /// Returns value of probability density function of scattered ray given the material and incoming ray, to weight the scattered ray influence on the overall color of the pixel, because less probable scattered rays are less frequent.
    fn scattering_pdf(&self, ray_in: &Ray, hit: &Hit, ray_scattered: &Ray) -> f32;

    /// Returns albedo of emitted light on specific point on object.
    fn color_emitted(&self, ray_in: &Ray, hit: &Hit) -> V3<f32>;
}

/// ScatterRecord represents ray scatter instance.
pub struct ScatterRecord<'ray> {
    /// Scattered ray vector from the surface of the object that scattered incoming ray.
    pub specular_ray: Option<Ray<'ray>>,
    ///  Color of scattered ray.
    pub attenuation: V3<f32>,
    /// PDF corresponding to this record.
    pub pdf: Box<dyn Pdf>,
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
    let r_out_parallel = -n * ((1.0f32 - r_out_perp.length().powi(2)).abs().sqrt());
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
/// Contains materials
pub struct MaterialContainer {
    materials: Vec<Box<dyn MaterialTrait>>,
    none_material: Box<dyn MaterialTrait>,
}

impl MaterialContainer {
    /// Returns new material container.
    pub fn new() -> MaterialContainer {
        MaterialContainer {
            materials: Vec::new(),
            none_material: Box::new(Metalic::new(V3::new(0.8, 0.8, 0.9), 1.0)),
        }
    }

    /// Adds new material to container, returns it's index
    pub fn add<T: MaterialTrait + 'static>(&mut self, mat: T) -> usize {
        self.materials.push(Box::new(mat));
        self.materials.len() - 1
    }

    /// Returns material from container, if material doesnt exists under given index, returns default material (magneta)
    pub fn get(&self, index: usize) -> &dyn MaterialTrait {
        match self.materials.get(index) {
            Some(mat) => mat.as_ref(),
            None => self.none_material.as_ref(),
        }
    }
}
