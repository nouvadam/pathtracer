use itertools::*;
use rand::Rng;

use pathtracer::hitables::*;
use pathtracer::material::*;
use pathtracer::misc::*;
use pathtracer::primitive::*;
use pathtracer::texture::*;
use pathtracer::transform::*;
use pathtracer::*;

fn main() {
    let mut boxes = HitableList::new();

    let ground = Lambertian {
        albedo: Box::new(ConstantTexture {
            color: V3::new(0.48, 0.83, 0.53),
        }),
    };

    let boxes_per_side = 20;

    iproduct!(0..boxes_per_side, 0..boxes_per_side).for_each(|point| {
        let w = 100.0;
        let x0 = -1000.0 + (point.0 as f32) * w;
        let z0 = -1000.0 + (point.1 as f32) * w;
        let y0 = 0.0;
        let x1 = x0 + w;
        let y1 = rand::thread_rng().gen_range(1.0, 101.0);
        let z1 = z0 + w;

        boxes.add(Box::new(HitBox::new(
            V3::new(x0, y0, z0),
            V3::new(x1, y1, z1),
            Box::new(ground.clone()),
        )))
    });

    let mut objects = HitableList::new();

    objects.add(Box::new(BvhNode::new(boxes)));

    let light = LightSource {
        albedo: Box::new(ConstantTexture {
            color: V3::new(7.0, 7.0, 7.0),
        }),
    };

    objects.add(Box::new(XZrect {
        x0: 123.0,
        x1: 423.0,
        z0: 147.0,
        z1: 412.0,
        k: 554.0,
        material: Box::new(light),
    }));

    let moving_sphere_material = ConstantTexture {
        color: V3::new(0.7, 0.3, 0.1),
    };

    objects.add(Box::new(MovingSphere {
        centers: (V3::new(400.0, 400.0, 200.0), V3::new(430.0, 400.0, 200.0)),
        time_range: (0.0, 1.0),
        radius: 50.0,
        material: Box::new(Lambertian {
            albedo: Box::new(moving_sphere_material),
        }),
    }));

    objects.add(Box::new(Sphere {
        center: V3::new(260.0, 150.0, 45.0),
        radius: 50.0,
        material: Box::new(Dielectric {
            refractive_index: 1.5,
        }),
    }));

    objects.add(Box::new(Sphere {
        center: V3::new(360.0, 150.0, 145.0),
        radius: 70.0,
        material: Box::new(Dielectric {
            refractive_index: 1.5,
        }),
    }));

    objects.add(
        Box::new(Sphere {
            center: V3::new(360.0, 150.0, 145.0),
            radius: 70.0,
            material: Box::new(Dielectric {
                refractive_index: 1.5,
            }),
        })
        .into_constant_medium(
            0.2,
            Box::new(Isotropic {
                albedo: Box::new(ConstantTexture {
                    color: V3::new(0.2, 0.4, 0.9),
                }),
            }),
        ),
    );

    objects.add(
        Box::new(Sphere {
            center: V3::new(0.0, 0.0, 0.0),
            radius: 5000.0,
            material: Box::new(Dielectric {
                refractive_index: 1.5,
            }),
        })
        .into_constant_medium(
            0.0001,
            Box::new(Isotropic {
                albedo: Box::new(ConstantTexture {
                    color: V3::new(1.0, 1.0, 1.0),
                }),
            }),
        ),
    );

    objects.add(Box::new(Sphere {
        center: V3::new(400.0, 200.0, 400.0),
        radius: 100.0,
        material: Box::new(Lambertian {
            albedo: Box::new(ImageTexture::new("assets/earthmap.png")),
        }),
    }));

    objects.add(Box::new(Sphere {
        center: V3::new(0.0, 150.0, 145.0),
        radius: 50.0,
        material: Box::new(Metalic {
            albedo: V3::new(0.8, 0.8, 0.9),
            fuzz: 1.0,
        }),
    }));

    let perlin_texture = PerlinNoiseTexture {
        perlin_noise: Perlin::new(),
        scale: 0.1,
    };

    objects.add(Box::new(Sphere {
        center: V3::new(220.0, 280.0, 300.0),
        radius: 80.0,
        material: Box::new(Lambertian {
            albedo: Box::new(perlin_texture),
        }),
    }));

    let mut boxes2 = HitableList::new();

    let white = Lambertian {
        albedo: Box::new(ConstantTexture {
            color: V3::new(0.73, 0.73, 0.73),
        }),
    };

    (0..1000).for_each(|_x| {
        boxes2.add(Box::new(Sphere {
            center: V3::new(
                rand::thread_rng().gen_range(0.0, 165.0),
                rand::thread_rng().gen_range(0.0, 165.0),
                rand::thread_rng().gen_range(0.0, 165.0),
            ),
            radius: 10.0,
            material: Box::new(white.clone()),
        }));
    });

    objects.add(
        Box::new(BvhNode::new(boxes2))
            .rotate(V3::new(0.0, 1.0, 0.0), 0.261_799_4)
            .translate(V3::new(-100.0, 270.0, 395.0)),
    );

    let image_config = ImageConfig {
        nx: 1024,
        ny: 1024,
        samples_per_pixel: 2,
        ray_setting: RaySetting {
            background_color: V3::new(0.0, 0.0, 0.0),
            depth: 32,
        },
        name: "final_scene",
    };

    let scene = Scene {
        camera: Camera::new(
            V3::new(478.0, 278.0, -600.0), //lookfrom
            V3::new(278.0, 278.0, 0.0),    //lookat
            V3::new(0.0, 1.0, 0.0),        //vup
            40.0,                          //vertical_fov
            1.0,                           //aspect
            0.03,                          //aperture
            661.0,                         //focus_dist
            0.0,                           //time0
            1.0,                           //time1
        ),
        world: Box::new(objects),
    };

    scene.loop_render(image_config, 12);
}
