use crate::hit::*;
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
    /// `hitable` - Struct implementing Hitable trait from which color should be taken; usually this is a Scene.
    pub fn get_color(&self, hitable: &dyn Hitable) -> V3<f32> {
        self.color(hitable, 0)
    }

    /// Recursively bounce ray between objects in scene, at each hit multiply current color of the ray with color of the object, or hit point on the object.
    fn color(&self, hitable: &dyn Hitable, depth: u16) -> V3<f32> {
        // Does the intersection occur at all?
        match hitable.hit(self, 0.001, 2048.0) {
            // If ray hit some object, then we bounce that Ray from the object with updated color.
            Some(hit) => match hit.material.scatter(&self, &hit) {
                // Scatter ray from a hit point

                // Ray has been scattered
                Some((ray, albedo, light)) => {
                    // Checks if ray had bounced too many times.
                    if depth < self.setting.depth {
                        // Checks if the material that Ray has hit emits light, if yes, then returns the color of this light without bouncing ray futher.
                        if light {
                            albedo
                        } else {
                            //
                            albedo.mul(ray.color(hitable, depth + 1))
                        }
                    } else {
                        V3::new(0.0, 0.0, 0.0)
                    }
                }
                // If scatter hasn't produced Ray, at example in case of absorbing the Ray, then the resulting color of the Ray is black.
                None => V3::new(0.0, 0.0, 0.0),
            },
            // No intersections occured, thus Ray came from the background.
            None => self.setting.background_color,
        }
    }
}
