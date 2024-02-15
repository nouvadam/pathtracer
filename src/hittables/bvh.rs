use crate::hit::*;
use crate::hittables::{Aabb, HittableList};
use crate::misc::{HittablePdf, Pdf};
use crate::ray::*;
use rand::Rng;

#[derive(Clone)]
/// Octtree that contains Scene, for checking ray intersections in O(nlog(n)) time rather than in O(n^2)
pub struct BvhNode {
    boxx: Aabb,
    left: Box<dyn HittablePdf>,
    right: Box<dyn HittablePdf>,
}

impl Hittable for BvhNode {
    fn hit(&self, ray: &Ray) -> Option<Hit> {
        if self.boxx.hit(ray) {
            let hit_right = self.right.hit(ray);

            match self.left.hit(ray) {
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
                        .x
                        .min
                        .partial_cmp(&b.bounding_box().x.min)
                        .unwrap()
                });
            }
            1 => {
                sorted.sort_by(|a, b| {
                    a.bounding_box()
                        .y
                        .min
                        .partial_cmp(&b.bounding_box().y.min)
                        .unwrap()
                });
            }
            2 => {
                sorted.sort_by(|a, b| {
                    a.bounding_box()
                        .z
                        .min
                        .partial_cmp(&b.bounding_box().z.min)
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
                left: sorted[0].clone(),
                right: sorted[0].clone(),
            },
            2 => BvhNode {
                boxx: sorted[0]
                    .bounding_box()
                    .surrounding_box(sorted[1].bounding_box()),
                left: sorted[0].clone(),
                right: sorted[1].clone(),
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
                    left: Box::new(left),
                    right: Box::new(right),
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
