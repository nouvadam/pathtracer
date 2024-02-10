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

impl MaterialTrait for LightSource {
    #[warn(unused_variables)]
    fn scatter<'a>(&self, _ray_in: &'a Ray, _hit: &Hit) -> Option<ScatterRecord<'a>> {
        None
    }

    fn scattering_pdf(&self, _ray_in: &Ray, hit: &Hit, ray_scattered: &Ray) -> f32 {
        let cosine = ray_scattered.end.norm().dot(hit.normal);

        if cosine < 0.0 {
            0.0
        } else {
            cosine / std::f32::consts::PI
        }
    }

    fn color_emitted(&self, _ray_in: &Ray, hit: &Hit) -> V3<f32> {
        if hit.front_face {
            self.albedo.value(hit.u, hit.v, hit.point)
        } else {
            V3::zero()
        }
    }
}

impl LightSource {
    /// Returns new Dielectric material.
    pub fn new(albedo: Box<dyn Texture + Sync + Send>) -> Self {
        LightSource { albedo }
    }
}
