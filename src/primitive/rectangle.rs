use crate::hit::*;
use crate::hittables::Aabb;
use crate::misc::Pdf;
use crate::ray::*;
use crate::V3;

const EPSILON: f32 = 0.001;
/// Rectangle aligned to XY axis.
#[derive(Clone)]
pub struct XYrect {
    /// Starting boundary on X axis.
    pub x0: f32,
    /// Ending boundary on X axis.
    pub x1: f32,
    /// Starting boundary on Y axis.
    pub y0: f32,
    /// Ending boundary on Y axis.
    pub y1: f32,
    /// Position on Z axis.
    pub k: f32,
    /// Material of the rectangle.
    pub material: usize,
}

impl Hittable for XYrect {
    fn hit(&self, r: &Ray, t_min: f32, t_max: f32) -> Option<Hit> {
        let t = (self.k - r.origin.z) / r.end.z;

        if t.is_nan() {
            return None;
        }

        if t < t_min || t > t_max {
            return None;
        }

        let x = r.origin.x + t * r.end.x;
        let y = r.origin.y + t * r.end.y;

        if x < self.x0 || x > self.x1 || y < self.y0 || y > self.y1 {
            return None;
        }

        let outward_normal = V3::new(0.0, 0.0, 1.0);

        Some(Hit::new(
            r,
            outward_normal,
            t,
            r.point_at_param(t),
            self.material,
            (x - self.x0) / (self.x1 - self.x0),
            (y - self.y0) / (self.y1 - self.y0),
        ))
    }

    fn bounding_box(&self) -> Aabb {
        Aabb {
            min: V3::new(self.x0, self.y0, self.k - EPSILON),
            max: V3::new(self.x1, self.y1, self.k + EPSILON),
        }
    }
}

impl XYrect {
    /// Creates new Xyrect primitive.
    pub fn new(x0: f32, x1: f32, y0: f32, y1: f32, k: f32, material: usize) -> Self {
        XYrect {
            x0,
            x1,
            y0,
            y1,
            k,
            material,
        }
    }
}

/// Rectangle aligned to XZ axis.
#[derive(Clone)]
pub struct XZrect {
    /// Starting boundary on X axis.
    pub x0: f32,
    /// Ending boundary on X axis.
    pub x1: f32,
    /// Starting boundary on Z axis.
    pub z0: f32,
    /// Ending boundary on Z axis.
    pub z1: f32,
    /// Position on Y axis.
    pub k: f32,
    /// Material of the rectangle.
    pub material: usize,
}

impl Hittable for XZrect {
    fn hit(&self, r: &Ray, t_min: f32, t_max: f32) -> Option<Hit> {
        let t = (self.k - r.origin.y) / r.end.y;

        if t.is_nan() {
            return None;
        }

        if t < t_min || t > t_max {
            return None;
        }

        let x = r.origin.x + t * r.end.x;
        let z = r.origin.z + t * r.end.z;

        if x < self.x0 || x > self.x1 || z < self.z0 || z > self.z1 {
            return None;
        }

        let outward_normal = V3::new(0.0, 1.0, 0.0);

        Some(Hit::new(
            r,
            outward_normal,
            t,
            r.point_at_param(t),
            self.material,
            (x - self.x0) / (self.x1 - self.x0),
            (z - self.z0) / (self.z1 - self.z0),
        ))
    }

    fn bounding_box(&self) -> Aabb {
        Aabb {
            min: V3::new(self.x0, self.k - EPSILON, self.z0),
            max: V3::new(self.x1, self.k + EPSILON, self.z1),
        }
    }
}

impl XZrect {
    /// Creates new Xyrect primitive.
    pub fn new(x0: f32, x1: f32, z0: f32, z1: f32, k: f32, material: usize) -> Self {
        XZrect {
            x0,
            x1,
            z0,
            z1,
            k,
            material,
        }
    }
}

/// Rectangle aligned to YZ axis.
#[derive(Clone)]
pub struct YZrect {
    /// Starting boundary on Y axis.
    pub y0: f32,
    /// Ending boundary on Y axis.
    pub y1: f32,
    /// Starting boundary on Z axis.
    pub z0: f32,
    /// Ending boundary on Z axis.
    pub z1: f32,
    /// Position on X axis.
    pub k: f32,
    /// Material of the rectangle.
    pub material: usize,
}

