use crate::hit::*;
use crate::hittables::Aabb;
use crate::misc::Pdf;
use crate::ray::*;
use crate::V3;
/// Primitive representing sphere that moves during some time interval.
#[derive(Clone)]
pub struct MovingSphere {
    /// Centers of the sphere between which moves, first one is the starting point, second in the ending point.
    pub centers: (V3<f32>, V3<f32>),
    /// Time interval during which sphere moves from the start to the end.
    pub time_range: (f32, f32),
    /// Radius of the sphere.
    pub radius: f32,
    /// Material of the sphere.
    pub material: usize,
}

impl MovingSphere {
    fn center(&self, time: f32) -> V3<f32> {
        self.centers.0
            + (self.centers.1 - self.centers.0)
                * ((time - self.time_range.0) / (self.time_range.1 - self.time_range.0))
    }

    /// Creates new MovingSphere primitive.
    pub fn new(
        centers: (V3<f32>, V3<f32>),
        time_range: (f32, f32),
        radius: f32,
        material: usize,
    ) -> Self {
        Self {
            centers,
            time_range,
            radius,
            material,
        }
    }
}

impl Hittable for MovingSphere {
    fn hit(&self, ray: &Ray) -> Option<Hit> {
        let oc = ray.origin - self.center(ray.time);
        let a = ray.end.dot(ray.end);
        let b = oc.dot(ray.end);
        let c = oc.dot(oc) - self.radius * self.radius;
        let discriminant = b * b - a * c;

        if discriminant > 0.0 {
            let time;
            let t1 = (-b - (b * b - a * c).sqrt()) / a;
            let t2 = (-b + (b * b - a * c).sqrt()) / a;

            if ray.setting.ray_time.surrounds(t1) {
                time = t1;
            } else if ray.setting.ray_time.surrounds(t2) {
                time = t2;
            } else {
                return None;
            }

            let point = ray.point_at_param(time);
            let normal = (point - self.center(ray.time)) / self.radius;

            let sphere_point = (point - self.center(ray.time)).norm();

            let phi = (sphere_point.z).atan2(sphere_point.x);
            let theta = (sphere_point.y).asin();
            let u = 1.0 - (phi + std::f32::consts::PI) / (2.0 * std::f32::consts::PI);
            let v = (theta + std::f32::consts::PI / 2.0) / std::f32::consts::PI;

            Some(Hit::new(ray, normal, time, point, self.material, u, v))
        } else {
            None
        }
    }

    fn bounding_box(&self) -> Aabb {
        let t0_aabb = Aabb {
            min: self.center(self.time_range.0) - V3::new(self.radius, self.radius, self.radius),
            max: self.center(self.time_range.0) + V3::new(self.radius, self.radius, self.radius),
        };

        let t1_aabb = Aabb {
            min: self.center(self.time_range.1) - V3::new(self.radius, self.radius, self.radius),
            max: self.center(self.time_range.1) + V3::new(self.radius, self.radius, self.radius),
        };

        t0_aabb.surrounding_box(t1_aabb)
    }
}

impl Pdf for MovingSphere {
    fn value(&self, _origin: V3<f32>, _direction: V3<f32>) -> f32 {
        todo!()
    }

    fn generate(&self, _origin: V3<f32>) -> V3<f32> {
        todo!()
    }
}
