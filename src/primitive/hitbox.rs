use crate::hit::*;
use crate::hittables::Aabb;
use crate::hittables::HittableList;
use crate::misc::Pdf;
use crate::primitive::rectangle::*;
use crate::ray::*;
use crate::V3;

/// Cuboid made from rectangles.
#[derive(Clone)]
pub struct HitBox {
    box_min: V3<f32>,
    box_max: V3<f32>,
    sides: HittableList,
}

impl HitBox {
    /// Returns new cuboid.
    ///
    /// `box_min` - First verticle of the cuboid.
    ///
    /// `box_max` - Opposite verticle of the cuboid.
    pub fn new(box_min: V3<f32>, box_max: V3<f32>, material: usize) -> Self {
        let mut sides = HittableList::new();

        sides.add(XYrect {
            x0: box_min.x,
            x1: box_max.x,
            y0: box_min.y,
            y1: box_max.y,
            k: box_max.z,
            material,
        });

        sides.add(XYrect {
            x0: box_min.x,
            x1: box_max.x,
            y0: box_min.y,
            y1: box_max.y,
            k: box_min.z,
            material,
        });

        sides.add(XZrect {
            x0: box_min.x,
            x1: box_max.x,
            z0: box_min.z,
            z1: box_max.z,
            k: box_max.y,
            material,
        });

        sides.add(XZrect {
            x0: box_min.x,
            x1: box_max.x,
            z0: box_min.z,
            z1: box_max.z,
            k: box_min.y,
            material,
        });

        sides.add(YZrect {
            y0: box_min.y,
            y1: box_max.y,
            z0: box_min.z,
            z1: box_max.z,
            k: box_max.x,
            material,
        });

        sides.add(YZrect {
            y0: box_min.y,
            y1: box_max.y,
            z0: box_min.z,
            z1: box_max.z,
            k: box_min.x,
            material,
        });

        Self {
            box_min,
            box_max,
            sides,
        }
    }
}

impl Hittable for HitBox {
    fn hit(&self, ray: &Ray) -> Option<Hit> {
        self.sides.hit(ray)
    }

    fn bounding_box(&self) -> Aabb {
        Aabb {
            min: self.box_min,
            max: self.box_max,
        }
    }
}

impl Pdf for HitBox {
    fn value(&self, _origin: V3<f32>, _direction: V3<f32>) -> f32 {
        todo!()
    }

    fn generate(&self, _origin: V3<f32>) -> V3<f32> {
        todo!()
    }
}