impl Hittable for YZrect {
    fn hit(&self, r: &Ray, t_min: f32, t_max: f32) -> Option<Hit> {
        let t = (self.k - r.origin.x) / r.end.x;

        if t.is_nan() {
            return None;
        }

        if t < t_min || t > t_max {
            return None;
        }

        let y = r.origin.y + t * r.end.y;
        let z = r.origin.z + t * r.end.z;

        if y < self.y0 || y > self.y1 || z < self.z0 || z > self.z1 {
            return None;
        }

        let outward_normal = V3::new(1.0, 0.0, 0.0);

        Some(Hit::new(
            r,
            outward_normal,
            t,
            r.point_at_param(t),
            self.material,
            (y - self.y0) / (self.y1 - self.y0),
            (z - self.z0) / (self.z1 - self.z0),
        ))
    }

    fn bounding_box(&self) -> Aabb {
        Aabb {
            min: V3::new(self.k - EPSILON, self.y0, self.z0),
            max: V3::new(self.k + EPSILON, self.y1, self.z1),
        }
    }
}

impl YZrect {
    /// Creates new Xyrect primitive.
    pub fn new(y0: f32, y1: f32, z0: f32, z1: f32, k: f32, material: usize) -> Self {
        YZrect {
            y0,
            y1,
            z0,
            z1,
            k,
            material,
        }
    }
}

impl Pdf for XZrect {
    fn value(&self, origin: V3<f32>, direction: V3<f32>) -> f32 {
        let ray = Ray {
            origin,
            end: direction,
            time: 1.0,
            setting: &RaySetting {
                depth: 32,
                background_color: V3::zero(),
            },
        };

        match self.hit(&ray, 0.001, 2048.0) {
            Some(hit) => {
                let area = (self.x1 - self.x0) * (self.z1 - self.z0);
                let distance_squared = hit.t * hit.t * direction.length().powi(2);
                let cosine = (direction.dot(hit.normal) / direction.length()).abs();
                distance_squared / (cosine * area)
            }
            None => 0.0,
        }
    }

    fn generate(&self, origin: V3<f32>) -> V3<f32> {
        //generate random point on XZ rectangle.
        use rand::Rng;
        let random_point = V3::new(
            rand::thread_rng().gen_range(self.x0, self.x1),
            self.k,
            rand::thread_rng().gen_range(self.z0, self.z1),
        );
        random_point - origin
    }
}

impl Pdf for XYrect {
    fn value(&self, origin: V3<f32>, direction: V3<f32>) -> f32 {
        let ray = Ray {
            origin,
            end: direction,
            time: 1.0,
            setting: &RaySetting {
                depth: 32,
                background_color: V3::zero(),
            },
        };

        match self.hit(&ray, 0.001, 2048.0) {
            Some(hit) => {
                let area = (self.x1 - self.x0) * (self.y1 - self.y0);
                let distance_squared = hit.t * hit.t * direction.length().powi(2);
                let cosine = (direction.dot(hit.normal) / direction.length()).abs();
                distance_squared / (cosine * area)
            }
            None => 0.0,
        }
    }

    fn generate(&self, origin: V3<f32>) -> V3<f32> {
        //generate random point on XZ rectangle.
        use rand::Rng;
        let random_point = V3::new(
            rand::thread_rng().gen_range(self.x0, self.x1),
            rand::thread_rng().gen_range(self.y0, self.y1),
            self.k,
        );
        random_point - origin
    }
}

impl Pdf for YZrect {
    fn value(&self, origin: V3<f32>, direction: V3<f32>) -> f32 {
        let ray = Ray {
            origin,
            end: direction,
            time: 1.0,
            setting: &RaySetting {
                depth: 32,
                background_color: V3::zero(),
            },
        };

        match self.hit(&ray, 0.001, 2048.0) {
            Some(hit) => {
                let area = (self.y1 - self.y0) * (self.z1 - self.z0);
                let distance_squared = hit.t * hit.t * direction.length().powi(2);
                let cosine = (direction.dot(hit.normal) / direction.length()).abs();
                distance_squared / (cosine * area)
            }
            None => 0.0,
        }
    }

    fn generate(&self, origin: V3<f32>) -> V3<f32> {
        //generate random point on XZ rectangle.
        use rand::Rng;
        let random_point = V3::new(
            self.k,
            rand::thread_rng().gen_range(self.y0, self.y1),
            rand::thread_rng().gen_range(self.z0, self.z1),
        );
        random_point - origin
    }
}
