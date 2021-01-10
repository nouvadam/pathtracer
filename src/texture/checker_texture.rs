use crate::V3;
use crate::texture::Texture;
/// Checker texture.
#[derive(Clone)]
pub struct CheckerTexture {
    /// Texture on the odd positions of the texture.
    pub odd: Box<dyn Texture>,
    /// Texture on the even positions of the texture.
    pub even: Box<dyn Texture>,
}

impl Texture for CheckerTexture {
    fn value(&self, u: f32, v: f32, p: V3<f32>) -> V3<f32> {
        let sines = (10.0 * p.x).sin() * (10.0 * p.y).sin() * (10.0 * p.z).sin();
        if sines < 0.0 {
            self.odd.value(u, v, p)
        } else {
            self.even.value(u, v, p)
        }
    }
}
