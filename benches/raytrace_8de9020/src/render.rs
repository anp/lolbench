use rand::{Rng, XorShiftRng};

use camera::Camera;
use model::Model;
use std::f32::consts::PI;
use vec::{Ray, Vec3};

fn color(mut r: Ray, model: &Model, rng: &mut XorShiftRng) -> Vec3 {
    const WHITE: Vec3 = Vec3(1.0, 1.0, 1.0);
    let sky_blue = 0.3 * Vec3(0.5, 0.7, 1.0) + 0.7 * WHITE;

    let mut attenuation = WHITE;
    let mut depth = 0;
    while let Some(hit) = model.hit(&r) {
        let scattered = hit.material.scatter(&r, &hit, rng);
        attenuation = attenuation * scattered.color;
        if let Some(bounce) = scattered.ray {
            r = bounce;
        } else {
            break;
        }

        depth += 1;
        if depth >= 50 {
            break;
        }
    }
    let sun_direction = Vec3(1.0, 1.0, 1.0).to_unit_vector();
    let unit_direction = r.direction.to_unit_vector();
    if sun_direction.dot(unit_direction) >= (5.0 * PI / 180.0).cos() {
        Vec3(5.0, 5.0, 3.0) * attenuation // SUPER BRIGHT
    } else {
        let t = 0.5 * (unit_direction.y() + 1.0);
        let orig_color = (1.0 - t) * WHITE + t * sky_blue;
        orig_color * attenuation
    }
}

pub(crate) fn render(
    rng: &mut XorShiftRng,
    scene: &Model,
    camera: &Camera,
    width: usize,
    height: usize,
    samples: usize,
) {
    use lolbench_support::black_box;

    let mut pixels = Vec::with_capacity(width * height);
    for y in 0..height {
        let j = height - 1 - y;
        for i in 0..width {
            let mut col = Vec3(0.0, 0.0, 0.0);
            for _ in 0..samples {
                let u = (i as f32 + rng.gen::<f32>()) / width as f32;
                let v = (j as f32 + rng.gen::<f32>()) / height as f32;

                let r = camera.get_ray(u, v, rng);
                col = col + color(r, scene, rng);
            }
            col = col / samples as f32;
            col = Vec3(col.x().sqrt(), col.y().sqrt(), col.z().sqrt());
            let rgb = col.to_u8();
            pixels.push(rgb);
        }
    }
    black_box(pixels);
}
