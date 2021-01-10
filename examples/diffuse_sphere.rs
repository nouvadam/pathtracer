use raytracer::*;
use raytracer::material::*;
use raytracer::primitive::*;
use raytracer::texture::*;
use raytracer::hitables::*;

fn main() {
    let mut hitable = HitableList::new();

    hitable.add(Box::new(Sphere {
        center: V3::new(0.0, 0.0, -2.0),
        radius: 0.5,
        material: Box::new(Lambertian {
            albedo: Box::new(ConstantTexture {
                color: V3::new(0.5, 0.5, 0.5),
            }),
        }),
    }));

    hitable.add(Box::new(Sphere {
        center: V3::new(0.0, -100.5, -2.0),
        radius: 100.0,
        material: Box::new(Lambertian {
            albedo: Box::new(ConstantTexture {
                color: V3::new(0.5, 0.5, 0.5),
            }),
        }),
    }));

    let image_config = ImageConfig {
        nx: 2048,
        ny: 1024,
        samples_per_pixel: 2,
        ray_setting: RaySetting {
            background_color: V3::new(0.5, 0.7, 1.0),
            depth: 32,
        },
        name: r#"diffuse_sphere"#
    };

    Scene {
        
        camera: Camera::new(
            V3::new(0.0, 0.0, 0.0),  //lookfrom
            V3::new(0.0, 0.0, -1.0), //lookat
            V3::new(0.0, 1.0, 0.0),  //vup
            45.0,                    //vertical_fov
            2.0,                     //aspect
            0.02,                    //aperture
            2.0,                     //focus_dist
            0.0,                     //time0
            1.0,                     //time1
        ),
        world: Box::new(hitable)
    }
    .loop_render(image_config, 12);
}
