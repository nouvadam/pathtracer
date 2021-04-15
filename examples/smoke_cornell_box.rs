use pathtracer::hitables::*;
use pathtracer::material::*;
use pathtracer::primitive::*;
use pathtracer::texture::*;
use pathtracer::transform::*;
use pathtracer::*;
fn main() {
    let mut hitable = HitableList::new();

    let red = Lambertian {
        albedo: Box::new(ConstantTexture {
            color: V3::new(0.65, 0.05, 0.05),
        }),
    };
    let white = Lambertian {
        albedo: Box::new(ConstantTexture {
            color: V3::new(1.00, 1.00, 1.00),
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
        y1: 555.0,
        z0: 0.0,
        z1: 555.0,
        k: 555.0,
        material: Box::new(green),
    }));

    hitable.add(Box::new(YZrect {
        y0: 0.0,
        y1: 555.0,
        z0: 0.0,
        z1: 555.0,
        k: 0.0,
        material: Box::new(red),
    }));

    hitable.add(Box::new(XZrect {
        x0: 250.0,
        x1: 380.0,
        z0: 250.0,
        z1: 380.0,
        k: 554.0,
        material: Box::new(light),
    }));

    hitable.add(Box::new(XZrect {
        x0: 0.0,
        x1: 555.0,
        z0: 0.0,
        z1: 555.0,
        k: 0.0,
        material: Box::new(white.clone()),
    }));

    hitable.add(Box::new(XZrect {
        x0: 0.0,
        x1: 555.0,
        z0: 0.0,
        z1: 555.0,
        k: 555.0,
        material: Box::new(white.clone()),
    }));

    hitable.add(Box::new(XYrect {
        x0: 0.0,
        x1: 555.0,
        y0: 0.0,
        y1: 555.0,
        k: 555.0,
        material: Box::new(white.clone()),
    }));

    let isoblack = Isotropic {
        albedo: Box::new(ConstantTexture {
            color: V3::new(0.0, 0.0, 0.0),
        }),
    };

    hitable.add(
        Box::new(HitBox::new(
            V3::new(0.0, 0.0, 0.0),
            V3::new(165.0, 330.0, 165.0),
            Box::new(white.clone()),
        ))
        .rotate(V3::new(0.0, 1.0, 0.0), 0.261_799_4)
        .translate(V3::new(265.0, 0.0, 295.0))
        .into_constant_medium(0.01, Box::new(isoblack)),
    );

    let isowhite = Isotropic {
        albedo: Box::new(ConstantTexture {
            color: V3::new(1.0, 1.0, 1.0),
        }),
    };

    hitable.add(
        Box::new(HitBox::new(
            V3::new(0.0, 0.0, 0.0),
            V3::new(165.0, 165.0, 165.0),
            Box::new(white),
        ))
        .rotate(V3::new(0.0, 1.0, 0.0), -0.314_159_27)
        .translate(V3::new(130.0, 0.0, 65.0))
        .into_constant_medium(0.01, Box::new(isowhite)),
    );

    let image_config = ImageConfig {
        nx: 1024,
        ny: 1024,
        samples_per_pixel: 2,
        ray_setting: RaySetting {
            background_color: V3::new(0.0, 0.0, 0.0),
            depth: 64,
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
        world: Box::new(hitable),
    }
    .loop_render(image_config, 12);
}
