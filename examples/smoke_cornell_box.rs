use pathtracer::hittables::*;
use pathtracer::material::*;
use pathtracer::misc::Interval;
use pathtracer::primitive::*;
use pathtracer::texture::*;
use pathtracer::transform::*;
use pathtracer::*;
fn main() {
    let mut hittable = HittableList::new();
    let mut lights = HittableList::new();
    let mut materials = MaterialContainer::new();

    let red = materials.add(Lambertian::new(Box::new(ConstantTexture {
        color: V3::new(0.65, 0.05, 0.05),
    })));
    let white = materials.add(Lambertian::new(Box::new(ConstantTexture {
        color: V3::new(1.00, 1.00, 1.00),
    })));
    let green = materials.add(Lambertian::new(Box::new(ConstantTexture {
        color: V3::new(0.12, 0.45, 0.15),
    })));
    let light = materials.add(LightSource::new(Box::new(ConstantTexture {
        color: V3::new(15.0, 15.0, 15.0),
    })));

    hittable.add(YZrect::new(0.0, 555.0, 0.0, 555.0, 555.0, green));

    hittable.add(YZrect::new(0.0, 555.0, 0.0, 555.0, 0.0, red));

    hittable.add(XZrect::new(250.0, 380.0, 250.0, 380.0, 554.0, light).flip_face());

    lights.add(XZrect::new(250.0, 380.0, 250.0, 380.0, 554.0, light).flip_face());

    hittable.add(XZrect::new(0.0, 555.0, 0.0, 555.0, 0.0, white));

    hittable.add(XZrect::new(0.0, 555.0, 0.0, 555.0, 555.0, white));

    hittable.add(XYrect::new(0.0, 555.0, 0.0, 555.0, 555.0, white));

    let isoblack = materials.add(Isotropic::new(Box::new(ConstantTexture {
        color: V3::new(0.0, 0.0, 0.0),
    })));

    hittable.add(
        HitBox::new(V3::new(0.0, 0.0, 0.0), V3::new(165.0, 330.0, 165.0), white)
            .rotate(V3::new(0.0, 1.0, 0.0), 0.261_799_4)
            .translate(V3::new(265.0, 0.0, 295.0))
            .into_constant_medium(0.01, isoblack),
    );

    let isowhite = materials.add(Isotropic::new(Box::new(ConstantTexture {
        color: V3::new(1.0, 1.0, 1.0),
    })));

    hittable.add(
        HitBox::new(V3::new(0.0, 0.0, 0.0), V3::new(165.0, 165.0, 165.0), white)
            .rotate(V3::new(0.0, 1.0, 0.0), -0.314_159_27)
            .translate(V3::new(130.0, 0.0, 65.0))
            .into_constant_medium(0.01, isowhite),
    );

    let image_config = ImageConfig {
        nx: 1024,
        ny: 1024,
        samples_per_pixel: 2,
        ray_setting: RaySetting {
            background_color: V3::new(0.0, 0.0, 0.0),
            depth: 64,
            ray_time: Interval {
                min: 0.001,
                max: 2048.0,
            },
        },
        name: "smoke_cornell_box",
    };

    Scene {
        camera: Camera::new(
            V3::new(278.0, 278.0, -800.0), //lookfrom
            V3::new(278.0, 278.0, 0.0),    //lookat
            V3::new(0.0, 1.0, 0.0),        //vup
            40.0,                          //vertical_fov
            1.0,                           //aspect
            0.0,                           //aperture
            26.0,                          //focus_dist
            0.0,                           //time0
            1.0,                           //time1
        ),
        world: BvhNode::new(&hittable),
        lights: Some(lights),
        materials,
    }
    .loop_render(image_config, 12);
}
