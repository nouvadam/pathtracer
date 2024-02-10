use crate::hit::Hit;
use crate::material::*;
use crate::misc::UniformPdf;
use crate::ray::Ray;
use crate::texture::Texture;
use crate::V3;

/// "Isotropic materials have identical material properties in all directions at every given point."

#[derive(Clone)]
pub struct Isotropic {
    /// Texture of the object.
    pub albedo: Box<dyn Texture + Sync + Send>,
}

impl MaterialTrait for Isotropic {
    fn scatter<'a>(&self, _ray_in: &'a Ray, hit: &Hit) -> Option<ScatterRecord<'a>> {
        Some(ScatterRecord {
            specular_ray: None,
            attenuation: self.albedo.value(hit.u, hit.v, hit.point),
            pdf: Box::new(UniformPdf),
        })
    }

    fn scattering_pdf(&self, _ray_in: &Ray, _hit: &Hit, _ray_scattered: &Ray) -> f32 {
        1.0 / (4.0 * std::f32::consts::PI)
    }

    fn color_emitted(&self, _ray_in: &Ray, _hit: &Hit) -> V3<f32> {
        V3::default()
    }
}

impl Isotropic {
    /// Returns new Dielectric material.
    pub fn new(albedo: Box<dyn Texture + Sync + Send>) -> Self {
        Isotropic { albedo }
    }
}
