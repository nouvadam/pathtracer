use crate::hit::*;
use crate::hitables::AABB;
use crate::material::Material;
use crate::ray::*;
use crate::V3;
/// Sphere primitive.
#[derive(Clone)]
pub struct Sphere {
    /// Center of the sphere.
    pub center: V3<f32>,
    /// Radius of the sphere.
    pub radius: f32,
    /// Material of the sphere.
    pub material: Box<dyn Material + Sync + Send>,
}

impl Hitable for Sphere {
    fn hit(&self, r: &Ray, t_min: f32, t_max: f32) -> Option<Hit> {
        let oc = r.origin - self.center;
        let a = r.end.dot(r.end);
        let b = oc.dot(r.end);
        let c = oc.dot(oc) - self.radius * self.radius;
        let discriminant = b * b - a * c;

        if discriminant > 0.0 {
            let t;
            let t1 = (-b - (b * b - a * c).sqrt()) / a;
            let t2 = (-b + (b * b - a * c).sqrt()) / a;

            if t1 < t_max && t1 > t_min {
                t = t1;
            } else if t2 < t_max && t2 > t_min {
                t = t2;
            } else {
                return None;
            }

            let point = r.point_at_param(t);
            let normal = (point - self.center) / self.radius;
            let pi = std::f32::consts::PI;

            let sphere_point = (point - self.center).norm();

            let phi = (sphere_point.z).atan2(sphere_point.x);
            let theta = (sphere_point.y).asin();
            let u = 1.0 - (phi + pi) / (2.0 * pi);
            let v = (theta + pi / 2.0) / pi;

            Some(Hit::new(r, normal, t, point, &*self.material, u, v))
        } else {
            None
        }
    }

    fn bounding_box(&self) -> AABB {
        let vektor = V3::new(self.radius, self.radius, self.radius);

        AABB {
            min: self.center - vektor,
            max: self.center + vektor,
        }
    }
}
