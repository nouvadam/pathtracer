use crate::hit::Hit;
use crate::material::*;
use crate::ray::Ray;
use crate::texture::Texture;
use crate::V3;
/// Emits light.
#[derive(Clone)]
pub struct LightSource {
    /// Texture of the object.
    pub albedo: Box<dyn Texture + Sync + Send>,
}

impl Material for LightSource {
    #[warn(unused_variables)]
    fn scatter<'a>(&self, ray: &'a Ray, hit: &Hit) -> Option<(Ray<'a>, V3<f32>, bool)> {
        let target = hit.point + hit.normal; // + V3::get_point_on_sphere();
        Some((
            Ray {
                origin: hit.point,
                end: target - hit.point,
                ..*ray
            },
            self.albedo.value(hit.u, hit.v, hit.point),
            true,
        ))
    }
}
