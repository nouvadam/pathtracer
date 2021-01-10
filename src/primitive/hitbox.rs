use crate::hit::*;
use crate::hitables::HitableList;
use crate::hitables::AABB;
use crate::material::Material;
use crate::V3;
use crate::primitive::rectangle::*;
use crate::ray::*;
/// Cuboid made from rectangles.
#[derive(Clone)]
pub struct HitBox {
    box_min: V3<f32>,
    box_max: V3<f32>,
    sides: HitableList,
    material: Box<dyn Material + Sync + Send>,
}

impl HitBox {
    /// Returns new cuboid.
    ///
    /// `box_min` - First verticle of the cuboid.
    ///
    /// `box_max` - Opposite verticle of the cuboid.
    pub fn new(
        box_min: V3<f32>,
        box_max: V3<f32>,
        material: Box<dyn Material + Sync + Send>,
    ) -> HitBox {
        let mut sides = HitableList::new();

        sides.add(Box::new(XYrect {
            x0: box_min.x,
            x1: box_max.x,
            y0: box_min.y,
            y1: box_max.y,
            k: box_max.z,
            material: material.clone(),
        }));

        sides.add(Box::new(XYrect {
            x0: box_min.x,
            x1: box_max.x,
            y0: box_min.y,
            y1: box_max.y,
            k: box_min.z,
            material: material.clone(),
        }));

        sides.add(Box::new(XZrect {
            x0: box_min.x,
            x1: box_max.x,
            z0: box_min.z,
            z1: box_max.z,
            k: box_max.y,
            material: material.clone(),
        }));

        sides.add(Box::new(XZrect {
            x0: box_min.x,
            x1: box_max.x,
            z0: box_min.z,
            z1: box_max.z,
            k: box_min.y,
            material: material.clone(),
        }));

        sides.add(Box::new(YZrect {
            y0: box_min.y,
            y1: box_max.y,
            z0: box_min.z,
            z1: box_max.z,
            k: box_max.x,
            material: material.clone(),
        }));

        sides.add(Box::new(YZrect {
            y0: box_min.y,
            y1: box_max.y,
            z0: box_min.z,
            z1: box_max.z,
            k: box_min.x,
            material: material.clone(),
        }));

        HitBox {
            box_min,
            box_max,
            sides,
            material,
        }
    }
}

impl Hitable for HitBox {
    fn hit(&self, r: &Ray, t_min: f32, t_max: f32) -> Option<Hit> {
        self.sides.hit(r, t_min, t_max)
    }

    fn bounding_box(&self) -> AABB {
        AABB {
            min: self.box_min,
            max: self.box_max,
        }
    }
}
