use rand::Rng;
use raytracer::*;
use raytracer::material::*;
use raytracer::primitive::*;
use raytracer::texture::*;
use raytracer::hitables::*;

fn main() {
    let mut hitable = HitableList::new();
    let mut seed = rand::thread_rng();

    let checker_texture = CheckerTexture {
        odd: Box::new(ConstantTexture {
            color: V3::new(0.2, 0.3, 0.1),
        }),
        even: Box::new(ConstantTexture {
            color: V3::new(0.9, 0.9, 0.9),
        }),
    };

    hitable.add(Box::new(Sphere {
        center: V3::new(0.0, -1000.0, 0.0),
        radius: 1000.0,
        material: Box::new(Lambertian {
            albedo: Box::new(checker_texture),
        }),
    }));
    let mut hitable2 = HitableList::new();
    for a in -15..15 {
        for b in -15..15 {
            let choose_mat: f32 = seed.gen();
            let center = V3::new(
                (a as f32) + 0.9 * seed.gen::<f32>(),
                0.2,
                (b as f32) + 0.9 * seed.gen::<f32>(),
            );
            if (center - V3::new(4.0, 0.2, 0.0)).len() > 0.9 {
                if choose_mat < 0.6 {
                    hitable2.add(Box::new(Sphere {
                        center,
                        radius: 0.2,
                        material: Box::new(Lambertian {
                            albedo: Box::new(ConstantTexture {
                                color: V3::new(
                                    seed.gen::<f32>() * seed.gen::<f32>(),
                                    seed.gen::<f32>() * seed.gen::<f32>(),
                                    seed.gen::<f32>() * seed.gen::<f32>(),
                                ),
                            }),
                        }),
                    }));
                } else if choose_mat < 0.8 {
                    hitable2.add(Box::new(Sphere {
                        center,
                        radius: 0.2,
                        material: Box::new(LightSource {
                            albedo: Box::new(ConstantTexture {
                                color: V3::new(
                                    seed.gen::<f32>() * seed.gen::<f32>(),
                                    seed.gen::<f32>() * seed.gen::<f32>(),
                                    seed.gen::<f32>() * seed.gen::<f32>(),
                                )
                                .norm()
                                    * (15.0 + 15.0 * seed.gen::<f32>()),
                            }),
                        }),
                    }));
                } else if choose_mat < 0.88 {
                    hitable2.add(Box::new(Sphere {
                        center,
                        radius: 0.2,
                        material: Box::new(Metalic {
                            albedo: V3::new(
                                0.5 * (1.0 + seed.gen::<f32>()),
                                0.5 * (1.0 + seed.gen::<f32>()),
                                0.5 * (1.0 + seed.gen::<f32>()),
                            ),
                            fuzz: 0.5 * seed.gen::<f32>(),
                        }),
                    }));
                } else {
                    hitable2.add(Box::new(Sphere {
                        center,
                        radius: 0.2,
                        material: Box::new(Dielectric {
                            refractive_index: 1.5,
                        }),
                    }));
                }
            }
        }
    }

    hitable2.add(Box::new(Sphere {
        center: V3::new(-4.0, 1.0, 0.0),
        radius: 1.0,
        material: Box::new(Lambertian {
            albedo: Box::new(ConstantTexture {
                color: V3::new(0.4, 0.2, 0.1),
            }),
        }),
    }));
    hitable2.add(Box::new(Sphere {
        center: V3::new(4.0, 1.0, 0.0),
        radius: 1.0,
        material: Box::new(Metalic {
            albedo: V3::new(0.7, 0.6, 0.5),
            fuzz: 0.0,
        }),
    }));
    hitable2.add(Box::new(Sphere {
        center: V3::new(0.0, 1.0, 0.0),
        radius: 1.0,
        material: Box::new(Dielectric {
            refractive_index: 1.5,
        }),
    }));

    hitable.add(Box::new(BvhNode::new(hitable2)));

    let image_config = ImageConfig {
        nx: 2048,
        ny: 1024,
        samples_per_pixel: 2,
        ray_setting: RaySetting {
            background_color: V3::new(0.0, 0.0, 0.0),
            depth: 128,
        },
        name: "random_scene_3"
    };

    Scene {
        
        camera: Camera::new(
            V3::new(13.0, 2.0, 3.0), //lookfrom
            V3::new(0.0, 0.0, 0.0),  //lookat
            V3::new(0.0, 1.0, 0.0),  //vup
            20.0,                    //vertical_fov
            2.0,                     //aspect
            0.1,                     //aperture
            10.0,                    //focus_dist
            0.0,                     //time0
            1.0,                     //time1
        ),
        world: Box::new(hitable)
    }
    .loop_render(image_config, 12);
}
