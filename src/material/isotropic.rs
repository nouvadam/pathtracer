use crate::hit::Hit;
use crate::material::*;
use crate::V3;
use crate::ray::Ray;
use crate::texture::Texture;

/// "Isotropic materials have identical material properties in all directions at every given point."

#[derive(Clone)]
pub struct Isotropic {
    /// Texture of the object.
    pub albedo: Box<dyn Texture + Sync + Send>,
}

impl Material for Isotropic {
    fn scatter<'a>(&self, ray: &'a Ray, hit: &Hit) -> Option<(Ray<'a>, V3<f32>, bool)> {
        let end = V3::get_point_on_sphere();
        Some((
            Ray {
                origin: hit.point,
                end,
                ..*ray
            },
            self.albedo.value(hit.u, hit.v, hit.point),
            false,
        ))
    }
}
