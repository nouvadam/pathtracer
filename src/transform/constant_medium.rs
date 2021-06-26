use crate::hittables::Aabb;
use crate::misc::Pdf;
use crate::Primitive;
use crate::{Hit, Hittable, Ray, V3};

use rand::Rng;
/// Struct representing some primitive like Sphere that was changed into some sort of smoke/fog/mist.
#[derive(Clone)]
pub struct ConstantMedium {
    /// The primitive that was transformed into smoke.
    boundary: Box<Primitive>,
    /// Some material that gives color when Ray hits the object.
    phase_function: usize,
    /// Some sort of inversed probability that Ray will scatter inside the object.
    neg_inv_density: f32,
}

impl Hittable for ConstantMedium {
    fn hit(&self, r: &Ray, t_min: f32, t_max: f32) -> Option<Hit> {
        match self.boundary.hit(r, f32::NEG_INFINITY, f32::INFINITY) {
            Some(mut first_hit) => {
                // if the ray hit the object at all
                match self.boundary.hit(r, first_hit.t + 0.0001, f32::INFINITY) {
                    Some(mut second_hit) => {
                        // when ray hits some particle inside the object
                        if first_hit.t < t_min {
                            first_hit.t = t_min
                        };
                        if second_hit.t > t_max {
                            second_hit.t = t_max
                        };
                        if first_hit.t >= second_hit.t {
                            return None;
                        };
                        if first_hit.t < 0.0 {
                            first_hit.t = 0.0;
                        }

                        let ray_length = r.end.length();
                        let distance_inside_boundary = (second_hit.t - first_hit.t) * ray_length;
                        let random: f32 = rand::thread_rng().gen_range(0.0, 1.0);
                        let hit_distance = self.neg_inv_density * random.ln();

                        if hit_distance > distance_inside_boundary {
                            return None;
                        };

                        Some(Hit::new(
                            r,
                            V3::new(1.0, 0.0, 0.0),
                            first_hit.t + hit_distance / ray_length,
                            r.point_at_param(first_hit.t + hit_distance / ray_length),
                            self.phase_function,
                            0.0,
                            0.0,
                        ))
                    }
                    None => None,
                }
            }
            None => None,
        }
    }

    fn bounding_box(&self) -> Aabb {
        self.boundary.bounding_box()
    }
}
/// Transforms object into ConstantMedium object
pub trait IntoConstantMedium {
    /// Transforms object into Rotated object
    ///
    /// `neg_inv_density` - Inversed probability that Ray will scatter inside the object.
    ///
    /// `phase_function` - Material that gives color when Ray scatter inside the object.
    fn into_constant_medium(self, neg_inv_density: f32, phase_function: usize) -> Primitive;
}

impl IntoConstantMedium for Primitive {
    fn into_constant_medium(self, neg_inv_density: f32, phase_function: usize) -> Primitive {
        Primitive::ConstantMedium(ConstantMedium {
            boundary: Box::new(self),
            phase_function,
            neg_inv_density: -1.0 / neg_inv_density,
        })
    }
}

impl Pdf for ConstantMedium {
    fn value(&self, origin: crate::V3<f32>, direction: crate::V3<f32>) -> f32 {
        self.boundary.value(origin, direction)
    }

    fn generate(&self, origin: crate::V3<f32>) -> crate::V3<f32> {
        self.boundary.generate(origin)
    }
}
