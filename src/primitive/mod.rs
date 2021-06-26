//! Objects shapes.

mod hitbox;
mod mesh;
mod moving_sphere;
mod rectangle;
mod sphere;
mod triangle;

pub use hitbox::HitBox;
pub use mesh::Mesh;
pub use moving_sphere::MovingSphere;
pub use rectangle::XYrect;
pub use rectangle::XZrect;
pub use rectangle::YZrect;
pub use sphere::Sphere;
pub use triangle::Triangle;

use crate::hit::*;
