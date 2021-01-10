use raytracer::*;
use raytracer::material::*;
use raytracer::primitive::*;
use raytracer::texture::*;
use raytracer::transform::*;
use raytracer::hitables::*;
//use raytracer::misc::*;

// Stanford dragon

fn main() {
    let mut hitable = HitableList::new();

    let _triangle_material = Box::new(Lambertian {
        albedo: Box::new(ConstantTexture {
            color: V3::new(0.4, 0.4, 0.9),
        }),
    });

    let gold_metal = Box::new(Metalic {
        albedo: V3::new(0.831, 0.686, 0.215),
        fuzz: 0.2,
    });

    hitable.add(Box::new(Sphere {
        center: V3::new(0.0, -100.0, -10.0),
        radius: 100.0,
        material: _triangle_material,
    }));

    hitable.add(
        Box::new(Mesh::new("assets/dragon.obj", gold_metal).unwrap())
            //.rotate(V3::new(0.0, 1.0, 0.0), 0.8)
            .rotate(V3::new(0.0, 1.0, 0.0), 3.1415)
            .translate(V3::new(0.0, 0.0, 1.0))
    );

    let image_config = ImageConfig {
        nx: 1024,
        ny: 1024,
        samples_per_pixel: 1,
        ray_setting: RaySetting {
            background_color: V3::new(0.5, 0.7, 1.0),
            depth: 16,
        },
        name: r#"dragon"#
    };

    Scene {
        camera: Camera::new(
            V3::new(0.0, 1.0, -2.0),   //lookfrom
            V3::new(0.0, 0.4, 0.0), //lookat
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
