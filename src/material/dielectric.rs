use crate::hit::Hit;
use crate::material::*;
use crate::V3;
use crate::ray::Ray;
use rand::Rng;
/// Struct representing dielectrics, in form of glass.
#[derive(Clone)]
pub struct Dielectric {
    /// How much Rays are refracted, proportional to the speed of light in this object.
    pub refractive_index: f32,
}

impl Material for Dielectric {
    fn scatter<'a>(&self, ray: &'a Ray, hit: &Hit) -> Option<(Ray<'a>, V3<f32>, bool)> {
        let etai_over_etat = if hit.front_face {
            1.0 / self.refractive_index
        } else {
            self.refractive_index
        };

        let unit_direction = ray.end.norm();

        let cos_theta = (-unit_direction).dot(hit.normal).min(1.0);

        let sin_theta = (1.0 - cos_theta.powi(2)).sqrt();

        let direction: V3<f32>;

        if (etai_over_etat * sin_theta > 1.0)
            || (rand::thread_rng().gen_range(0.0, 1.0) < schlick(cos_theta, etai_over_etat))
        {
            direction = reflect(unit_direction, hit.normal);
        } else {
            direction = refract(unit_direction, hit.normal, etai_over_etat);
        }

        Some((
            Ray {
                origin: hit.point,
                end: direction,
                ..*ray
            },
            V3::new(1.0, 1.0, 1.0),
            false,
        ))
    }
}
