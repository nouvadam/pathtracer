use crate::V3;
use crate::texture::Texture;
/// Texture with constant color.
#[derive(Clone)]
pub struct ConstantTexture {
    /// Color of the texture.
    pub color: V3<f32>,
}

impl Texture for ConstantTexture {
    fn value(&self, _u: f32, _v: f32, _p: V3<f32>) -> V3<f32> {
        self.color
    }
}
