use crate::hit::*;
use crate::hittables::Aabb;
use crate::misc::Interval;
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
    fn hit(&self, ray: &Ray) -> Option<Hit> {
        let t = (self.k - ray.origin.z) / ray.end.z;

        if t.is_nan() {
            return None;
        }

        if t < ray.setting.ray_time.min || t > ray.setting.ray_time.max {
            return None;
        }

        let x = ray.origin.x + t * ray.end.x;
        let y = ray.origin.y + t * ray.end.y;

        if x < self.x0 || x > self.x1 || y < self.y0 || y > self.y1 {
            return None;
        }

        let outward_normal = V3::new(0.0, 0.0, 1.0);

        Some(Hit::new(
            ray,
            outward_normal,
            t,
            ray.point_at_param(t),
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
    fn hit(&self, ray: &Ray) -> Option<Hit> {
        let t = (self.k - ray.origin.y) / ray.end.y;

        if t.is_nan() {
            return None;
        }

        if t < ray.setting.ray_time.min || t > ray.setting.ray_time.max {
            return None;
        }

        let x = ray.origin.x + t * ray.end.x;
        let z = ray.origin.z + t * ray.end.z;

        if x < self.x0 || x > self.x1 || z < self.z0 || z > self.z1 {
            return None;
        }

        let outward_normal = V3::new(0.0, 1.0, 0.0);

        Some(Hit::new(
            ray,
            outward_normal,
            t,
            ray.point_at_param(t),
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
    fn hit(&self, ray: &Ray) -> Option<Hit> {
        let t = (self.k - ray.origin.x) / ray.end.x;

        if t.is_nan() {
            return None;
        }

        if t < ray.setting.ray_time.min || t > ray.setting.ray_time.max {
            return None;
        }

        let y = ray.origin.y + t * ray.end.y;
        let z = ray.origin.z + t * ray.end.z;

        if y < self.y0 || y > self.y1 || z < self.z0 || z > self.z1 {
            return None;
        }

        let outward_normal = V3::new(1.0, 0.0, 0.0);

        Some(Hit::new(
            ray,
            outward_normal,
            t,
            ray.point_at_param(t),
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
                background_color: V3::default(),
                ray_time: Interval {
                    min: 0.001,
                    max: 2048.0,
                },
            },
        };

        match self.hit(&ray) {
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
                background_color: V3::default(),
                ray_time: Interval {
                    min: 0.001,
                    max: 2048.0,
                },
            },
        };

        match self.hit(&ray) {
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
                background_color: V3::default(),
                ray_time: Interval {
                    min: 0.001,
                    max: 2048.0,
                },
            },
        };

        match self.hit(&ray) {
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
