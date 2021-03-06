use pathtracer::hittables::{BvhNode, HittableList};
use pathtracer::material::{Dielectric, Lambertian, MaterialContainer, Metalic};
use pathtracer::misc::Perlin;
use pathtracer::primitive::{MovingSphere, Sphere};
use pathtracer::texture::{CheckerTexture, ConstantTexture, PerlinNoiseTexture};
use pathtracer::*;
use rand::Rng;

pub fn main() {
    let mut hittable = HittableList::new();
    let mut materials = MaterialContainer::new();
    let mut seed = rand::thread_rng();

    let checker_texture = CheckerTexture {
        odd: Box::new(ConstantTexture {
            color: V3::new(0.2, 0.3, 0.1),
        }),
        even: Box::new(ConstantTexture {
            color: V3::new(0.9, 0.9, 0.9),
        }),
    };

    let perlin_texture = PerlinNoiseTexture {
        perlin_noise: Perlin::new(),
        scale: 5.0,
    };

    let glass_material = materials.add(Dielectric::new(1.5));

    let material = materials.add(Lambertian::new(Box::new(perlin_texture)));

    hittable.add(Sphere::new(V3::new(0.0, -1000.0, 0.0), 1000.0, material));

    let mut hittable2 = HittableList::new();
    for a in -15..15 {
        for b in -15..15 {
            let choose_mat: f32 = seed.gen();
            let center = V3::new(
                (a as f32) + 0.9 * seed.gen::<f32>(),
                0.2,
                (b as f32) + 0.9 * seed.gen::<f32>(),
            );
            if (center - V3::new(4.0, 0.2, 0.0)).length() > 0.9 {
                if choose_mat < 0.75 {
                    let center2 = center + V3::new(0.0, seed.gen_range(0.0, 0.5), 0.0);

                    let material = materials.add(Lambertian::new(Box::new(ConstantTexture {
                        color: V3::new(
                            seed.gen::<f32>() * seed.gen::<f32>(),
                            seed.gen::<f32>() * seed.gen::<f32>(),
                            seed.gen::<f32>() * seed.gen::<f32>(),
                        ),
                    })));

                    hittable2.add(MovingSphere::new(
                        (center, center2),
                        (0.0, 1.0),
                        0.2,
                        material,
                    ));
                } else if choose_mat < 0.83 {
                    let material = materials.add(Metalic::new(
                        V3::new(
                            0.5 * (1.0 + seed.gen::<f32>()),
                            0.5 * (1.0 + seed.gen::<f32>()),
                            0.5 * (1.0 + seed.gen::<f32>()),
                        ),
                        0.5 * seed.gen::<f32>(),
                    ));

                    hittable2.add(Sphere::new(center, 0.2, material));
                } else {
                    hittable2.add(Sphere::new(center, 0.2, glass_material));
                }
            }
        }
    }

    hittable2.add(Sphere::new(
        V3::new(-4.0, 1.0, 0.0),
        1.0,
        materials.add(Lambertian::new(Box::new(checker_texture))),
    ));
    hittable2.add(Sphere::new(
        V3::new(4.0, 1.0, 0.0),
        1.0,
        materials.add(Metalic::new(V3::new(0.7, 0.6, 0.5), 0.0)),
    ));
    hittable2.add(Sphere::new(V3::new(0.0, 1.0, 0.0), 1.0, glass_material));

    hittable.add(Primitive::BvhNode(BvhNode::new(&hittable2)));

    let image_config = ImageConfig {
        nx: 2048,
        ny: 1024,
        samples_per_pixel: 2,
        ray_setting: RaySetting {
            background_color: V3::new(0.5, 0.7, 1.0),
            depth: 32,
        },
        name: "bouncing_spheres",
    };

    let scene = Scene {
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
        world: BvhNode::new(&hittable),
        lights: None,
        materials,
    };

    scene.loop_render(image_config, 8);
}
