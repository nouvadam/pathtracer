use crate::hittables::Aabb;
use crate::misc::{HittablePdf, Pdf};
use crate::{Hit, Hittable, Ray, V3};
/// Represents translated object.
#[derive(Clone)]
pub struct Translated {
    /// Object position is translated by this vector.
    offset: V3<f32>,
    /// Object that is translated.
    hittable: Box<dyn HittablePdf>,
}

impl Hittable for Translated {
    fn hit(&self, ray: &Ray) -> Option<Hit> {
        let offset_ray = Ray {
            origin: ray.origin - self.offset,
            ..*ray
        };

        let hit = self.hittable.hit(&offset_ray);

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
            x: self.hittable.bounding_box().x + self.offset.x,
            y: self.hittable.bounding_box().y + self.offset.y,
            z: self.hittable.bounding_box().z + self.offset.z,
        }
    }
}
/// Transforms object into Translated object
pub trait IntoTranslated {
    /// Transforms object into Translated object
    fn translate(self, offset: V3<f32>) -> Translated;
}

// Trait IntoTranslated is implemented for all hittable objects
impl<T> IntoTranslated for T
where
    T: HittablePdf + 'static,
{
    fn translate(self, offset: V3<f32>) -> Translated {
        Translated {
            offset,
            hittable: Box::new(self),
        }
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
