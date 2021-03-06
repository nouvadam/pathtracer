use crate::hittables::{BvhNode, HittableList};
use crate::material::MaterialContainer;
use crate::misc::Ppm;
use crate::{Camera, RaySetting, V3};

use itertools::*;
use rand::Rng;
use std::time::Instant;

use rayon::prelude::*;

#[derive(Clone, Copy)]
/// Configuration of desired Image to render.
pub struct ImageConfig {
    /// Width of image in pixels.
    pub nx: u32,
    /// Height of image in pixels.
    pub ny: u32,
    /// How many rays should be sent through each pixel, more is better and slower.
    pub samples_per_pixel: u16,
    /// Global setting shared by all rays
    pub ray_setting: RaySetting,
    /// Name of the Scene to use when creating a file.
    pub name: &'static str,
}
/// Represents scene to render.
pub struct Scene {
    /// Camera
    pub camera: Camera,
    /// Scene to render
    pub world: BvhNode,
    /// Lights
    pub lights: Option<HittableList>,
    /// Materials
    pub materials: MaterialContainer,
}
/// Enables rendering into file.
pub trait Render {
    /// Renders scene into Ppm struct, which could later be saved into file.
    ///
    /// `image_config` - Configuration of rendered image.
    fn render(&self, image_config: ImageConfig) -> Ppm;
    /// Renders Scene multiple time, each time with better quality, and saves images into files.
    ///
    /// `image_config` - Starting configuration of rendered image.
    ///
    /// `iterations` - How many times image should be rendered, each consequtive time number of rays increases by the factor of 2.
    fn loop_render(&self, image_config: ImageConfig, iterations: u16);
}

impl Render for Scene {
    fn render(&self, image_config: ImageConfig) -> Ppm {
        let mut output_file = Ppm::new(image_config.ny, image_config.nx);

        // Create vector of pixels; first two u32 values are pixel's x and y coordinate, and V3 is color of the pixel.
        let pixels: Vec<(u32, u32, V3<f32>)> = iproduct!(0..image_config.ny, 0..image_config.nx)
            .par_bridge()
            .map(|pixel| {
                // Map pixel's coords into his color.
                let mut color = V3::zero();
                let mut rng = rand::thread_rng();

                // u and v are coordinates of subpixel, pixel's color is the average of samples
                for _subpixel in 0..image_config.samples_per_pixel {
                    let u = ((pixel.1 as f32) + rng.gen::<f32>()) / (image_config.nx as f32);
                    let v = ((pixel.0 as f32) + rng.gen::<f32>()) / (image_config.ny as f32);

                    color = color
                        + self
                            .camera
                            .get_ray(u, v, &image_config.ray_setting)
                            .get_color(&self);
                }

                (pixel.1, pixel.0, color)
            })
            .collect();

        // Puts vector of pixels into Ppm struct
        for pixel in pixels {
            // Average color of subpixels
            let scale = 1.0 / (image_config.samples_per_pixel as f32);

            let color_without_nan: V3<f32> = pixel
                .2
                .into_iter()
                .map(|color| if color.is_nan() { 0.0 } else { color })
                .collect();

            let mut color = color_without_nan * scale;

            //normalize
            if color.x > 1.0 {
                color.y /= color.x;
                color.z /= color.x;
                color.x = 1.0;
            }
            if color.y > 1.0 {
                color.x /= color.y;
                color.z /= color.y;
                color.y = 1.0;
            }
            if color.z > 1.0 {
                color.x /= color.z;
                color.y /= color.z;
                color.z = 1.0;
            }

            let color = V3::new(
                (color.x.sqrt().clamp(0.0, 0.999) * 254.99) as u8,
                (color.y.sqrt().clamp(0.0, 0.999) * 254.99) as u8,
                (color.z.sqrt().clamp(0.0, 0.999) * 254.99) as u8,
            );

            output_file.set_pixel(pixel.0, pixel.1, color);
        }

        output_file
    }

    fn loop_render(&self, image_config: ImageConfig, iterations: u16) {
        let mut image_config = ImageConfig { ..image_config };

        for _i in 0..iterations {
            let now = Instant::now();

            let image = self.render(image_config);

            image
                .write_file(&format!(
                    "{}_{}.ppm",
                    image_config.name, image_config.samples_per_pixel
                ))
                .expect("YOU FAILED");
            image
                .median_filter(1)
                .write_file(&format!(
                    "{}_{}_median_1.ppm",
                    image_config.name, image_config.samples_per_pixel
                ))
                .expect("YOU FAILED");

            println!(
                "{} milliseconds for {} rays.",
                now.elapsed().as_millis(),
                image_config.samples_per_pixel
            );

            image_config = ImageConfig {
                samples_per_pixel: image_config.samples_per_pixel * 2,
                ..image_config
            };
        }
    }
}
