use crate::hittables::Aabb;
use crate::{Ray, V3};

use objekt_clonable::*;
/// Hit event, created when ray hits some object.

//#[derive(Default)]
pub struct Hit {
    /// Time of collision.
    pub t: f32,
    /// Point on the object where hit occured.
    pub point: V3<f32>,
    /// Normal vector to the surface where hit occured.
    pub normal: V3<f32>,
    /// Object hit material index.
    pub material: usize,
    /// Position on the texture.
    pub u: f32,
    /// Position on the texture.
    pub v: f32,
    /// True if ray hit the object from the exterior.
    pub front_face: bool,
}

impl Hit {
    /// Method to create new hit struct, it imposes correct normal vector
    ///
    /// `r` - Ray that hit the object.
    ///
    /// `outward_normal` - Vector normal to the surface of hitted object at collision point.
    ///
    /// `t` - Time of collision.
    ///
    /// `point` - Position of collision.
    ///
    /// `material` - Material of hitted object.
    ///
    /// `u`, `v` - Position on the texture.
    pub fn new(
        r: &Ray,
        outward_normal: V3<f32>,
        t: f32,
        point: V3<f32>,
        material: usize,
        u: f32,
        v: f32,
    ) -> Hit {
        let front_face = r.end.dot(outward_normal) < 0.0;
        let normal = if front_face {
            outward_normal
        } else {
            -outward_normal
        };

        Hit {
            t,
            point,
            normal,
            material,
            u,
            v,
            front_face,
        }
    }

    /// Returns hit event with front face atribute flipped, making hit event from interior to exterior and vice versa.
    pub fn flip_front_face(mut self) -> Self {
        self.front_face = !self.front_face;
        self
    }
}

use enum_dispatch::enum_dispatch;
#[clonable]
#[enum_dispatch(Primitive)]

/// Hittables are able to be hit by Rays.
pub trait Hittable: Send + Sync + Clone {
    /// Returns Hit structure, if Ray intersects with this object surface in passed time interval.
    ///
    /// `r` - Ray that should hit the object.
    ///
    /// `t_min`, `t_max` - Time interval in which hit should occur.
    fn hit(&self, r: &Ray, t_min: f32, t_max: f32) -> Option<Hit>;
    /// Returns bounding box of the object
    fn bounding_box(&self) -> Aabb;
}

use crate::hittables::*;
/// Objects implementing this trait can be hit by rays.
use crate::primitive::*;
use crate::transform::*;

#[enum_dispatch]
#[derive(Clone)]

/// Primitive implements Hittable trait, thus could be hit by ray.
pub enum Primitive {
    HitBox,
    Mesh,
    MovingSphere,
    XYrect,
    XZrect,
    YZrect,
    Sphere,
    Triangle,
    BvhNode,
    HittableList,
    FlipFace,
    Rotated,
    Translated,
    ConstantMedium,
}
