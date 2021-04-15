use crate::texture::Texture;
use crate::V3;
/// Texture that represents some sort of rasterized image.
#[derive(Clone)]
pub struct ImageTexture {
    texture: Vec<u8>,
    bytes_per_pixel: u32,
    width: u32,
    height: u32,
}

impl ImageTexture {
    /// Creates new image texture.
    ///
    /// `file` - Path to image file from which Texture should be made.
    pub fn new(file: &str) -> ImageTexture {
        //reading texture
        use std::fs::File;
        let decoder = png::Decoder::new(File::open(file).unwrap());
        let (info, mut reader) = decoder.read_info().unwrap();
        // Allocate the output buffer.
        let mut texture = vec![0; info.buffer_size()];
        // Read the next frame. Currently this function should only called once.
        // The default options
        reader.next_frame(&mut texture).unwrap();

        ImageTexture {
            texture: texture,
            bytes_per_pixel: 4,
            width: info.width,
            height: info.height,
        }
    }

    fn clamp<T: PartialOrd>(num: T, x: T, y: T) -> T {
        if num < x {
            return x;
        }
        if num > y {
            return y;
        }
        num
    }
}

impl Texture for ImageTexture {
    fn value(&self, ui: f32, vi: f32, _pp: V3<f32>) -> V3<f32> {
        let u = ((ImageTexture::clamp(ui, 0.0, 1.0)) * (self.width as f32)) as u32;
        let v = ((1.0 - ImageTexture::clamp(vi, 0.0, 1.0)) * (self.height as f32)) as u32;

        let u = ImageTexture::clamp(u, 0, self.width - 1);
        let v = ImageTexture::clamp(v, 0, self.height - 1);

        let color_scale = 1.0 / 255.0;

        let pixel = (v * self.width * self.bytes_per_pixel + u * self.bytes_per_pixel) as usize;

        V3::new(
            (f32::from(self.texture[pixel])) * color_scale,
            (f32::from(self.texture[pixel + 1])) * color_scale,
            (f32::from(self.texture[pixel + 2])) * color_scale,
        )
    }
}
