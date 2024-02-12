use crate::hit::*;
use crate::hittables::Aabb;
use crate::misc::{HittablePdf, Pdf};
use crate::ray::Ray;

/// hittable objects aggregated into list.
#[derive(Clone, Default)]
pub struct HittableList {
    /// Underlying list of hittable objects
    pub list: Vec<Box<dyn HittablePdf>>,
}

impl HittableList {
    /// Creates new list
    pub fn new() -> HittableList {
        HittableList { list: Vec::new() }
    }

    /// Adds object to the list
    pub fn add<T: HittablePdf + 'static>(&mut self, object: T) {
        self.list.push(Box::new(object));
    }
}

impl Hittable for HittableList {
    fn hit(&self, ray: &Ray) -> Option<Hit> {
        self.list
            .iter()
            .filter_map(|obj| obj.hit(ray)) //get objects that were passed though by the ray
            .min_by(|x, y| x.t.partial_cmp(&y.t).expect("Tried to compare a NaN"))
        //choose the closest object to the cam, the one that is visible
    }

    // Adds consequtive objects from list to the bounding box, to create bounding box for the whole scene
    fn bounding_box(&self) -> Aabb {
        self.list.iter().fold(
            self.list[0].bounding_box(),
            |surrounded_objects, current_object| {
                surrounded_objects.surrounding_box(current_object.bounding_box())
            },
        )
    }
}

impl Pdf for HittableList {
    fn value(&self, origin: crate::V3<f32>, direction: crate::V3<f32>) -> f32 {
        use std::convert::TryFrom;

        let weight: f32 = 1.0f32
            / f32::try_from(self.list.len() as u16)
                .expect("HittableList PDF value generation critically failed.");
        let weighted_sum_of_probs = self.list.iter().fold(0.0, |acc, object| {
            acc + object.value(origin, direction) * weight
        });

        weighted_sum_of_probs
    }

    fn generate(&self, origin: crate::V3<f32>) -> crate::V3<f32> {
        use rand::seq::SliceRandom;
        let mut rng = rand::thread_rng();

        self.list
            .choose(&mut rng)
            .expect("HittableList is empty, thus cannot generate random direction toward it.")
            .generate(origin)
    }
}
