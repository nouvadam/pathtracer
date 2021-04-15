use crate::texture::Texture;
use crate::V3;
/// Old school plasma texture.
#[derive(Clone)]
pub struct PlasmaTexture {
    /// Different values makes different patterns.
    pub param: f32,
    /// How big the pattern is.
    pub scale: f32,
}

impl Texture for PlasmaTexture {
    fn value(&self, u: f32, v: f32, _p: V3<f32>) -> V3<f32> {
        let u = (u - 0.5) * self.scale;
        let v = (v - 0.5) * self.scale;

        let value1 = (u + self.param).sin();
        let value2 = ((v + self.param) * 0.5).sin();
        let value3 = ((u + v + self.param) * 0.5).sin();

        let cx = u + (self.param / 5.0).sin() * 0.5;
        let cy = v + (self.param / 3.0).cos() * 0.5;

        let value4 = ((100.0 * (cx * cx + cy * cy) + 1.0).sqrt() + self.param).sin();

        let added_value = value1 + value2 + value3 + value4;
        let pi = std::f32::consts::PI;

        V3::new(
            (added_value * pi).sin(),
            (added_value * pi + 2.0 * pi / 3.0).sin(),
            (added_value * pi + 4.0 * pi / 3.0).sin(),
        ) + V3::<f32>::new(1.0, 1.0, 1.0) / 2.0
    }
}
