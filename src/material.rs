extern crate rand;

use std::fmt;
use rand::distributions::{Distribution, Uniform};

extern crate cgmath;
use cgmath::InnerSpace;

use crate::color::Color;
use crate::geo::{Point, Ray, Vector};

pub trait Material: fmt::Debug {
    fn scatter(&self, ray: &Ray, point: &Point, normal: &Vector) -> (Color, Ray, f32);
    fn emitted(&self, uvw: &Vector, ray: &Vector) -> Color;
}

#[derive(Debug)]
pub struct Lambertian {
    pub albedo: Color,
}

impl Material for Lambertian {
    fn scatter(&self, _ray: &Ray, point: &Point, normal: &Vector) -> (Color, Ray, f32) {
        let target = point + normal + random_in_unit_sphere();
        let direction = (target - point).normalize();
        (self.albedo, Ray {point: *point, direction}, normal.dot(direction) / std::f32::consts::PI)
    }

    fn emitted(&self, _uvw: &Vector, _ray: &Vector) -> Color {
        Color::black()
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

#[derive(Debug)]
pub struct Metal {
    pub albedo: Color,
    pub shinyness: f32,
}

impl Material for Metal {
    fn scatter(&self, ray: &Ray, point: &Point, normal: &Vector) -> (Color, Ray, f32) {
        let dir = reflect(&ray.direction, normal);
        (self.albedo, Ray {point: *point, direction: dir}, normal.dot(dir) / std::f32::consts::PI)
    }

    fn emitted(&self, _uvw: &Vector, _ray: &Vector) -> Color {
        Color::black()
    }
}

#[derive(Debug)]
pub struct DiffuseLight {
    pub color: Color,
}

impl Material for DiffuseLight {
    fn scatter(&self, ray: &Ray, point: &Point, normal: &Vector) -> (Color, Ray, f32) {
        let dir = reflect(&ray.direction, normal);
        (Color::black(), Ray {point: *point, direction: dir}, normal.dot(dir) / std::f32::consts::PI)
    }

    fn emitted(&self, _uvw: &Vector, _ray: &Vector) -> Color {
        self.color
    }
}
