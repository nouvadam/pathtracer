//! Image filtering from noise.

use crate::misc::algs::*;
use crate::misc::PPM;
use crate::misc::multizip::*;
use crate::V3;
use itertools::*;

impl PPM {
    /// Filters PPM image from noise, by calculating mean of neighbour pixels, returns filtered image. Skips pixel with zero value - those doesn't contribute to image at all, at the cost of assumption, that all pixels should have non-zero color.
    pub fn box_filter(&self) -> PPM {
        let mut copied_self = self.clone();
        iproduct!(1..(self.width - 1), 1..(self.height - 1)).for_each(|pixel| {
            let new_color: Vec<V3<u8>> =
                iproduct!((pixel.0 - 1)..=(pixel.0 + 1), (pixel.1 - 1)..=(pixel.1 + 1))
                    .map(|offset_pixel| {
                        self.get_pixel(offset_pixel.0, offset_pixel.1)
                            .expect(&format!("{} {}", offset_pixel.0, offset_pixel.1))
                    })
                    .filter(|pixel| *pixel != V3::new(0u8, 0u8, 0u8))
                    .collect();

            let sum: V3<u16> = new_color
                .iter()
                .map(|pixel| V3::new(pixel.x as u16, pixel.y as u16, pixel.z as u16))
                .fold(V3::new(0, 0, 0), |pixel, acc| acc + pixel);

            if new_color.len() > 0 {
                let mean_color = sum / (new_color.len() as u16);
                let new_mean_color: V3<u8> =
                    V3::new(mean_color.x as u8, mean_color.y as u8, mean_color.z as u8);

                copied_self.set_pixel(pixel.0, pixel.1, new_mean_color);
            }
        });

        copied_self.clone()
    }

    /// Filters PPM image from noise, by returning median value of neighbour pixels. `size` - size of window from the center, ie. size of 1 would mean 3x3 window, size 2 is 5x5 etc.
    pub fn median_filter(&self, size: u32) -> PPM {
        let mut copied_self = self.clone();
		
		let mid = (size*2+1).pow(2)/2;
		
        // For each pixel
        iproduct!(size..(self.width - size), size..(self.height - size)).for_each(|pixel| {
            // Create matrix 3x3 with this pixel in the center
            let mut matrix: Vec<V3<u8>> =
                iproduct!((pixel.0 - size)..=(pixel.0 + size), (pixel.1 - size)..=(pixel.1 + size))
                    .map(|offset_pixel| {
                        self.get_pixel(offset_pixel.0, offset_pixel.1)
                            .expect(&format!("{} {}", offset_pixel.0, offset_pixel.1))
                    })
                    .collect::<Vec<V3<u8>>>();

            // Find median for each channel

            let medians: Vec<u8> = matrix
                .iter()
                .map(|pixel| pixel.into_iter())
                .multizip()
                .map(|vec| quickselect(&mut vec.clone(), mid as usize))
				.collect();

            copied_self.set_pixel(pixel.0, pixel.1, V3::new(medians[0],medians[1],medians[2]));
        });

        copied_self.clone()
    }
}


#[cfg(test)]
mod tests {
    use super::*;
    use rand::*;

    #[test]
    fn median_filter_test() {
		let before_filter = vec![1,0,0, 1,4,0, 1,1,0, 0,7,0, 0,3,255, 1,5,0, 0,2,0, 0,8,0, 1,6,0];
		let correct_filter = vec![1,0,0, 1,4,0, 1,1,0, 0,7,0, 1,4,0, 1,5,0, 0,2,0, 0,8,0, 1,6,0];
		let ppm_after_filter = PPM::new_from_vec(3,3,before_filter.clone()).unwrap().median_filter(1);
		let ppm_correct_filter = PPM::new_from_vec(3,3,correct_filter).unwrap();
		
        assert_eq!(ppm_after_filter, ppm_correct_filter);
    }
}
