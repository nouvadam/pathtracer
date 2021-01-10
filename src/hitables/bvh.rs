use crate::hit::*;
use crate::hitables::{HitableList, AABB};
use crate::ray::*;
use rand::Rng;

#[derive(Clone)]
/// Octtree that contains Scene, for checking ray intersections in O(nlog(n)) time rather than in O(n^2)
pub struct BvhNode {
    boxx: AABB,
    left: Box<dyn Hitable>,
    right: Box<dyn Hitable>,
}

impl Hitable for BvhNode {
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
                None => match hit_right {
                    Some(hit_right) => Some(hit_right),
                    None => None,
                },
            }
        } else {
            None
        }
    }

    fn bounding_box(&self) -> AABB {
        self.boxx.clone()
    }
}

impl BvhNode {
    /// Tree is created by recursively dividing the scene, in form of HitableList, in half by plane aligned to each time randomly choosen axis
    pub fn new(hlist: HitableList) -> BvhNode {
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

        let sorted_len = sorted.len();
        match sorted_len {
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
                let left = BvhNode::new(HitableList {
                    list: sorted[0..sorted_len / 2].to_vec(),
                });
                let right = BvhNode::new(HitableList {
                    list: sorted[sorted_len / 2..sorted_len].to_vec(),
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
