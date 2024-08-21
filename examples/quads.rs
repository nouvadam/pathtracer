use pathtracer::hittables::{BvhNode, HittableList};
use pathtracer::material::{Lambertian, MaterialContainer};
use pathtracer::misc::Interval;
use pathtracer::primitive::Quad;
use pathtracer::texture::ConstantTexture;
use pathtracer::*;

pub fn main() {
    let mut hittable = HittableList::new();
    let mut materials = MaterialContainer::default();

    let left_red = materials.add(Lambertian::new(Box::new(ConstantTexture {
        color: V3::new(1.0, 0.2, 0.2),
    })));
    let back_green = materials.add(Lambertian::new(Box::new(ConstantTexture {
        color: V3::new(0.2, 1.0, 0.2),
    })));
    let right_blue = materials.add(Lambertian::new(Box::new(ConstantTexture {
        color: V3::new(0.2, 0.2, 1.0),
    })));
    let upper_orange = materials.add(Lambertian::new(Box::new(ConstantTexture {
        color: V3::new(1.0, 0.5, 0.0),
    })));
    let lower_teal = materials.add(Lambertian::new(Box::new(ConstantTexture {
        color: V3::new(0.2, 0.8, 0.8),
    })));

    hittable.add(Quad::new(
        V3::new(-3.0, -2.0, 5.0),
        V3::new(0.0, 0.0, -4.0),
        V3::new(0.0, 4.0, 0.0),
        left_red,
    ));
    hittable.add(Quad::new(
        V3::new(-2.0, -2.0, 0.0),
        V3::new(4.0, 0.0, 0.0),
        V3::new(0.0, 4.0, 0.0),
        back_green,
    ));
    hittable.add(Quad::new(
        V3::new(3.0, -2.0, 1.0),
        V3::new(0.0, 0.0, 4.0),
        V3::new(0.0, 4.0, 0.0),
        right_blue,
    ));
    hittable.add(Quad::new(
        V3::new(-2.0, 3.0, 1.0),
        V3::new(4.0, 0.0, 0.0),
        V3::new(0.0, 0.0, 4.0),
        upper_orange,
    ));
    hittable.add(Quad::new(
        V3::new(-2.0, -3.0, 5.0),
        V3::new(4.0, 0.0, 0.0),
        V3::new(0.0, 0.0, -4.0),
        lower_teal,
    ));

    let image_config = ImageConfig {
        nx: 1024,
        ny: 1024,
        samples_per_pixel: 128,
        ray_setting: RaySetting {
            background_color: V3::new(0.5, 0.7, 1.0),
            depth: 32,
            ray_time: Interval {
                min: 0.001,
                max: 2048.0,
            },
        },
        name: "quads",
    };

    let scene = Scene {
        camera: Camera::new(
            V3::new(0.0, 0.0, 9.0), //lookfrom
            V3::new(0.0, 0.0, 0.0), //lookat
            V3::new(0.0, 1.0, 0.0), //vup
            80.0,                   //vertical_fov
            1.0,                    //aspect
            0.1,                    //aperture
            10.0,                   //focus_dist
            0.0,                    //time0
            1.0,                    //time1
        ),
        world: BvhNode::new(&hittable),
        lights: None,
        materials,
    };

    scene.loop_render(image_config, 8);
}
