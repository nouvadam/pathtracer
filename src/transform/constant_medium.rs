use crate::hittables::Aabb;
use crate::misc::{HittablePdf, Interval, Pdf};
use crate::{Hit, Hittable, Ray, RaySetting, V3};

use rand::Rng;
/// Struct representing some primitive like Sphere that was changed into some sort of smoke/fog/mist.
#[derive(Clone)]
pub struct ConstantMedium {
    /// The primitive that was transformed into smoke.
    boundary: Box<dyn HittablePdf>,
    /// Some material that gives color when Ray hits the object.
    phase_function: usize,
    /// Some sort of inversed probability that Ray will scatter inside the object.
    neg_inv_density: f32,
}

const UNIVERSE: Interval = Interval::universe();

impl Hittable for ConstantMedium {
    fn hit(&self, ray: &Ray) -> Option<Hit> {
        match self.boundary.hit(&Ray {
            setting: &RaySetting {
                ray_time: UNIVERSE,
                ..*ray.setting
            },
            ..*ray
        }) {
            Some(mut first_hit) => {
                // if the ray hit the object at all
                match self.boundary.hit(&Ray {
                    setting: &RaySetting {
                        ray_time: Interval {
                            min: first_hit.t + 0.0001,
                            max: f32::INFINITY,
                        },
                        ..*ray.setting
                    },
                    ..*ray
                }) {
                    Some(mut second_hit) => {
                        // when ray hits some particle inside the object
                        if first_hit.t < ray.setting.ray_time.min {
                            first_hit.t = ray.setting.ray_time.min
                        };
                        if second_hit.t > ray.setting.ray_time.max {
                            second_hit.t = ray.setting.ray_time.max
                        };
                        if first_hit.t >= second_hit.t {
                            return None;
                        };
                        if first_hit.t < 0.0 {
                            first_hit.t = 0.0;
                        }

                        let ray_length = ray.end.length();
                        let distance_inside_boundary = (second_hit.t - first_hit.t) * ray_length;
                        let random: f32 = rand::thread_rng().gen_range(0.0..1.0);
                        let hit_distance = self.neg_inv_density * random.ln();

                        if hit_distance > distance_inside_boundary {
                            return None;
                        };

                        Some(Hit::new(
                            ray,
                            V3::new(1.0, 0.0, 0.0),
                            first_hit.t + hit_distance / ray_length,
                            ray.point_at_param(first_hit.t + hit_distance / ray_length),
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
    fn into_constant_medium(self, neg_inv_density: f32, phase_function: usize) -> ConstantMedium;
}

impl<T> IntoConstantMedium for T
where
    T: HittablePdf + 'static,
{
    fn into_constant_medium(self, neg_inv_density: f32, phase_function: usize) -> ConstantMedium {
        ConstantMedium {
            boundary: Box::new(self),
            phase_function,
            neg_inv_density: -1.0 / neg_inv_density,
        }
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
