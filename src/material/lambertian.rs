use crate::hit::Hit;
use crate::material::{Material, MaterialTrait};
use crate::misc::CosinePdf;
use crate::ray::Ray;
use crate::texture::Texture;
use crate::V3;

use super::ScatterRecord;

/// Diffusely reflecting surface.

pub struct Lambertian {
    /// Texture of the object.
    pub albedo: Box<dyn Texture + Sync + Send>,
}

impl MaterialTrait for Lambertian {
    #[warn(unused_variables)]
    fn scatter<'a>(&self, _ray_in: &'a Ray, hit: &Hit) -> Option<ScatterRecord<'a>> {
        Some(ScatterRecord {
            specular_ray: None,
            attenuation: self.albedo.value(hit.u, hit.v, hit.point),
            pdf: Box::new(CosinePdf::new(&hit.normal)),
        })
    }

    fn scattering_pdf<'a>(&self, _ray_in: &'a Ray, hit: &Hit, ray_scattered: &Ray) -> f32 {
        let cosine = ray_scattered.end.norm().dot(hit.normal);

        if cosine < 0.0 {
            0.0
        } else {
            cosine / std::f32::consts::PI
        }
    }

    fn color_emitted(&self, _ray: &Ray, _hit: &Hit) -> V3<f32> {
        V3::zero()
    }
}

impl Lambertian {
    /// Returns new Dielectric material.
    pub fn new(albedo: Box<dyn Texture + Sync + Send>) -> Material {
        Material::Lambertian(Lambertian { albedo })
    }
}
