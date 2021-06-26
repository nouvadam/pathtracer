use pathtracer::hittables::*;
use pathtracer::material::*;
use pathtracer::primitive::*;
use pathtracer::texture::*;
use pathtracer::transform::*;
use pathtracer::*;

use rand::Rng;

fn main() {
    let mut hittable = HittableList::new();
    let mut lights = HittableList::new();
    let mut materials = MaterialContainer::new();

    hittable.add(Sphere::new(
        V3::new(0.0, -100.5, -2.0),
        100.0,
        materials.add(Lambertian::new(Box::new(ConstantTexture {
            color: V3::new(0.5, 0.5, 0.5),
        }))),
    ));

    let scale = 2.0;
    let mut seed = rand::thread_rng();

    hittable.add(Sphere::new(
        V3::new(-1.0, 0.0, -2.0),
        0.5,
        materials.add(LightSource::new(Box::new(PlasmaTexture {
            param: seed.gen::<f32>() * 100.0,
            scale,
        }))),
    ));

    lights.add(Sphere::new(
        V3::new(-1.0, 0.0, -2.0),
        0.5,
        materials.add(LightSource::new(Box::new(PlasmaTexture {
            param: seed.gen::<f32>() * 100.0,
            scale,
        }))),
    ));

    hittable.add(
        XYrect::new(
            -0.5,
            0.5,
            -0.5,
            0.5,
            0.0,
            materials.add(LightSource::new(Box::new(PlasmaTexture {
                param: seed.gen::<f32>() * 100.0,
                scale: 1.0,
            }))),
        )
        //.rotate(V3::new(0.0, 1.0, 0.0), -0.8)
        .translate(V3::new(0.5, 0.0, -2.0)),
    );

    lights.add(
        XYrect::new(
            -0.5,
            0.5,
            -0.5,
            0.5,
            0.0,
            materials.add(LightSource::new(Box::new(PlasmaTexture {
                param: seed.gen::<f32>() * 100.0,
                scale: 1.0,
            }))),
        )
        //.rotate(V3::new(0.0, 1.0, 0.0), -0.8)
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
        name: r#"plasma_test"#,
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
        world: BvhNode::new(&hittable),
        lights: Some(lights),
        materials,
    }
    .loop_render(image_config, 12);
}
