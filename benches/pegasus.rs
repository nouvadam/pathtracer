use std::time::Duration;

use criterion::{criterion_group, criterion_main, Criterion};
use pathtracer::hittables::BvhNode;
use pathtracer::hittables::HittableList;
use pathtracer::material::*;
use pathtracer::misc::Interval;
use pathtracer::primitive::*;
use pathtracer::texture::ConstantTexture;
use pathtracer::transform::IntoFlipFace;
use pathtracer::transform::IntoRotated;
use pathtracer::transform::IntoTranslated;
use pathtracer::*;

pub fn criterion_benchmark(c: &mut Criterion) {
    let mut hittable = HittableList::new();
    let mut lights = HittableList::new();
    let mut materials = MaterialContainer::new();

    let _triangle_material = materials.add(Lambertian::new(Box::new(ConstantTexture {
        color: V3::new(1.0, 0.3, 0.3),
    })));

    let glass_material = materials.add(Dielectric::new(1.5));

    let red = materials.add(Lambertian {
        albedo: Box::new(ConstantTexture {
            color: V3::new(0.65, 0.05, 0.05),
        }),
    });
    let white = materials.add(Lambertian::new(Box::new(ConstantTexture {
        color: V3::new(0.73, 0.73, 0.73),
    })));
    let green = materials.add(Lambertian {
        albedo: Box::new(ConstantTexture {
            color: V3::new(0.12, 0.45, 0.15),
        }),
    });
    let light = materials.add(LightSource {
        albedo: Box::new(ConstantTexture {
            color: V3::new(15.0, 15.0, 15.0),
        }),
    });

    hittable.add(YZrect {
        y0: 0.0,
        y1: 1.7,
        z0: 0.0,
        z1: 1.7,
        k: 1.7,
        material: green,
    });

    hittable.add(YZrect {
        y0: 0.0,
        y1: 1.7,
        z0: 0.0,
        z1: 1.7,
        k: 0.0,
        material: red,
    });

    hittable.add(
        XZrect {
            x0: 0.595,
            x1: 1.105,
            z0: 0.595,
            z1: 1.105,
            k: 1.69,
            material: light,
        }
        .flip_face(),
    );

    lights.add(
        XZrect {
            x0: 0.595,
            x1: 1.105,
            z0: 0.595,
            z1: 1.105,
            k: 1.69,
            material: light,
        }
        .flip_face(),
    );

    hittable.add(XZrect {
        x0: 0.0,
        x1: 1.7,
        z0: 0.0,
        z1: 1.7,
        k: 0.0,
        material: white,
    });

    hittable.add(XZrect {
        x0: 0.0,
        x1: 1.7,
        z0: 0.0,
        z1: 1.7,
        k: 1.7,
        material: white,
    });

    hittable.add(XYrect {
        x0: 0.0,
        x1: 1.7,
        y0: 0.0,
        y1: 1.7,
        k: 1.7,
        material: white,
    });

    hittable.add(
        Mesh::new("assets/pegasus.obj", glass_material)
            .unwrap()
            //.rotate(V3::new(0.0, 1.0, 0.0), 0.8)
            .rotate(V3::new(0.0, 1.0, 0.0), 3.1415)
            .translate(V3::new(0.85, 0.0, 0.85)),
    );

    let image_config = ImageConfig {
        nx: 1024,
        ny: 1024,
        samples_per_pixel: 16,
        ray_setting: RaySetting {
            background_color: V3::new(0.0, 0.0, 0.0),
            depth: 16,
            ray_time: Interval {
                min: 0.001,
                max: 2048.0,
            },
        },
        name: "pegasus",
    };

    let scene = Scene {
        camera: Camera::new(
            V3::new(0.85, 0.85, -1.7), //lookfrom
            V3::new(0.85, 0.85, 0.0),  //lookat
            V3::new(0.0, 1.0, 0.0),    //vup
            45.0,                      //vertical_fov
            1.0,                       //aspect
            0.0,                       //aperture
            5.0,                       //focus_dist
            0.0,                       //time0
            1.0,                       //time1
        ),
        world: BvhNode::new(&hittable),
        lights: Some(lights),
        materials,
    };

    let mut group = c.benchmark_group("renders");
    group.warm_up_time(Duration::new(15, 0));
    group.bench_function("pegasus", |b| b.iter(|| scene.render(image_config)));
    group.finish();
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
