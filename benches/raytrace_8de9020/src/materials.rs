use model::Hit;
use rand::{Rng, XorShiftRng};
use vec::{random_in_unit_sphere, Ray, Vec3};

#[derive(Clone, Copy, Debug)]
pub struct Scatter {
    pub color: Vec3,
    pub ray: Option<Ray>,
}

pub trait Material {
    fn scatter(&self, r_in: &Ray, rec: &Hit, rng: &mut XorShiftRng) -> Scatter;
}

pub struct Lambertian {
    pub albedo: Vec3,
}

impl Material for Lambertian {
    fn scatter(&self, _r_in: &Ray, hit: &Hit, rng: &mut XorShiftRng) -> Scatter {
        let target = hit.p + hit.normal + random_in_unit_sphere(rng);
        Scatter {
            color: self.albedo,
            ray: Some(Ray::new(hit.p, target - hit.p)),
        }
    }
}

fn reflect(v: Vec3, n: Vec3) -> Vec3 {
    v - 2.0 * v.dot(n) * n
}

pub struct Metal {
    pub albedo: Vec3,
    pub fuzz: f32,
}

impl Material for Metal {
    fn scatter(&self, r_in: &Ray, hit: &Hit, rng: &mut XorShiftRng) -> Scatter {
        let reflected = reflect(r_in.direction, hit.normal);
        let scattered = Ray::new(hit.p, reflected + self.fuzz * random_in_unit_sphere(rng));

        Scatter {
            color: self.albedo,
            ray: if scattered.direction.dot(hit.normal) <= 0.0 {
                None
            } else {
                Some(scattered)
            },
        }
    }
}

pub struct Dielectric {
    // Technically, this is not the index of refaction but the ratio of the
    // index of refraction inside the material to the index of refraction
    // outside.  But if the material outside is air, its index of refraction is
    // 1 and so it amounts to the same thing.
    pub index: f32,
}

fn refract(v: Vec3, n: Vec3, ni_over_nt: f32) -> Option<Vec3> {
    let uv = v.to_unit_vector();

    let dt = uv.dot(n);
    let discriminant = 1.0 - ni_over_nt * ni_over_nt * (1.0 - dt * dt);
    if discriminant > 0.0 {
        Some(ni_over_nt * (uv - dt * n) - discriminant.sqrt() * n)
    } else {
        None
    }
}

/// Christophe Schlick's approximation for the reflectivity of glass,
/// as a function of the angle of incidence and index of refraction.
fn schlick(cosine: f32, index: f32) -> f32 {
    let r0 = (1.0 - index) / (1.0 + index);
    let r0 = r0 * r0;
    r0 + (1.0 - r0) * (1.0 - cosine).powi(5)
}

const WHITE: Vec3 = Vec3(1.0, 1.0, 1.0);

impl Material for Dielectric {
    fn scatter(&self, r_in: &Ray, hit: &Hit, rng: &mut XorShiftRng) -> Scatter {
        let outward_normal: Vec3;
        let ni_over_nt: f32;
        let cosine: f32;

        if r_in.direction.dot(hit.normal) > 0.0 {
            outward_normal = -hit.normal;
            ni_over_nt = self.index;
            cosine = self.index * r_in.direction.dot(hit.normal) / r_in.direction.length();
        } else {
            outward_normal = hit.normal;
            ni_over_nt = 1.0 / self.index;
            cosine = -r_in.direction.dot(hit.normal) / r_in.direction.length();
        }

        match refract(r_in.direction, outward_normal, ni_over_nt) {
            Some(refracted) => {
                if rng.gen::<f32>() > schlick(cosine, self.index) {
                    return Scatter {
                        color: WHITE,
                        ray: Some(Ray::new(hit.p, refracted)),
                    };
                }
            }
            None => {}
        }

        Scatter {
            color: WHITE,
            ray: Some(Ray::new(hit.p, reflect(r_in.direction, hit.normal))),
        }
    }
}
