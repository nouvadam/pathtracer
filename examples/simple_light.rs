use pathtracer::hitables::*;
use pathtracer::material::*;
use pathtracer::misc::*;
use pathtracer::primitive::*;
use pathtracer::texture::*;
use pathtracer::*;
fn main() {
    let mut hitable = HitableList::new();

    let perlin_texture = PerlinNoiseTexture {
        perlin_noise: Perlin::new(),
        scale: 5.0,
    };

    hitable.add(Box::new(Sphere {
        center: V3::new(0.0, -1000.0, 0.0),
        radius: 1000.0,
        material: Box::new(Lambertian {
            albedo: Box::new(perlin_texture),
        }),
    }));

    let perlin_texture = PerlinNoiseTexture {
        perlin_noise: Perlin::new(),
        scale: 5.0,
    };

    hitable.add(Box::new(Sphere {
        center: V3::new(0.0, 2.0, 0.0),
        radius: 2.0,
        material: Box::new(Lambertian {
            albedo: Box::new(perlin_texture),
        }),
    }));

    hitable.add(Box::new(XYrect {
        x0: 3.0,
        x1: 5.0,
        y0: 1.0,
        y1: 3.0,
        k: -2.0,
        material: Box::new(LightSource {
            albedo: Box::new(ConstantTexture {
                color: V3::new(4.0, 4.0, 4.0),
            }),
        }),
    }));

    let image_config = ImageConfig {
        nx: 2048,
        ny: 1024,
        samples_per_pixel: 2,
        ray_setting: RaySetting {
            background_color: V3::new(0.0, 0.0, 0.0),
            depth: 4,
        },
        name: "simple_light",
    };

    Scene {
        camera: Camera::new(
            V3::new(26.0, 3.0, 6.0), //lookfrom
            V3::new(0.0, 2.0, 0.0),  //lookat
            V3::new(0.0, 1.0, 0.0),  //vup
            20.0,                    //vertical_fov
            2.0,                     //aspect
            0.05,                    //aperture
            26.0,                    //focus_dist
            0.0,                     //time0
            1.0,                     //time1
        ),
        world: Box::new(hitable),
    }
    .loop_render(image_config, 12);
}
