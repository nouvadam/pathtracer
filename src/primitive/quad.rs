use crate::{
    hittables::Aabb,
    misc::{Interval, Pdf},
    Hit, Hittable, Ray, V3,
};

/// Quad primitive
#[derive(Clone)]
pub struct Quad {
    Q: V3<f32>,
    u: V3<f32>,
    v: V3<f32>,
    normal: V3<f32>,
    D: f32,
    w: V3<f32>,
    /// Material of this Quad
    material: usize,
    bounding_box: Aabb,
}

impl Quad {
    /// Creates new Quad primitive.
    ///
    /// `Q` - Point representing the starting corner
    /// `u` - Vector representing first side of the quad.
    //  `v` - Vector representing second side of the quad.
    //  `material` - Material used.
    pub fn new(Q: V3<f32>, u: V3<f32>, v: V3<f32>, material: usize) -> Self {
        let n = u.cross(v);
        let normal = n.norm();

        Quad {
            Q,
            u,
            v,
            material,
            bounding_box: Aabb::new(Q, Q + u + v).surrounding_box(Aabb::new(Q + u, Q + v)),
            normal,
            D: normal.dot(Q),
            w: n / n.dot(n),
        }
    }
}

impl Hittable for Quad {
    #[doc = r" Returns Hit structure, if Ray intersects with this object surface in passed time interval."]
    #[doc = r""]
    #[doc = r" `r` - Ray that should hit the object."]
    fn hit(&self, ray: &Ray) -> Option<Hit> {
        let denominator = self.normal.dot(ray.end);

        // Parallel to the plane
        if denominator.abs() < 1e-8 {
            return None;
        }

        let t = (self.D - self.normal.dot(ray.origin)) / denominator;
        if !ray.setting.ray_time.contains(t) {
            return None;
        }

        // getting point of intersection at the plane containing this quad
        let intersection = ray.point_at_param(t);

        // Determine if the hit point lies within the planar shape using its plane coordinates.
        let relative_intersection = intersection - self.Q;
        let alpha = self.w.dot(relative_intersection.cross(self.v));
        let beta = self.w.dot(self.u.cross(relative_intersection));

        if !is_interior(alpha, beta) {
            return None;
        }

        Some(Hit::new(
            ray,
            self.normal,
            t,
            intersection,
            self.material,
            alpha,
            beta,
        ))
    }

    #[doc = r" Returns bounding box of the object"]
    fn bounding_box(&self) -> Aabb {
        self.bounding_box.clone()
    }
}

fn is_interior(alpha: f32, beta: f32) -> bool {
    let interval = Interval::new(0.0, 1.0);
    interval.contains(alpha) && interval.contains(beta)
}

impl Pdf for Quad {
    fn value(&self, _origin: V3<f32>, _direction: V3<f32>) -> f32 {
        todo!()
    }

    fn generate(&self, _origin: V3<f32>) -> V3<f32> {
        todo!()
    }
}
