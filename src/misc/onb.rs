//! Orthonormal basis

use crate::V3;

#[derive(Clone)]

/// Onb represents ortonormal basis.
pub struct Onb {
    axis: V3<V3<f32>>,
}

impl Onb {
    /// Returns `u` component of orthonormal basis.
    pub fn u(&self) -> V3<f32> {
        self.axis.x
    }

    /// Returns `v` component of orthonormal basis.
    pub fn v(&self) -> V3<f32> {
        self.axis.y
    }

    /// Returns `w` component of orthonormal basis.
    pub fn w(&self) -> V3<f32> {
        self.axis.z
    }

    /// Builds orthonormal basis from `w` vector.
    pub fn build_from_w(n: &V3<f32>) -> Self {
        let w = n.norm();

        let a = if w.x.abs() > 0.9 {
            V3::new(0.0, 1.0, 0.0)
        } else {
            V3::new(1.0, 0.0, 0.0)
        };

        let v = (w.cross(a)).norm();

        let u = w.cross(v);

        Onb {
            axis: V3::new(u, v, w),
        }
    }

    /// Returns point from this orthonormal basis in standard basis.
    pub fn local(&self, a: f32, b: f32, c: f32) -> V3<f32> {
        self.u() * a + self.v() * b + self.w() * c
    }

    /// Returns point from this orthonormal basis in standard basis.
    pub fn local_from_vec(&self, a: &V3<f32>) -> V3<f32> {
        self.u() * a.x + self.v() * a.y + self.w() * a.z
    }
}
