use crate::hittables::Aabb;
use crate::misc::Pdf;
use crate::{Hit, Hittable, Primitive, Ray};
/// FlipFace represents primitive with flipped normals.
#[derive(Clone)]
pub struct FlipFace {
    /// Object that it's normal vectors are flipped.
    hittable: Box<Primitive>,
}

impl Hittable for FlipFace {
    fn hit(&self, r: &Ray, t_min: f32, t_max: f32) -> Option<Hit> {
        let hit = self.hittable.hit(r, t_min, t_max);

        hit.map(|hit| hit.flip_front_face())
    }

    fn bounding_box(&self) -> Aabb {
        Aabb {
            min: self.hittable.bounding_box().min,
            max: self.hittable.bounding_box().max,
        }
    }
}

impl Pdf for FlipFace {
    fn value(&self, origin: crate::V3<f32>, direction: crate::V3<f32>) -> f32 {
        self.hittable.value(origin, direction)
    }

    fn generate(&self, origin: crate::V3<f32>) -> crate::V3<f32> {
        self.hittable.generate(origin)
    }
}

/// Transforms object into FlipFace object
pub trait IntoFlipFace {
    /// Transforms object into Translated object
    fn flip_face(self) -> Primitive;
}

// Trait IntoTranslated is implemented for all hittable objects
impl<'mat> IntoFlipFace for Primitive {
    fn flip_face(self) -> Primitive {
        Primitive::FlipFace(FlipFace {
            hittable: Box::new(self),
        })
    }
}
