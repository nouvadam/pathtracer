use crate::hitables::AABB;
use crate::{Hit, Hitable, Ray, V3};
/// Represents translated object.
#[derive(Clone)]
pub struct Translated {
    /// Object position is translated by this vector.
    offset: V3<f32>,
    /// Object that is translated.
    hitable: Box<dyn Hitable>,
}

impl Hitable for Translated {
    fn hit(&self, r: &Ray, t_min: f32, t_max: f32) -> Option<Hit> {
        let offset_ray = Ray {
            origin: r.origin - self.offset,
            ..*r
        };

        let hit = self.hitable.hit(&offset_ray, t_min, t_max);

        match hit {
            Some(hit) => Some(Hit::new(
                &offset_ray,
                hit.normal,
                hit.t,
                hit.point + self.offset,
                hit.material,
                hit.u,
                hit.v,
            )),
            None => None,
        }
    }

    fn bounding_box(&self) -> AABB {
        AABB {
            min: self.hitable.bounding_box().min + self.offset,
            max: self.hitable.bounding_box().max + self.offset,
        }
    }
}
/// Transforms object into Translated object
pub trait IntoTranslated {
    /// Transforms object into Translated object
    fn translate(self, offset: V3<f32>) -> Box<Translated>;
}

// Trait IntoTranslated is implemented for all Hitable objects
impl<T: 'static + Hitable> IntoTranslated for Box<T> {
    fn translate(self, offset: V3<f32>) -> Box<Translated> {
        Box::new(Translated {
            offset,
            hitable: self,
        })
    }
}
