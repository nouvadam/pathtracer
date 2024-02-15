use crate::misc::Interval;
use crate::ray::Ray;
use crate::V3;
use itertools::*;

/// Bounding box, to use with Octtree
#[derive(Clone)]
pub struct Aabb {
    /// Represents cube's vertex that is lower-back-left  (on the negative side of coordinate system).
    // pub min: V3<f32>,
    /// Represents cube's vertex that is upper-front-right  (on the positive side of coordinate system).
    // pub max: V3<f32>,
    pub x: Interval,
    pub y: Interval,
    pub z: Interval,
}

static DELTA: f32 = 0.0001f32;

impl Aabb {
    /// Checks if Ray intersects with this bounding box.
    pub fn hit(&self, ray: &Ray) -> bool {
        let inv_end = V3::new(1.0, 1.0, 1.0).div(ray.end);

        let tx0 = (self.x.min - ray.origin.x) * inv_end.x;
        let tx1 = (self.x.max - ray.origin.x) * inv_end.x;
        let ntx0 = tx0.min(tx1);
        let ntx1 = tx0.max(tx1);

        let ty0 = (self.y.min - ray.origin.y) * inv_end.y;
        let ty1 = (self.y.max - ray.origin.y) * inv_end.y;
        let nty0 = ty0.min(ty1);
        let nty1 = ty0.max(ty1);

        let lol0 = ntx0.max(nty0);
        let lol1 = ntx1.min(nty1);

        if lol0 > lol1 {
            return false;
        }

        let tz0 = (self.z.min - ray.origin.z) * inv_end.z;
        let tz1 = (self.z.max - ray.origin.z) * inv_end.z;

        let ntz0 = tz0.min(tz1);
        let ntz1 = tz0.max(tz1);

        let lolol0 = ntz0.max(lol0);
        let lolol1 = ntz1.min(lol1);

        lolol0 <= lolol1
    }

    /// Creates new bounding box that surrounds two bounding boxes.
    pub fn surrounding_box(&self, second: Aabb) -> Self {
        Aabb {
            x: Interval {
                min: self.x.min.min(second.x.min),
                max: self.x.max.max(second.x.max),
            },
            y: Interval {
                min: self.y.min.min(second.y.min),
                max: self.y.max.max(second.y.max),
            },
            z: Interval {
                min: self.z.min.min(second.z.min),
                max: self.z.max.max(second.z.max),
            },
        }
    }

    /// Returns list of verticles of this bounding box.
    pub fn get_box_points(&self) -> Vec<V3<f32>> {
        iproduct!(0..=1, 0..=1, 0..=1)
            .map(|point| {
                V3::new(
                    ((point.0 as f32) * self.x.min) + ((1.0 - (point.0 as f32)) * self.x.max),
                    ((point.1 as f32) * self.y.min) + ((1.0 - (point.1 as f32)) * self.y.max),
                    ((point.2 as f32) * self.z.min) + ((1.0 - (point.2 as f32)) * self.z.max),
                )
            })
            .collect()
    }

    /// Add padding to this bounding box to overcome numerical problems
    pub fn pad(&self) -> Self {
        Aabb {
            x: if self.x.size() > DELTA {
                self.x
            } else {
                self.x.add_padding(DELTA)
            },
            y: if self.y.size() > DELTA {
                self.y
            } else {
                self.y.add_padding(DELTA)
            },
            z: if self.z.size() > DELTA {
                self.z
            } else {
                self.z.add_padding(DELTA)
            },
        }
    }
}
