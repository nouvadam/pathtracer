use crate::{V3, Ray, RaySetting};
use rand::Rng;
/// Lens-based camera.
pub struct Camera {
    lower_left_corner: V3<f32>,
    horizontal: V3<f32>,
    vertical: V3<f32>,
    origin: V3<f32>,
    u: V3<f32>,
    v: V3<f32>,
    lens_radius: f32,
    time_begin: f32,
    time_end: f32,
}

impl Camera {
    /// Creates new camera.
    ///
    /// `lookfrom` - Point from which Camera operates.
    ///
    /// `lookat` - Point at which Camera looks.
    ///
    /// `vup` - Vertical vector of the Camera.
    ///
    /// `vertical_fov` - Vertical field of view, or how wide Camera should look.
    ///
    /// `aspect` - Ratio between desired width of the rendered image and height.
    ///
    /// `aperture` - Width of Camera's lens, more wide lens equals more blurry image.
    ///
    /// `focus_dist` - Distance that lens should focus to.
    ///
    /// `time_begin` - Time at which lens should be open.
    ///
    /// `time_end` - Time at which lens should be closed.

    pub fn new(
        lookfrom: V3<f32>,
        lookat: V3<f32>,
        vup: V3<f32>,
        vertical_fov: f32,
        aspect: f32,
        aperture: f32,
        focus_dist: f32,
        time_begin: f32,
        time_end: f32,
    ) -> Camera {
        let w = (lookfrom - lookat).norm();
        let u = vup.cross(w).norm();
        let v = w.cross(u);

        let theta = vertical_fov * std::f32::consts::PI / 180.0;
        let half_height = (theta / 2.0).tan();
        let half_width = aspect * half_height;

        let lens_radius = aperture / 2.0;

        Camera {
            lower_left_corner: lookfrom
                - u * half_width * focus_dist
                - v * half_height * focus_dist
                - w * focus_dist,
            horizontal: u * half_width * focus_dist * 2.0,
            vertical: v * half_height * focus_dist * 2.0,
            origin: lookfrom,
            u,
            v,
            lens_radius,
            time_begin,
            time_end,
        }
    }
    /// Get Ray from Camera origin through wanted position on the final image.
    ///
    /// `U`, `V` - Coords of pixel, should be in range <0,1>.
    ///
    /// `setting` - Global ray setting.
    pub fn get_ray<'a>(&self, u: f32, v: f32, setting: &'a RaySetting) -> Ray<'a> {
        let rd = V3::get_point_in_sphere() * self.lens_radius;

        let offset = self.u * rd.x + self.v * rd.y;
        let end =
            self.lower_left_corner + self.horizontal * u + self.vertical * v - self.origin - offset;

        Ray {
            origin: self.origin + offset,
            end,
            time: rand::thread_rng().gen_range(self.time_begin, self.time_end),
            setting,
        }
    }
}
