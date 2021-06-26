use crate::hittables::Aabb;
use crate::misc::Pdf;
use crate::{Hit, Hittable, Primitive, Ray, V3};
/// Represents translated object.
#[derive(Clone)]
pub struct Translated {
    /// Object position is translated by this vector.
    offset: V3<f32>,
    /// Object that is translated.
    hittable: Box<Primitive>,
}

impl Hittable for Translated {
    fn hit(&self, r: &Ray, t_min: f32, t_max: f32) -> Option<Hit> {
        let offset_ray = Ray {
            origin: r.origin - self.offset,
            ..*r
        };

        let hit = self.hittable.hit(&offset_ray, t_min, t_max);

        hit.map(|hit| {
            Hit::new(
                &offset_ray,
                hit.normal,
                hit.t,
                hit.point + self.offset,
                hit.material,
                hit.u,
                hit.v,
            )
        })
    }

    fn bounding_box(&self) -> Aabb {
        Aabb {
            min: self.hittable.bounding_box().min + self.offset,
            max: self.hittable.bounding_box().max + self.offset,
        }
    }
}
/// Transforms object into Translated object
pub trait IntoTranslated {
    /// Transforms object into Translated object
    fn translate(self, offset: V3<f32>) -> Primitive;
}

// Trait IntoTranslated is implemented for all hittable objects
impl IntoTranslated for Primitive {
    fn translate(self, offset: V3<f32>) -> Primitive {
        Primitive::Translated(Translated {
            offset,
            hittable: Box::new(self),
        })
    }
}

impl Pdf for Translated {
    fn value(&self, origin: V3<f32>, direction: V3<f32>) -> f32 {
        self.hittable
            .value(origin - self.offset, direction - self.offset)
    }

    fn generate(&self, origin: V3<f32>) -> V3<f32> {
        self.hittable.generate(origin - self.offset)
    }
}
