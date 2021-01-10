use crate::V3;

use std::fs::File;
use std::io::Write;
use std::path::Path;
/// PPM file format.
#[derive(Clone)]
pub struct PPM {
    /// Height of the image.
    pub height: u32,
    /// Width of the image.
    pub width: u32,
    data: Vec<u8>,
}

impl PPM {
    /// Returns new PPM object.
    pub fn new(height: u32, width: u32) -> PPM {
        let size = 3 * height * width;
        let data = vec![0; size as usize];
        PPM {
            height,
            width,
            data,
        }
    }

    fn buffer_size(&self) -> u32 {
        3 * self.height * self.width
    }

    fn get_offset(&self, x: u32, y: u32) -> Option<usize> {
        let offset = (((self.height-1)- y) * self.width * 3) + (x * 3);
        if offset < self.buffer_size() {
            Some(offset as usize)
        } else {
            None
        }
    }
    /// Return color of the pixel at given position.
    #[allow(dead_code)]
    pub fn get_pixel(&self, x: u32, y: u32) -> Option<V3<u8>> {
        match self.get_offset(x, y) {
            Some(offset) => {
                let red = self.data[offset];
                let green = self.data[offset + 1];
                let blue = self.data[offset + 2];
                Some(V3::new(red, green, blue))
            }
            None => None,
        }
    }
    /// Sets color of the pixel at given position.
    pub fn set_pixel(&mut self, x: u32, y: u32, color: V3<u8>) -> bool {
        match self.get_offset(x, y) {
            Some(offset) => {
                self.data[offset] = color.x;
                self.data[offset + 1] = color.y;
                self.data[offset + 2] = color.z;
                true
            }
            None => false,
        }
    }
    /// Returns all pixels in vector.
    pub fn get_rgba(&mut self) -> Vec<u8> {
        let result = Vec::with_capacity(self.data.len() * 2);

        self.data
            .clone()
            .into_iter()
            .zip(1..self.data.len() + 1)
            .fold(result, |mut acc, n| {
                acc.push(n.0);
                if n.1 % 3 == 0 {
                    acc.push(255);
                }
                acc
            })
    }

    /// Write PPM into file with given `filename`.
    pub fn write_file(&self, filename: &str) -> std::io::Result<()> {
        let path = Path::new(filename);
        let mut file = File::create(&path)?;
        let header = format!("P6 {} {} 255\n", self.width, self.height);
        file.write_all(header.as_bytes())?;
        file.write_all(&self.data)?;
        Ok(())
    }
}