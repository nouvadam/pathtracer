//! Defines color on the surface of objects.

mod checker_texture;
mod constant_texture;
mod image_texture;
mod perlin_noise_texture;
mod plasma_texture;

pub use checker_texture::CheckerTexture;
pub use constant_texture::ConstantTexture;
pub use image_texture::ImageTexture;
pub use perlin_noise_texture::PerlinNoiseTexture;
pub use plasma_texture::PlasmaTexture;

use crate::V3;
use objekt_clonable::*;
/// Objects implementing this trait behaves like texture.
#[clonable]
pub trait Texture: Send + Sync + Clone + TextureClone {
    /// Returns color of the Texture at the UV position.
    ///
    /// `u`, `v` - Position on the texture.
    ///
    /// `p` - Position in the world of the point on the texture.
    fn value(&self, u: f32, v: f32, p: V3<f32>) -> V3<f32>;
}
/// Needed to clone trait object, which are implementations of Texture trait, into threads for parralelization.
pub trait TextureClone {
    /// Needed to clone trait object, which are implementations of Texture trait, into threads for parralelization.
    fn box_clone(&self) -> Box<dyn Texture + Send + Sync>;
}

impl<T> TextureClone for T
where
    T: 'static + Texture + Clone,
{
    fn box_clone(&self) -> Box<dyn Texture + Send + Sync> {
        Box::new((*self).clone())
    }
}

impl Clone for Box<dyn Texture + Send + Sync> {
    fn clone(&self) -> Box<dyn Texture + Send + Sync> {
        self.box_clone()
    }
}
