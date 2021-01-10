//! Image filtering from noise.

use crate::misc::PPM;
use crate::V3;
use itertools::*;

impl PPM {
	/// Filters PPM image from noise, returns filtered image. Skips pixel with zero value - those doesn't contribute to image at all, at the cost of assumption, that all pixels should have non-zero color.
	pub fn filter(&self) -> PPM {
		let mut copied_self = self.clone();
		iproduct!(1..(self.width-1), 1..(self.height-1))
		.for_each(|pixel| {
				let new_color: Vec<V3<u8>> = iproduct!((pixel.0-1)..=(pixel.0+1), (pixel.1-1)..=(pixel.1+1)).map(|offset_pixel| {
					self.get_pixel(offset_pixel.0, offset_pixel.1).expect(&format!("{} {}", offset_pixel.0, offset_pixel.1))
				})
				.filter(|pixel| *pixel != V3::new(0u8,0u8,0u8))
				.collect();

				let sum: V3<u16> = new_color.iter()
				.map(|pixel| V3::new(pixel.x as u16, pixel.y as u16, pixel.z as u16))
				.fold(V3::new(0,0,0), |pixel, acc| acc+pixel);

				if new_color.len() > 0 {
					let mean_color = sum/ (new_color.len() as u16);
					let new_mean_color:V3<u8> = V3::new(mean_color.x as u8, mean_color.y as u8, mean_color.z as u8);
					
					copied_self.set_pixel(pixel.0, pixel.1, new_mean_color);
				}
			}
		);

		copied_self.clone()
	}
}