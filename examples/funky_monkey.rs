use pathtracer::hittables::*;
use pathtracer::material::*;
use pathtracer::misc::Interval;
use pathtracer::primitive::*;
use pathtracer::texture::*;
use pathtracer::transform::*;
use pathtracer::*;

use rand::Rng;

// Blender monkey

fn main() {
    let mut hittable = HittableList::new();
    let mut lights = HittableList::new();
    let mut materials = MaterialContainer::default();

    let scale = 2.0;
    let mut seed = rand::thread_rng();

    let glass_material = materials.add(Dielectric::new(1.5));

    hittable.add(
        XYrect::new(
            -5.0,
            5.0,
            -5.0,
            5.0,
            0.0,
            materials.add(LightSource::new(Box::new(PlasmaTexture {
                param: seed.gen::<f32>() * 100.0,
                scale,
            }))),
        )
        .translate(V3::new(0.0, 0.0, -10.0)),
    );

    lights.add(
        XYrect::new(
            -5.0,
            5.0,
            -5.0,
            5.0,
            0.0,
            materials.add(LightSource::new(Box::new(PlasmaTexture {
                param: seed.gen::<f32>() * 100.0,
                scale,
            }))),
        )
        .translate(V3::new(0.0, 0.0, -10.0)),
    );

    hittable.add(
        Mesh::new("assets/monkey (2).obj", glass_material)
            .unwrap()
            .translate(V3::new(0.0, 0.0, -4.0)),
    );

    let image_config = ImageConfig {
        nx: 1024,
        ny: 1024,
        samples_per_pixel: 2,
        ray_setting: RaySetting {
            background_color: V3::new(0.0, 0.0, 0.0),
            depth: 32,
            ray_time: Interval {
                min: 0.001,
                max: 2048.0,
            },
        },
        name: "funky_monkey",
    };

    Scene {
        camera: Camera::new(
            V3::new(0.0, 0.0, 0.0),  //lookfrom
            V3::new(0.0, 0.0, -1.0), //lookat
            V3::new(0.0, 1.0, 0.0),  //vup
            45.0,                    //vertical_fov
            1.0,                     //aspect
            0.0,                     //aperture
            2.0,                     //focus_dist
            0.0,                     //time0
            1.0,                     //time1
        ),
        world: BvhNode::new(&hittable),
        lights: None,
        materials,
    }
    .loop_render(image_config, 12);
}
