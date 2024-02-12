use crate::ray::Ray;
use crate::V3;
use itertools::*;

/// Bounding box, to use with Octtree
#[derive(Clone)]
pub struct Aabb {
    /// Represents cube's vertex that is lower-back-left  (on the negative side of coordinate system).
    pub min: V3<f32>,
    /// Represents cube's vertex that is upper-front-right  (on the positive side of coordinate system).
    pub max: V3<f32>,
}

// static DELTA: f32 = 0.0001f32;

impl Aabb {
    /// Checks if Ray intersects with this bounding box.
    pub fn hit(&self, ray: &Ray) -> bool {
        let inv_end = V3::new(1.0, 1.0, 1.0).div(ray.end);

        let tx0 = (self.min.x - ray.origin.x) * inv_end.x;
        let tx1 = (self.max.x - ray.origin.x) * inv_end.x;
        let ntx0 = tx0.min(tx1);
        let ntx1 = tx0.max(tx1);

        let ty0 = (self.min.y - ray.origin.y) * inv_end.y;
        let ty1 = (self.max.y - ray.origin.y) * inv_end.y;
        let nty0 = ty0.min(ty1);
        let nty1 = ty0.max(ty1);

        let lol0 = ntx0.max(nty0);
        let lol1 = ntx1.min(nty1);

        if lol0 > lol1 {
            return false;
        }

        let tz0 = (self.min.z - ray.origin.z) * inv_end.z;
        let tz1 = (self.max.z - ray.origin.z) * inv_end.z;

        let ntz0 = tz0.min(tz1);
        let ntz1 = tz0.max(tz1);

        let lolol0 = ntz0.max(lol0);
        let lolol1 = ntz1.min(lol1);

        lolol0 <= lolol1
    }

    /// Creates new bounding box that surrounds two bounding boxes.
    pub fn surrounding_box(&self, second: Aabb) -> Self {
        Aabb {
            min: V3::new(
                self.min.x.min(second.min.x),
                self.min.y.min(second.min.y),
                self.min.z.min(second.min.z),
            ),
            max: V3::new(
                self.max.x.max(second.max.x),
                self.max.y.max(second.max.y),
                self.max.z.max(second.max.z),
            ),
        }
    }

    /// Returns list of verticles of this bounding box.
    pub fn get_box_points(&self) -> Vec<V3<f32>> {
        iproduct!(0..=1, 0..=1, 0..=1)
            .map(|point| {
                V3::new(
                    ((point.0 as f32) * self.min.x) + ((1.0 - (point.0 as f32)) * self.max.x),
                    ((point.1 as f32) * self.min.y) + ((1.0 - (point.1 as f32)) * self.max.y),
                    ((point.2 as f32) * self.min.z) + ((1.0 - (point.2 as f32)) * self.max.z),
                )
            })
            .collect()
    }

    // /// Add padding to this bounding box to overcome numerical problems
    // pub fn pad(&self) -> Self {
    //     Aabb {

    //     }
    // }
}
