use crate::hittables::HittableList;
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
    let mut materials = MaterialContainer::default();

    let red = materials.add(Lambertian::new(Box::new(ConstantTexture {
        color: V3::new(0.65, 0.05, 0.05),
    })));
    let white = materials.add(Lambertian::new(Box::new(ConstantTexture {
        color: V3::new(0.73, 0.73, 0.73),
    })));
    let green = materials.add(Lambertian::new(Box::new(ConstantTexture {
        color: V3::new(0.12, 0.45, 0.15),
    })));
    let light = materials.add(LightSource::new(Box::new(ConstantTexture {
        color: V3::new(15.0, 15.0, 15.0),
    })));

    let aluminium = materials.add(Metalic::new(V3::new(0.8, 0.85, 0.88), 0.0));

    let glass = materials.add(Dielectric::new(1.5));

    hittable.add(YZrect::new(0.0, 555.0, 0.0, 555.0, 555.0, green));

    hittable.add(YZrect::new(0.0, 555.0, 0.0, 555.0, 0.0, red));

    hittable.add(XZrect::new(213.0, 343.0, 227.0, 332.0, 554.0, light).flip_face());

    hittable.add(XZrect::new(0.0, 555.0, 0.0, 555.0, 0.0, white));

    hittable.add(XZrect::new(0.0, 555.0, 0.0, 555.0, 555.0, white));

    hittable.add(XYrect::new(0.0, 555.0, 0.0, 555.0, 555.0, white));

    hittable.add(
        HitBox::new(
            V3::new(0.0, 0.0, 0.0),
            V3::new(165.0, 330.0, 165.0),
            aluminium,
        )
        .rotate(V3::new(0.0, 1.0, 0.0), 0.261_799_4)
        .translate(V3::new(265.0, 0.0, 295.0)),
    );

    /*hittable.add(
        Primitive::HitBox(HitBox::new(
            V3::new(0.0, 0.0, 0.0),
            V3::new(165.0, 165.0, 165.0),
            &white,
        ))
        .rotate(V3::new(0.0, 1.0, 0.0), -0.314_159_27)
        .translate(V3::new(130.0, 0.0, 65.0)),
    );*/

    hittable.add(Sphere::new(V3::new(190.0, 90.0, 190.0), 90.0, glass));

    lights.add(XZrect::new(213.0, 343.0, 227.0, 332.0, 554.0, light).flip_face());

    lights.add(Sphere {
        center: V3::new(190.0, 90.0, 190.0),
        radius: 90.0,
        material: glass,
    });

    let image_config = ImageConfig {
        nx: 1024,
        ny: 1024,
        samples_per_pixel: 2,
        ray_setting: RaySetting {
            background_color: V3::new(0.0, 0.0, 0.0),
            depth: 32,
            ray_time: Interval {
                min: 0.001,
                max: 2048.0,
            },
        },
        name: "cornell_box_ball",
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
