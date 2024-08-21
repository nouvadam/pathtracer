use crate::hit::*;
use crate::misc::{Interval, MixturePdf, Pdf};
use crate::Scene;
use crate::V3;

/// Ray in form of segment of the straight line.
#[derive(Copy, Clone)]
pub struct Ray<'setting> {
    /// Origin point of the Ray.
    pub origin: V3<f32>,
    /// End point of the Ray.
    pub end: V3<f32>,
    /// Timestamp during which Ray lives.
    pub time: f32,
    /// Global setting shared by all Rays during rendering.
    pub setting: &'setting RaySetting,
}

/// Global setting of all rays.
#[derive(Copy, Clone)]
pub struct RaySetting {
    /// Color of the current Scene background.
    pub background_color: V3<f32>,
    /// How many times rays should bounce between objects before returning color.
    pub depth: u16,
    /// Time interval in which hit should occur for this ray.
    pub ray_time: Interval,
}

impl Ray<'_> {
    /// Returns point lying on the Ray at the specific time.
    ///
    /// `t` - Time at which point should be taken.
    pub fn point_at_param(&self, t: f32) -> V3<f32> {
        self.origin + self.end * t
    }

    /// Get the color of casted Ray.
    ///
    /// `hittable` - Struct implementing hittable trait from which color should be taken; usually this is a Scene.
    pub fn get_color(&self, scene: &Scene) -> V3<f32> {
        self.color(scene, 0)
    }

    /// Recursively bounce ray between objects in scene, at each hit multiply current color of the ray with color of the object, or hit point on the object.
    fn color(&self, scene: &Scene, depth: u16) -> V3<f32> {
        // Does the intersection occur at all?
        match scene.world.hit(self) {
            // If ray hit some object, then we bounce that Ray from the object with updated color.
            Some(hit) => {
                let material = scene.materials.get(hit.material);
                match material.scatter(self, &hit) {
                    // Scatter ray from a hit point

                    // Ray has been scattered
                    Some(scatter_record) => {
                        // Checks if ray had bounced too many times.
                        if depth < self.setting.depth {
                            let generated_dir;
                            let pdf_val: f32;

                            // If there are lights on the scene, then sample rays to those lights and with accordance to material, else sample just from the material.
                            match &scene.lights {
                                Some(lights) => {
                                    let mixture_pdf = MixturePdf::new(lights, &*scatter_record.pdf);
                                    generated_dir = mixture_pdf.generate(hit.point);
                                    pdf_val = mixture_pdf.value(hit.point, generated_dir);
                                }
                                None => {
                                    generated_dir = scatter_record.pdf.generate(hit.point);
                                    pdf_val = scatter_record.pdf.value(hit.point, generated_dir);
                                }
                            }

                            let scattered_ray = Ray {
                                origin: hit.point,
                                end: generated_dir,
                                time: self.time,
                                setting: self.setting,
                            };

                            match scatter_record.specular_ray {
                                Some(ray) => {
                                    // Returns specular ray.
                                    scatter_record
                                        .attenuation
                                        .hadamard(ray.color(scene, depth + 1))
                                }
                                None => {
                                    // Returns scattered ray.
                                    material.color_emitted(self, &hit)
                                        + ((scatter_record.attenuation
                                            * material.scattering_pdf(self, &hit, &scattered_ray))
                                        .hadamard(scattered_ray.color(scene, depth + 1))
                                            / pdf_val)
                                }
                            }
                        } else {
                            // If the ray bounce limit is exceeded, no more light is gathered.
                            V3::new(0.0, 0.0, 0.0)
                        }
                    }
                    // If scatter hasn't produced Ray, at example in case of absorbing the Ray, then the resulting color of the Ray is emitted light by the object.
                    None => material.color_emitted(self, &hit),
                }
            }
            // No intersections occured, thus Ray came from the background.
            None => self.setting.background_color,
        }
    }
}
