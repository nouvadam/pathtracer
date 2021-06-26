use pathtracer::hittables::*;
use pathtracer::material::*;
use pathtracer::misc::*;
use pathtracer::texture::*;
use pathtracer::*;
fn main() {
    let mut hittable = HittableList::new();
    let mut materials = MaterialContainer::new();
    let mut lights = HittableList::new();

    let perlin_texture = PerlinNoiseTexture {
        perlin_noise: Perlin::new(),
        scale: 5.0,
    };

    hittable.add(Sphere::new(
        V3::new(0.0, -1000.0, 0.0),
        1000.0,
        materials.add(Lambertian::new(Box::new(perlin_texture))),
    ));

    let perlin_texture = PerlinNoiseTexture {
        perlin_noise: Perlin::new(),
        scale: 5.0,
    };

    hittable.add(Sphere::new(
        V3::new(0.0, 2.0, 0.0),
        2.0,
        materials.add(Lambertian::new(Box::new(perlin_texture))),
    ));

    hittable.add(XYrect::new(
        3.0,
        5.0,
        1.0,
        3.0,
        -2.0,
        materials.add(LightSource::new(Box::new(ConstantTexture {
            color: V3::new(4.0, 4.0, 4.0),
        }))),
    ));

    lights.add(XYrect::new(
        3.0,
        5.0,
        1.0,
        3.0,
        -2.0,
        materials.add(LightSource::new(Box::new(ConstantTexture {
            color: V3::new(4.0, 4.0, 4.0),
        }))),
    ));

    let image_config = ImageConfig {
        nx: 2048,
        ny: 1024,
        samples_per_pixel: 2,
        ray_setting: RaySetting {
            background_color: V3::new(0.0, 0.0, 0.0),
            depth: 4,
        },
        name: "simple_light",
    };

    Scene {
        camera: Camera::new(
            V3::new(26.0, 3.0, 6.0), //lookfrom
            V3::new(0.0, 2.0, 0.0),  //lookat
            V3::new(0.0, 1.0, 0.0),  //vup
            20.0,                    //vertical_fov
            2.0,                     //aspect
            0.05,                    //aperture
            26.0,                    //focus_dist
            0.0,                     //time0
            1.0,                     //time1
        ),
        world: BvhNode::new(&hittable),
        lights: Some(lights),
        materials,
    }
    .loop_render(image_config, 12);
}
