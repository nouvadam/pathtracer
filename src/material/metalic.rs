use crate::hit::Hit;
use crate::material::*;
use crate::V3;
use crate::ray::Ray;
/// Metalic material.
#[derive(Clone)]
pub struct Metalic {
    /// Color of metalic surface.
    pub albedo: V3<f32>,
    /// Irregularity of surface.
    pub fuzz: f32,
}

impl Material for Metalic {
    fn scatter<'a>(&self, ray: &'a Ray, hit: &Hit) -> Option<(Ray<'a>, V3<f32>, bool)> {
        let norm = ray.end.norm();
        let reflected = reflect(norm, hit.normal);

        let end = reflected + V3::get_point_on_sphere() * self.fuzz;

        let scattered = Ray {
            origin: hit.point,
            end,
            ..*ray
        };

        if scattered.end.dot(hit.normal) > 0.0 {
            Some((scattered, self.albedo, false))
        } else {
            None
        }
    }
}
