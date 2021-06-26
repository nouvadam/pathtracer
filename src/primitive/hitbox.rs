use crate::hit::*;
use crate::hittables::Aabb;
use crate::hittables::HittableList;
use crate::misc::Pdf;
use crate::primitive::rectangle::*;
use crate::primitive::Primitive;
use crate::ray::*;
use crate::V3;
/// Cuboid made from rectangles.
#[derive(Clone)]
pub struct HitBox {
    box_min: V3<f32>,
    box_max: V3<f32>,
    sides: HittableList,
    material: usize,
}

impl HitBox {
    /// Returns new cuboid.
    ///
    /// `box_min` - First verticle of the cuboid.
    ///
    /// `box_max` - Opposite verticle of the cuboid.
    pub fn new(box_min: V3<f32>, box_max: V3<f32>, material: usize) -> Primitive {
        let mut sides = HittableList::new();

        sides.add(Primitive::XYrect(XYrect {
            x0: box_min.x,
            x1: box_max.x,
            y0: box_min.y,
            y1: box_max.y,
            k: box_max.z,
            material: material,
        }));

        sides.add(Primitive::XYrect(XYrect {
            x0: box_min.x,
            x1: box_max.x,
            y0: box_min.y,
            y1: box_max.y,
            k: box_min.z,
            material: material,
        }));

        sides.add(Primitive::XZrect(XZrect {
            x0: box_min.x,
            x1: box_max.x,
            z0: box_min.z,
            z1: box_max.z,
            k: box_max.y,
            material: material,
        }));

        sides.add(Primitive::XZrect(XZrect {
            x0: box_min.x,
            x1: box_max.x,
            z0: box_min.z,
            z1: box_max.z,
            k: box_min.y,
            material: material,
        }));

        sides.add(Primitive::YZrect(YZrect {
            y0: box_min.y,
            y1: box_max.y,
            z0: box_min.z,
            z1: box_max.z,
            k: box_max.x,
            material: material,
        }));

        sides.add(Primitive::YZrect(YZrect {
            y0: box_min.y,
            y1: box_max.y,
            z0: box_min.z,
            z1: box_max.z,
            k: box_min.x,
            material: material,
        }));

        Primitive::HitBox(Self {
            box_min,
            box_max,
            sides,
            material,
        })
    }
}

impl Hittable for HitBox {
    fn hit(&self, r: &Ray, t_min: f32, t_max: f32) -> Option<Hit> {
        self.sides.hit(r, t_min, t_max)
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
