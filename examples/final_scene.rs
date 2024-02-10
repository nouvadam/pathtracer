use itertools::*;
use rand::Rng;

use pathtracer::hittables::*;
use pathtracer::material::*;
use pathtracer::misc::*;
use pathtracer::texture::*;
use pathtracer::transform::*;
use pathtracer::*;

fn main() {
    let mut boxes = HittableList::new();
    let mut objects = HittableList::new();
    let mut lights = HittableList::new();
    let mut materials = MaterialContainer::new();

    let glass_material = materials.add(Dielectric::new(1.5));

    let ground = materials.add(Lambertian::new(Box::new(ConstantTexture {
        color: V3::new(0.48, 0.83, 0.53),
    })));

    let boxes_per_side = 20;

    iproduct!(0..boxes_per_side, 0..boxes_per_side).for_each(|point| {
        let w = 100.0;
        let x0 = -1000.0 + (point.0 as f32) * w;
        let z0 = -1000.0 + (point.1 as f32) * w;
        let y0 = 0.0;
        let x1 = x0 + w;
        let y1 = rand::thread_rng().gen_range(1.0, 101.0);
        let z1 = z0 + w;

        boxes.add(HitBox::new(
            V3::new(x0, y0, z0),
            V3::new(x1, y1, z1),
            ground,
        ))
    });

    objects.add(BvhNode::new(&boxes));

    let light = materials.add(LightSource::new(Box::new(ConstantTexture {
        color: V3::new(7.0, 7.0, 7.0),
    })));

    objects.add(XZrect::new(123.0, 423.0, 147.0, 412.0, 554.0, light).flip_face());

    lights.add(XZrect::new(123.0, 423.0, 147.0, 412.0, 554.0, light).flip_face());

    let moving_sphere_material = ConstantTexture {
        color: V3::new(0.7, 0.3, 0.1),
    };

    objects.add(MovingSphere::new(
        (V3::new(400.0, 400.0, 200.0), V3::new(430.0, 400.0, 200.0)),
        (0.0, 1.0),
        50.0,
        materials.add(Lambertian::new(Box::new(moving_sphere_material))),
    ));

    objects.add(Sphere::new(
        V3::new(260.0, 150.0, 45.0),
        50.0,
        glass_material,
    ));

    objects.add(Sphere::new(
        V3::new(360.0, 150.0, 145.0),
        70.0,
        glass_material,
    ));

    objects.add(
        Sphere::new(V3::new(360.0, 150.0, 145.0), 70.0, glass_material).into_constant_medium(
            0.2,
            materials.add(Isotropic::new(Box::new(ConstantTexture {
                color: V3::new(0.2, 0.4, 0.9),
            }))),
        ),
    );

    objects.add(
        Sphere::new(V3::new(0.0, 0.0, 0.0), 5000.0, glass_material).into_constant_medium(
            0.0001,
            materials.add(Isotropic::new(Box::new(ConstantTexture {
                color: V3::new(1.0, 1.0, 1.0),
            }))),
        ),
    );

    objects.add(Sphere::new(
        V3::new(400.0, 200.0, 400.0),
        100.0,
        materials.add(Lambertian::new(Box::new(ImageTexture::new(
            "assets/earthmap.png",
        )))),
    ));

    objects.add(Sphere::new(
        V3::new(0.0, 150.0, 145.0),
        50.0,
        materials.add(Metalic::new(V3::new(0.8, 0.8, 0.9), 1.0)),
    ));

    let perlin_texture = PerlinNoiseTexture {
        perlin_noise: Perlin::new(),
        scale: 0.1,
    };

    objects.add(Sphere::new(
        V3::new(220.0, 280.0, 300.0),
        80.0,
        materials.add(Lambertian::new(Box::new(perlin_texture))),
    ));

    let mut boxes2 = HittableList::new();

    let white = materials.add(Lambertian::new(Box::new(ConstantTexture {
        color: V3::new(0.73, 0.73, 0.73),
    })));

    (0..1000).for_each(|_x| {
        boxes2.add(Sphere::new(
            V3::new(
                rand::thread_rng().gen_range(0.0, 165.0),
                rand::thread_rng().gen_range(0.0, 165.0),
                rand::thread_rng().gen_range(0.0, 165.0),
            ),
            10.0,
            white,
        ));
    });

    objects.add(
        BvhNode::new(&boxes2)
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
        world: BvhNode::new(&objects),
        lights: Some(lights),
        materials,
    };

    scene.loop_render(image_config, 12);
}
