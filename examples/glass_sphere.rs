use pathtracer::hittables::*;
use pathtracer::material::*;
use pathtracer::primitive::*;
use pathtracer::texture::*;
use pathtracer::*;

fn main() {
    let mut hittable = HittableList::new();
    let mut materials = MaterialContainer::new();

    //ground
    hittable.add(Sphere::new(
        V3::new(0.0, -100.5, -1.0),
        100.0,
        materials.add(Lambertian::new(Box::new(ConstantTexture {
            color: V3::new(0.8, 0.8, 0.0),
        }))),
    ));

    //center
    hittable.add(Sphere::new(
        V3::new(0.0, 0.0, -1.0),
        0.5,
        materials.add(Lambertian::new(Box::new(ConstantTexture {
            color: V3::new(0.1, 0.2, 0.5),
        }))),
    ));

    //left
    hittable.add(Sphere::new(
        V3::new(-1.0, 0.0, -1.0),
        0.5,
        materials.add(Dielectric::new(1.5)),
    ));

    hittable.add(Sphere::new(
        V3::new(-1.0, 0.0, -1.0),
        -0.45,
        materials.add(Dielectric::new(1.5)),
    ));

    //right
    hittable.add(Sphere::new(
        V3::new(1.0, 0.0, -1.0),
        0.5,
        materials.add(Metalic::new(V3::new(0.8, 0.6, 0.2), 0.0)),
    ));

    let image_config = ImageConfig {
        nx: 2048,
        ny: 1024,
        samples_per_pixel: 2,
        ray_setting: RaySetting {
            background_color: V3::new(0.5, 0.7, 1.0),
            depth: 32,
        },
        name: r#"glass_sphere"#,
    };

    Scene {
        camera: Camera::new(
            V3::new(-2.0, 2.0, 1.0), //lookfrom
            V3::new(0.0, 0.0, -1.0), //lookat
            V3::new(0.0, 1.0, 0.0),  //vup
            20.0,                    //vertical_fov
            2.0,                     //aspect
            0.02,                    //aperture
            3.0,                     //focus_dist
            0.0,                     //time0
            1.0,                     //time1
        ),
        world: BvhNode::new(&hittable),
        lights: None,
        materials,
    }
    .loop_render(image_config, 12);
}
