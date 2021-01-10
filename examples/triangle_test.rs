/*use raytracer::camera::Camera;
use raytracer::hitable_list::HitableList;
use raytracer::image_config::ImageConfig;
use raytracer::scene::Scene;
use raytracer::vec3::V3;

use raytracer::material::lambertian::Lambertian;
use raytracer::primitive::sphere::Sphere;
use raytracer::primitive::triangle::Triangle;
use raytracer::texture::constant_texture::ConstantTexture;
use raytracer::transform::translate::*;*/

//use raytracer::Camera;
use raytracer::*;
use raytracer::material::*;
use raytracer::primitive::*;
use raytracer::texture::*;
use raytracer::transform::*;
use raytracer::hitables::*;

fn main() {
    let mut hitable = HitableList::new();

    let triangle_material_1 = Box::new(Lambertian {
        albedo: Box::new(ConstantTexture {
            color: V3::new(0.5, 1.0, 0.5),
        }),
    });

    let triangle_material_2 = Box::new(Lambertian {
        albedo: Box::new(ConstantTexture {
            color: V3::new(1.0, 0.5, 0.5),
        }),
    });

    hitable.add(Box::new(Triangle::new(
        V3::new(
            V3::new(0.0, 0.5, -2.0),
            V3::new(-0.25, 0.0, -2.0),
            V3::new(0.25, 0.0, -2.0),
        ),
        None,
        triangle_material_1,
    )));

    hitable.add(
        Box::new(Triangle::new(
            V3::new(
                V3::new(0.0, 0.5, -2.0),
                V3::new(-0.25, 0.0, -2.0),
                V3::new(0.25, 0.0, -2.0),
            ),
            None,
            triangle_material_2,
        ))
        .translate(V3::new(0.1, 0.1, 0.1)),
    );

    let image_config = ImageConfig {
        nx: 2048,
        ny: 1024,
        samples_per_pixel: 2,
        ray_setting: RaySetting {
            background_color: V3::new(0.5, 0.7, 1.0),
            depth: 32,
        },
        name: r#"triangle_test"#
    };

    Scene {
        camera: Camera::new(
            V3::new(0.0, 0.0, 0.0),  //lookfrom
            V3::new(0.0, 0.0, -1.0), //lookat
            V3::new(0.0, 1.0, 0.0),  //vup
            45.0,                    //vertical_fov
            2.0,                     //aspect
            0.02,                    //aperture
            2.0,                     //focus_dist
            0.0,                     //time0
            1.0,                     //time1
        ),
        world: Box::new(hitable)
    }
    .loop_render(image_config,12);
}
