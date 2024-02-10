use crate::hit::*;
use crate::hittables::Aabb;
use crate::misc::Onb;
use crate::misc::Pdf;
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
    pub material: usize,
}

impl Hittable for Sphere {
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

            Some(Hit::new(r, normal, t, point, self.material, u, v))
        } else {
            None
        }
    }

    fn bounding_box(&self) -> Aabb {
        let vektor = V3::new(self.radius, self.radius, self.radius);

        Aabb {
            min: self.center - vektor,
            max: self.center + vektor,
        }
    }
}

impl Sphere {
    /// Creates new Xyrect primitive.
    pub fn new(center: V3<f32>, radius: f32, material: usize) -> Self {
        Sphere {
            center,
            radius,
            material,
        }
    }
}

impl Pdf for Sphere {
    fn value(&self, origin: V3<f32>, direction: V3<f32>) -> f32 {
        let ray = Ray {
            origin,
            end: direction,
            time: 1.0,
            setting: &RaySetting {
                depth: 32,
                background_color: V3::default(),
            },
        };

        match self.hit(&ray, 0.001, 2048.0) {
            Some(_hit) => {
                let cos_theta_max =
                    (1.0 - (self.radius.powi(2) / (self.center - origin).length().powi(2))).sqrt();
                let solid_angle = 2.0 * std::f32::consts::PI * (1.0 - cos_theta_max);

                1.0 / solid_angle
            }
            None => 0.0,
        }
    }

    fn generate(&self, origin: V3<f32>) -> V3<f32> {
        let direction = self.center - origin;
        let distance_squared = direction.length().powi(2);
        let uvw = Onb::build_from_w(&direction);

        uvw.local_from_vec(&random_to_sphere(self.radius, distance_squared))
    }
}

fn random_to_sphere(radius: f32, distance_squared: f32) -> V3<f32> {
    use rand::Rng;
    let mut rng = rand::thread_rng();

    let r1 = rng.gen::<f32>();
    let r2 = rng.gen::<f32>();

    let z = 1.0 + r2 * ((1.0 - radius.powi(2) / distance_squared).sqrt() - 1.0);

    let phi = 2.0 * std::f32::consts::PI * r1;

    let temp = (1.0f32 - z.powi(2)).sqrt();

    let x = phi.cos() * temp;
    let y = phi.sin() * temp;

    V3::new(x, y, z)
}
