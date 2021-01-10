//! Simple unbiased pathtracer

#![warn(missing_docs)]

pub use camera::*;
pub use hit::*;
pub use ray::*;
pub use render::*;
pub use vec3::*;

mod camera;
mod hit;
mod ray;
mod render;
mod vec3;

pub mod hitables;
pub mod material;
pub mod misc;
pub mod primitive;
pub mod texture;
pub mod transform;
