use crate::hit::*;
use crate::hitables::AABB;
use crate::ray::Ray;

/// Hitable objects aggregated into list.
#[derive(Clone, Default)]
pub struct HitableList {
    /// Underlying list of Hitable objects
    pub list: Vec<Box<dyn Hitable>>,
}

impl HitableList {
    /// Creates new list
    pub fn new() -> HitableList {
        HitableList { list: Vec::new() }
    }

    /// Adds object to the list
    pub fn add(&mut self, object: Box<dyn Hitable>) {
        self.list.push(object);
    }
}

impl Hitable for HitableList {
    fn hit(&self, r: &Ray, t_min: f32, t_max: f32) -> Option<Hit> {
        self.list
            .iter()
            .filter_map(|obj| obj.hit(r, t_min, t_max)) //get objects that were passed though by the ray
            .min_by(|x, y| x.t.partial_cmp(&y.t).expect("Tried to compare a NaN"))
        //choose the closest object to the cam, the one that is visible
    }

    // Adds consequtive objects from list to the bounding box, to create bounding box for the whole scene
    fn bounding_box(&self) -> AABB {
        self.list.iter().fold(
            self.list[0].bounding_box(),
            |surrounded_objects, current_object| {
                surrounded_objects.surrounding_box(current_object.bounding_box())
            },
        )
    }
}
