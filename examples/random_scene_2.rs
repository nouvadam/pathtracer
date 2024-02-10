use pathtracer::hittables::*;
use pathtracer::material::*;
use pathtracer::primitive::*;
use pathtracer::texture::*;
use pathtracer::*;
use rand::Rng;
fn main() {
    let mut hittable = HittableList::new();
    let mut materials = MaterialContainer::new();
    let mut lights = HittableList::new();
    let mut seed = rand::thread_rng();

    let checker_texture = CheckerTexture {
        odd: Box::new(ConstantTexture {
            color: V3::new(0.2, 0.3, 0.1),
        }),
        even: Box::new(ConstantTexture {
            color: V3::new(0.9, 0.9, 0.9),
        }),
    };

    hittable.add(Sphere::new(
        V3::new(0.0, -1000.0, 0.0),
        1000.0,
        materials.add(Lambertian::new(Box::new(checker_texture))),
    ));
    let mut hittable2 = HittableList::new();
    for a in -15..15 {
        for b in -15..15 {
            let choose_mat: f32 = seed.gen();
            let center = V3::new(
                (a as f32) + 0.9 * seed.gen::<f32>(),
                0.2,
                (b as f32) + 0.9 * seed.gen::<f32>(),
            );
            if (center - V3::new(4.0, 0.2, 0.0)).length() > 1.2 {
                if choose_mat < 0.8 {
                    hittable2.add(Sphere::new(
                        center,
                        0.2,
                        materials.add(Lambertian::new(Box::new(ConstantTexture {
                            color: V3::new(
                                seed.gen::<f32>() * seed.gen::<f32>(),
                                seed.gen::<f32>() * seed.gen::<f32>(),
                                seed.gen::<f32>() * seed.gen::<f32>(),
                            ),
                        }))),
                    ));
                } else if choose_mat < 0.88 {
                    hittable2.add(Sphere::new(
                        center,
                        0.2,
                        materials.add(Metalic::new(
                            V3::new(
                                0.5 * (1.0 + seed.gen::<f32>()),
                                0.5 * (1.0 + seed.gen::<f32>()),
                                0.5 * (1.0 + seed.gen::<f32>()),
                            ),
                            0.5 * seed.gen::<f32>(),
                        )),
                    ));
                } else {
                    hittable2.add(Sphere::new(
                        center,
                        0.2,
                        materials.add(Dielectric::new(1.5)),
                    ));
                }
            }
        }
    }

    hittable2.add(Sphere::new(
        V3::new(4.0, 0.2, 0.0),
        0.1,
        materials.add(LightSource::new(Box::new(ConstantTexture {
            color: V3::new(1.0, 1.0, 1.0) * 300.0 as f32,
        }))),
    ));

    lights.add(Sphere::new(
        V3::new(4.0, 0.2, 0.0),
        0.1,
        materials.add(LightSource::new(Box::new(ConstantTexture {
            color: V3::new(1.0, 1.0, 1.0) * 300.0 as f32,
        }))),
    ));

    hittable.add(BvhNode::new(&hittable2));

    let image_config = ImageConfig {
        nx: 2048,
        ny: 1024,
        samples_per_pixel: 2,
        ray_setting: RaySetting {
            background_color: V3::new(0.0, 0.0, 0.0),
            depth: 32,
        },
        name: "random_scene_2",
    };

    Scene {
        camera: Camera::new(
            V3::new(13.0, 2.0, 3.0), //lookfrom
            V3::new(0.0, -0.5, 0.0), //lookat
            V3::new(0.0, 1.0, 0.0),  //vup
            20.0,                    //vertical_fov
            2.0,                     //aspect
            0.1,                     //aperture
            10.0,                    //focus_dist
            0.0,                     //time0
            1.0,                     //time1
        ),
        world: BvhNode::new(&hittable),
        lights: Some(lights),
        materials,
    }
    .loop_render(image_config, 12);
}
