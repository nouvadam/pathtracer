use pathtracer::hitables::*;
use pathtracer::material::*;
use pathtracer::primitive::*;
use pathtracer::texture::*;
use pathtracer::*;
use rand::Rng;
fn main() {
    let mut hitable = HitableList::new();

    hitable.add(Box::new(Sphere {
        center: V3::new(0.0, 0.0, 0.0),
        radius: 2.0,
        material: Box::new(Lambertian {
            albedo: Box::new(ImageTexture::new("assets/earthmap.png")),
        }),
    }));
    let mut seed = rand::thread_rng();

    hitable.add(Box::new(Sphere {
        center: V3::new(15.0, 0.0, 60.0),
        radius: 20.0,
        material: Box::new(LightSource {
            albedo: Box::new(ConstantTexture {
                color: V3::new(1.0, 1.0, 1.0).norm() * (15.0 + 15.0 * seed.gen::<f32>()),
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
        name: "earth",
    };

    Scene {
        camera: Camera::new(
            V3::new(13.0, 2.0, 3.0), //lookfrom
            V3::new(0.0, 0.0, 0.0),  //lookat
            V3::new(0.0, 1.0, 0.0),  //vup
            20.0,                    //vertical_fov
            2.0,                     //aspect
            0.05,                    //aperture
            10.0,                    //focus_dist
            0.0,                     //time0
            1.0,                     //time1
        ),
        world: Box::new(hitable),
    }
    .loop_render(image_config, 12);
}
