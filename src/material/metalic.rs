use crate::hit::Hit;
use crate::material::*;
use crate::misc::ZeroPdf;
use crate::ray::Ray;
use crate::V3;

/// Metalic material.
#[derive(Clone)]
pub struct Metalic {
    /// Color of metalic surface.
    pub albedo: V3<f32>,
    /// Irregularity of surface.
    pub fuzz: f32,
}

impl MaterialTrait for Metalic {
    fn scatter<'a>(&self, ray_in: &'a Ray, hit: &Hit) -> Option<ScatterRecord<'a>> {
        let norm = ray_in.end.norm();
        let reflected = reflect(norm, hit.normal);
        let end = reflected + V3::get_point_on_sphere() * self.fuzz;

        let specular_ray = Ray {
            origin: hit.point,
            end,
            ..*ray_in
        };

        Some(ScatterRecord {
            specular_ray: Some(specular_ray),
            attenuation: self.albedo,
            pdf: Box::new(ZeroPdf),
        })
    }

    fn scattering_pdf<'a>(&self, _ray_in: &'a Ray, _hit: &Hit, _ray_scattered: &Ray) -> f32 {
        todo!()
    }

    fn color_emitted<'a>(&self, _ray_in: &'a Ray, _hit: &Hit) -> V3<f32> {
        todo!()
    }
}

impl Metalic {
    /// Returns new Dielectric material.
    pub fn new(albedo: V3<f32>, fuzz: f32) -> Material {
        Material::Metalic(Metalic { albedo, fuzz })
    }
}
