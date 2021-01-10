use raytracer::*;
use raytracer::material::*;
use raytracer::primitive::*;
use raytracer::texture::*;
use raytracer::transform::*;
use raytracer::hitables::*;

// https://www.cgtrader.com/free-3d-print-models/art/sculptures/pegasus-statue-sculpture-statuette-figurine-horse

fn main() {
    let mut hitable = HitableList::new();

    let _triangle_material = Box::new(Lambertian {
        albedo: Box::new(ConstantTexture {
            color: V3::new(1.0, 0.3, 0.3),
        }),
    });

    let _glass_material = Box::new(Dielectric {
        refractive_index: 1.5,
    });

    let red = Lambertian {
        albedo: Box::new(ConstantTexture {
            color: V3::new(0.65, 0.05, 0.05),
        }),
    };
    let white = Lambertian {
        albedo: Box::new(ConstantTexture {
            color: V3::new(0.73, 0.73, 0.73),
        }),
    };
    let green = Lambertian {
        albedo: Box::new(ConstantTexture {
            color: V3::new(0.12, 0.45, 0.15),
        }),
    };
    let light = LightSource {
        albedo: Box::new(ConstantTexture {
            color: V3::new(15.0, 15.0, 15.0),
        }),
    };

    hitable.add(Box::new(YZrect {
        y0: 0.0,
        y1: 1.7,
        z0: 0.0,
        z1: 1.7,
        k: 1.7,
        material: Box::new(green),
    }));

    hitable.add(Box::new(YZrect {
        y0: 0.0,
        y1: 1.7,
        z0: 0.0,
        z1: 1.7,
        k: 0.0,
        material: Box::new(red),
    }));

    hitable.add(Box::new(XZrect {
        x0: 0.595,
        x1: 1.105,
        z0: 0.595,
        z1: 1.105,
        k: 1.69,
        material: Box::new(light),
    }));

    hitable.add(Box::new(XZrect {
        x0: 0.0,
        x1: 1.7,
        z0: 0.0,
        z1: 1.7,
        k: 0.0,
        material: Box::new(white.clone()),
    }));

    hitable.add(Box::new(XZrect {
        x0: 0.0,
        x1: 1.7,
        z0: 0.0,
        z1: 1.7,
        k: 1.7,
        material: Box::new(white.clone()),
    }));

    hitable.add(Box::new(XYrect {
        x0: 0.0,
        x1: 1.7,
        y0: 0.0,
        y1: 1.7,
        k: 1.7,
        material: Box::new(white.clone()),
    }));

    hitable.add(
        Box::new(Mesh::new("assets/pegasus.obj", _glass_material).unwrap())
            //.rotate(V3::new(0.0, 1.0, 0.0), 0.8)
            .rotate(V3::new(0.0, 1.0, 0.0), 3.1415)
            .translate(V3::new(0.85, 0.0, 0.85))
    );

    let image_config = ImageConfig {
        nx: 1024,
        ny: 1024,
        samples_per_pixel: 2,
        ray_setting: RaySetting {
            background_color: V3::new(0.0, 0.0, 0.0),
            depth: 16,
        },
        name: r#"pegasus"#
    };

    Scene {
        camera: Camera::new(
            V3::new(0.85, 0.85, -1.7),   //lookfrom
            V3::new(0.85, 0.85, 0.0), //lookat
            V3::new(0.0, 1.0, 0.0),   //vup
            45.0,                     //vertical_fov
            1.0,                      //aspect
            0.0,                      //aperture
            5.0,                      //focus_dist
            0.0,                      //time0
            1.0,                      //time1
        ),
        world: Box::new(BvhNode::new(hitable))
    }
    .loop_render(image_config, 12);
}
