extern crate rand;
use rand::distributions::{Distribution, Uniform};

extern crate cgmath;
use cgmath::InnerSpace;

use crate::color::Color;
use crate::geo::{Point, Ray, Vector};

pub trait Material {
    fn scatter(&self, ray: &Ray, point: &Point, normal: &Vector) -> (Color, Ray);
}

pub struct Lambertian {
    pub albedo: Color,
}

impl Material for Lambertian {
    fn scatter(&self, ray: &Ray, point: &Point, normal: &Vector) -> (Color, Ray) {
        let target = point + normal + random_in_unit_sphere();
        (self.albedo, Ray {point: *point, direction: target - point})
    }
}

fn random_in_unit_sphere() -> Vector {
    let between = Uniform::from(-1.0..1.0);
    let mut rng = rand::thread_rng();

    loop {
        let v = Vector{x: between.sample(&mut rng),
                       y: between.sample(&mut rng),
                       z: between.sample(&mut rng)};
        if v.magnitude2() <= 1.0 {
            break v;
        }
    }
}

fn reflect(v: &Vector, normal: &Vector) -> Vector {
    v - 2.0 * v.dot(*normal) * normal
}
