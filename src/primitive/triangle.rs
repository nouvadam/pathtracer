use crate::hit::*;
use crate::hitables::AABB;
use crate::material::Material;
use crate::ray::*;
use crate::V3;
/// Triangle primitive.
#[derive(Clone)]
pub struct Triangle {
    /// Verticles of a triangle, CCW
    verticles: [V3<f32>; 3],
    /// Normals of corresponding verticles
    normals: Option<[V3<f32>; 3]>,
    /// Material of verticle
    material: Box<dyn Material + Sync + Send>,
    bounding_box: AABB,
}

impl Hitable for Triangle {
    fn hit(&self, r: &Ray, t_min: f32, t_max: f32) -> Option<Hit> {
        let v0v1 = self.verticles[1] - self.verticles[0];
        let v0v2 = self.verticles[2] - self.verticles[0];

        let pvec = r.end.cross(v0v2);
        let det = v0v1.dot(pvec);
        let inv_det = 1.0 / det;
        let epsilon = 0.0000001;

        if det.abs() < epsilon {
            return None;
        }

        let tvec = r.origin - self.verticles[0];
        let u = tvec.dot(pvec) * inv_det;

        if u < 0.0 || u > 1.0 {
            return None;
        }

        let qvec = tvec.cross(v0v1);
        let v = r.end.dot(qvec) * inv_det;

        if v < 0.0 || u + v > 1.0 {
            return None;
        }

        let t = v0v2.dot(qvec) * inv_det;

        if t < t_min || t > t_max {
            return None;
        }

        let normal_to_triangle = match self.normals {
            Some(normals) => normals[0] * (1.0 - u - v) + normals[1] * u + normals[2] * v,
            None => (v0v2).cross(v0v1).norm(),
        };

        Some(Hit::new(
            r,
            normal_to_triangle,
            t,
            r.point_at_param(t),
            &*self.material,
            u,
            v,
        ))
    }

    fn bounding_box(&self) -> AABB {
        self.bounding_box.clone()
    }
}

impl<'material> Triangle {
    /// Returns new Triangle
    ///
    /// `verticles` - Verticles of created Triangle, CCW
    ///
    /// `normals` - Normals of corresponding verticles.
    ///
    /// `material` - Material of created Triangle.
    pub fn new(
        verticles: V3<V3<f32>>,
        normals: Option<V3<V3<f32>>>,
        material: Box<dyn Material + Sync + Send>,
    ) -> Triangle {
        Triangle {
            verticles: [verticles.x, verticles.y, verticles.z],
            normals: match normals {
                Some(normals) => Some([normals.x, normals.y, normals.z]),
                None => None,
            },
            material,
            bounding_box: Self::init_bounding_box(&verticles),
        }
    }

    fn init_bounding_box(verticles: &V3<V3<f32>>) -> AABB {
        use crate::misc::IntoMultizip;

        let rotated_and_zipped_points = verticles
            .into_iter()
            .map(|element| element.into_iter()) // casting V3 tuple to 3-element vector
            .multizip(); // creating 3 vectors which elements are from the same coordinate

        let min: V3<f32> = rotated_and_zipped_points
            .clone()
            .map(|axis| {
                axis.into_iter()
                    .min_by(|x, y| x.partial_cmp(y).expect("Tried to compare a NaN"))
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

        AABB { min, max }
    }
}
