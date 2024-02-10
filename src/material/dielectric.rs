use crate::hit::Hit;
use crate::material::*;
use crate::misc::ZeroPdf;
use crate::ray::Ray;
use crate::V3;
use rand::Rng;

/// Struct representing dielectrics, in form of glass.
#[derive(Clone)]
pub struct Dielectric {
    /// How much Rays are refracted, proportional to the speed of light in this object.
    pub refractive_index: f32,
}

impl MaterialTrait for Dielectric {
    fn scatter<'a>(&self, ray: &'a Ray, hit: &Hit) -> Option<ScatterRecord<'a>> {
        let etai_over_etat = if hit.front_face {
            1.0 / self.refractive_index
        } else {
            self.refractive_index
        };

        let unit_direction = ray.end.norm();

        let cos_theta = (-unit_direction).dot(hit.normal).min(1.0);

        let sin_theta = (1.0 - cos_theta.powi(2)).sqrt();

        let direction: V3<f32> = if (etai_over_etat * sin_theta > 1.0)
            || (rand::thread_rng().gen_range(0.0, 1.0) < schlick(cos_theta, etai_over_etat))
        {
            reflect(unit_direction, hit.normal)
        } else {
            refract(unit_direction, hit.normal, etai_over_etat)
        };

        Some(ScatterRecord {
            specular_ray: Some(Ray {
                origin: hit.point,
                end: direction,
                ..*ray
            }),
            attenuation: V3::new(1.0, 1.0, 1.0),
            pdf: Box::new(ZeroPdf),
        })
    }

    fn scattering_pdf(&self, _ray_in: &Ray, _hit: &Hit, _ray_scattered: &Ray) -> f32 {
        todo!()
    }

    fn color_emitted(&self, _ray_in: &Ray, _hit: &Hit) -> V3<f32> {
        todo!()
    }
}

impl Dielectric {
    /// Returns new Dielectric material.
    pub fn new(refractive_index: f32) -> Self {
        Dielectric { refractive_index }
    }
}
