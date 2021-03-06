use crate::hit::*;
use crate::hittables::{Aabb, HittableList};
use crate::misc::Pdf;
use crate::ray::*;
use rand::Rng;

#[derive(Clone)]
/// Octtree that contains Scene, for checking ray intersections in O(nlog(n)) time rather than in O(n^2)
pub struct BvhNode {
    boxx: Aabb,
    left: Box<Primitive>,
    right: Box<Primitive>,
}

impl Hittable for BvhNode {
    fn hit(&self, r: &Ray, t_min: f32, t_max: f32) -> Option<Hit> {
        if self.boxx.hit(r) {
            let hit_right = self.right.hit(r, t_min, t_max);

            match self.left.hit(r, t_min, t_max) {
                Some(hit_left) => match hit_right {
                    Some(hit_right) => {
                        if hit_left.t < hit_right.t {
                            Some(hit_left)
                        } else {
                            Some(hit_right)
                        }
                    }
                    None => Some(hit_left),
                },
                None => hit_right,
            }
        } else {
            None
        }
    }

    fn bounding_box(&self) -> Aabb {
        self.boxx.clone()
    }
}

impl BvhNode {
    /// Tree is created by recursively dividing the scene, in form of hittableList, in half by plane aligned to each time randomly choosen axis
    pub fn new(hlist: &HittableList) -> BvhNode {
        let rng = rand::thread_rng().gen_range(0, 2);

        let mut sorted = hlist.list.clone();

        match rng % 2 {
            0 => {
                sorted.sort_by(|a, b| {
                    a.bounding_box()
                        .min
                        .x
                        .partial_cmp(&b.bounding_box().min.x)
                        .unwrap()
                });
            }
            1 => {
                sorted.sort_by(|a, b| {
                    a.bounding_box()
                        .min
                        .y
                        .partial_cmp(&b.bounding_box().min.y)
                        .unwrap()
                });
            }
            2 => {
                sorted.sort_by(|a, b| {
                    a.bounding_box()
                        .min
                        .z
                        .partial_cmp(&b.bounding_box().min.z)
                        .unwrap()
                });
            }
            _ => (),
        }

        let sorted_length = sorted.len();
        match sorted_length {
            1 => BvhNode {
                boxx: sorted[0]
                    .bounding_box()
                    .surrounding_box(sorted[0].bounding_box()),
                left: Box::new(sorted[0].clone()),
                right: Box::new(sorted[0].clone()),
            },
            2 => BvhNode {
                boxx: sorted[0]
                    .bounding_box()
                    .surrounding_box(sorted[1].bounding_box()),
                left: Box::new(sorted[0].clone()),
                right: Box::new(sorted[1].clone()),
            },
            _ => {
                let left = BvhNode::new(&HittableList {
                    list: sorted[0..sorted_length / 2].to_vec(),
                });
                let right = BvhNode::new(&HittableList {
                    list: sorted[sorted_length / 2..sorted_length].to_vec(),
                });
                BvhNode {
                    boxx: left.boxx.surrounding_box(right.boxx.clone()),
                    left: Box::new(Primitive::BvhNode(left)),
                    right: Box::new(Primitive::BvhNode(right)),
                }
            }
        }
    }
}

impl Pdf for BvhNode {
    fn value(&self, _origin: crate::V3<f32>, _direction: crate::V3<f32>) -> f32 {
        todo!()
    }

    fn generate(&self, _origin: crate::V3<f32>) -> crate::V3<f32> {
        todo!()
    }
}
