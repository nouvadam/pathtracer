use raytracer::*;
use raytracer::material::*;
use raytracer::primitive::*;
use raytracer::texture::*;
use raytracer::transform::*;
use raytracer::hitables::*;

use rand::Rng;

fn main() {
    let mut hitable = HitableList::new();

    hitable.add(Box::new(Sphere {
        center: V3::new(0.0, -100.5, -2.0),
        radius: 100.0,
        material: Box::new(Lambertian {
            albedo: Box::new(ConstantTexture {
                color: V3::new(0.5, 0.5, 0.5),
            }),
        }),
    }));

    let scale = 2.0;
    let mut seed = rand::thread_rng();

    hitable.add(Box::new(Sphere {
        center: V3::new(-1.0, 0.0, -2.0),
        radius: 0.5,
        material: Box::new(LightSource {
            albedo: Box::new(PlasmaTexture {
                param: seed.gen::<f32>() * 100.0,
                scale,
            }),
        }),
    }));

    hitable.add(
        Box::new(XYrect {
            x0: -0.5,
            x1: 0.5,
            y0: -0.5,
            y1: 0.5,
            k: 0.0,
            material: Box::new(LightSource {
                albedo: Box::new(PlasmaTexture {
                    param: seed.gen::<f32>() * 100.0,
                    scale: 1.0,
                }),
            }),
        })
        .rotate(V3::new(0.0, 1.0, 0.0), -0.8)
        .translate(V3::new(0.5, 0.0, -2.0)),
    );

    let image_config = ImageConfig {
        nx: 2048,
        ny: 1024,
        samples_per_pixel: 2,
        ray_setting: RaySetting {
            background_color: V3::new(0.0, 0.0, 0.0),
            depth: 32,
        },
        name: r#"plasma_test"#
    };

    Scene {
        
        camera: Camera::new(
            V3::new(0.0, 0.0, 0.0),  //lookfrom
            V3::new(0.0, 0.0, -1.0), //lookat
            V3::new(0.0, 1.0, 0.0),  //vup
            45.0,                    //vertical_fov
            2.0,                     //aspect
            0.0,                     //aperture
            2.0,                     //focus_dist
            0.0,                     //time0
            1.0,                     //time1
        ),
        world: Box::new(hitable)
    }
    .loop_render(image_config, 12);
}
