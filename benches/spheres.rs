use std::time::Duration;

use criterion::{criterion_group, criterion_main, Criterion};
use pathtracer::hittables::BvhNode;
use pathtracer::hittables::HittableList;
use pathtracer::material::*;
use pathtracer::misc::Interval;
use pathtracer::primitive::*;
use pathtracer::*;

pub fn criterion_benchmark(c: &mut Criterion) {
    let mut hittable = HittableList::new();
    let mut materials = MaterialContainer::default();

    //ground
    hittable.add(Sphere::new(
        V3::new(0.0, -100.5, -1.0),
        100.0,
        materials.add(Metalic::new(V3::new(0.8, 0.8, 0.0), 1.0)),
    ));

    //center
    hittable.add(Sphere::new(
        V3::new(0.0, 0.0, -1.0),
        0.5,
        materials.add(Metalic::new(V3::new(0.1, 0.2, 0.5), 1.0)),
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
        nx: 512,
        ny: 512,
        samples_per_pixel: 16,
        ray_setting: RaySetting {
            background_color: V3::new(0.5, 0.7, 1.0),
            depth: 32,
            ray_time: Interval {
                min: 0.001,
                max: 2048.0,
            },
        },
        name: "glass_sphere",
    };

    let scene = Scene {
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
    };
    let mut group = c.benchmark_group("renders");
    group.warm_up_time(Duration::new(15, 0));
    group.bench_function("spheres", |b| b.iter(|| scene.render(image_config)));
    group.finish();
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
