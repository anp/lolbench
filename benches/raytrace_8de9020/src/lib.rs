// Dependencies
extern crate criterion;
extern crate lodepng;
extern crate rand; // generate random numbers // output PNG image files
#[macro_use]
extern crate wrap_libtest;

// Ray-tracer modules
mod vec; // basic 3D vector math
mod model; // geometry of objects in the scene
mod materials; // reflective properties of surfaces
mod camera; // translate 2D pixel coordinates to 3D rays
mod render; // the core ray-tracing algorithm

// The rest of the code in this file brings the pieces together
// to render a scene made of a bunch of spheres.

use rand::{Rng, SeedableRng, XorShiftRng};
use vec::{random_in_unit_disc, Vec3};
use model::{Model, Sphere};
use materials::{Dielectric, Lambertian, Material, Metal};
use camera::Camera;

/// Generate a Model containing a bunch of randomly placed spheres.
fn random_scene(rng: &mut XorShiftRng) -> Box<Model> {
    let mut spheres: Vec<Sphere> = vec![
        Sphere {
            center: Vec3(0.0, 0.0, -1000.0),
            radius: 1000.0,
            material: Box::new(Lambertian {
                albedo: Vec3(1.0, 0.6, 0.5),
            }),
        },
        Sphere {
            center: Vec3(-4.0, 0.0, 2.0),
            radius: 2.0,
            material: Box::new(Lambertian {
                albedo: Vec3(0.6, 0.2, 0.2),
            }),
        },
        Sphere {
            center: Vec3(0.0, 0.0, 2.0),
            radius: 2.0,
            material: Box::new(Dielectric { index: 1.5 }),
        },
        Sphere {
            center: Vec3(4.0, 0.0, 2.0),
            radius: 2.0,
            material: Box::new(Metal {
                albedo: Vec3(0.85, 0.9, 0.7),
                fuzz: 0.0,
            }),
        },
    ];

    fn random_material(rng: &mut XorShiftRng) -> Box<Material> {
        match (rng.gen::<f32>() * 10.0) as u8 {
            0...7 => Box::new(Lambertian { albedo: rng.gen() }),
            7...9 => Box::new(Metal {
                albedo: Vec3(0.5, 0.5, 0.5) + 0.5 * rng.gen::<Vec3>(),
                fuzz: 0.5 * rng.gen::<f32>(),
            }),
            _ => Box::new(Dielectric { index: 1.5 }),
        }
    }

    for _ in 0..500 {
        let r = 0.4;
        let Vec3(x, y, _) = random_in_unit_disc(rng);
        let pos = 20.0 * Vec3(x, y, 0.0) + Vec3(0.0, 0.0, r);
        if spheres
            .iter()
            .all(|s| (s.center - pos).length() >= s.radius + r)
        {
            spheres.push(Sphere {
                center: pos,
                radius: r,
                material: random_material(rng),
            });
        }
    }

    let world: Vec<Box<Model>> = spheres
        .into_iter()
        .map(|s| Box::new(s) as Box<Model>)
        .collect();
    Box::new(world)
}

wrap_libtest! {
    fn raytrace_random_scenes(b: &mut Bencher) {
        b.iter(|| {
            const WIDTH: usize = 50;
            const HEIGHT: usize = 25;

            const NSAMPLES: usize = 5;

            let mut rng = XorShiftRng::from_seed([22, 0, 22, 0]);

            let scene = random_scene(&mut rng);

            let lookfrom = Vec3(20.0 * 0.47f32.cos(), 20.0 * 0.47f32.sin(), 3.0);
            let lookat = Vec3(0.0, 0.0, 1.0);
            let vup = Vec3(0.0, 0.0, 1.0);
            let focus_distance = (lookfrom - lookat).length();
            let aperture = 0.3;
            let camera = Camera::new(
                lookfrom,
                lookat,
                vup,
                20.0,
                WIDTH as f32 / HEIGHT as f32,
                aperture,
                focus_distance,
            );

            render::render(&mut rng, &*scene, &camera, WIDTH, HEIGHT, NSAMPLES);
        });
    }
}
