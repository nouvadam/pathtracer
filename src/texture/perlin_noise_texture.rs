use crate::V3;
use crate::misc::Perlin;
use crate::texture::Texture;
/// Perlin noise texture.
#[derive(Clone)]
pub struct PerlinNoiseTexture {
    /// Some sort of Perlin noise source.
    pub perlin_noise: Perlin,
    /// How big the pattern is.
    pub scale: f32,
}

impl Texture for PerlinNoiseTexture {
    fn value(&self, _u: f32, _vv: f32, p: V3<f32>) -> V3<f32> {
        V3::new(1.0, 1.0, 1.0)
            * 0.5_f32
            * (1.0 + (10.0 * self.perlin_noise.turbulence(p, 7) + self.scale * p.z).sin())
    }
}
