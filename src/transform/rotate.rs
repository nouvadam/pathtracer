use crate::hitables::AABB;
use crate::{Hit, Hitable, Ray, V3};

/// Represents a Hitable object that was rotated by some angle around some axis.
#[derive(Clone)]
pub struct Rotated {
    /// Sinus of the angle.
    sin_theta: f32,
    /// Cosinus of the angle.
    cos_theta: f32,
    /// Axis around which should the object be rotated.
    axis: V3<f32>,
    /// Bounding box of the rotated object.
    bounding_box: AABB,
    /// Object that is rotated.
    hitable: Box<dyn Hitable>,
}

impl Hitable for Rotated {
    fn hit(&self, r: &Ray, t_min: f32, t_max: f32) -> Option<Hit> {
        let origin = r.origin.rot(-self.axis, self.sin_theta, self.cos_theta);
        let end = r.end.rot(-self.axis, self.sin_theta, self.cos_theta);

        let rotated_ray = Ray { origin, end, ..*r };

        let hit = self.hitable.hit(&rotated_ray, t_min, t_max);

        match hit {
            Some(hit) => Some(Hit::new(
                &rotated_ray,
                hit.normal.rot(self.axis, self.sin_theta, self.cos_theta),
                hit.t,
                hit.point.rot(self.axis, self.sin_theta, self.cos_theta),
                hit.material,
                hit.u,
                hit.v,
            )),
            None => None,
        }
    }

    fn bounding_box(&self) -> AABB {
        self.bounding_box.clone()
    }
}
/// Transforms object into Rotated object
pub trait IntoRotated {
    /// Transforms object into Rotated object
    fn rotate(self, axis: V3<f32>, angle: f32) -> Box<Rotated>;
}

// All Hitable objects now implements 'rotate' method, that takes ownership of underlying Hitable object, creates new Rotated object that surrounds Hitable with rotation info, and returns that new object
impl<T: 'static + Hitable> IntoRotated for Box<T> {
    fn rotate(self, axis: V3<f32>, angle: f32) -> Box<Rotated> {
        let angle = angle / 2.0;

        // Needed for quaterion rotation
        let sin_theta = angle.sin();
        let cos_theta = angle.cos();

        use crate::misc::IntoMultizip;

        // Creating new bounding box, aligned with the coordinate system, from rotated bounding box

        // Rotate verticles of Bounding Box of underlying Hitable object, and multizip verticles' axis to 3 vectors
        let rotated_and_zipped_points = self
            .bounding_box()
            .get_box_points()
            .iter()
            .map(|point| point.rot(axis, sin_theta, cos_theta))
            .map(|element| element.into_iter()) // casting V3 tuple to iter
            .multizip(); // creating 3 vectors which elements are from the same coordinate - first vector contain all x component of verticles etc

        // Find minimum values in each axis vector
        let min: V3<f32> = rotated_and_zipped_points
            .clone()
            .map(|axis| {
                axis.into_iter()
                    .min_by(|component0, component1| {
                        component0
                            .partial_cmp(component1)
                            .expect("Tried to compare a NaN")
                    })
                    .unwrap()
            }) // for each vector containing elements from the same coordinate find the smallest element
            .collect::<V3<f32>>();

        let max: V3<f32> = rotated_and_zipped_points
            .map(|axis| {
                axis.into_iter()
                    .max_by(|x, y| x.partial_cmp(y).expect("Tried to compare a NaN"))
                    .unwrap()
            })
            .collect::<V3<f32>>();

        Box::new(Rotated {
            sin_theta,
            cos_theta,
            axis,
            bounding_box: AABB { min, max },
            hitable: self,
        })
    }
}
