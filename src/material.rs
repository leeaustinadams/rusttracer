extern crate rand;

use std::fmt;
use rand::prelude::*;
use rand::distributions::{Distribution, Uniform};

extern crate cgmath;
use cgmath::InnerSpace;

use crate::color::Color;
use crate::geo::{Point, Ray, Vector};

pub trait Material: fmt::Debug {
    fn scatter(&self, ray: &Ray, point: &Point, normal: &Vector) -> (Color, Ray, f32);

    fn emitted(&self, _uvw: &Vector, _ray: &Vector) -> Color {
        Color::grey(0.0)
    }
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

fn refract(v: &Vector, normal: &Vector, ni_over_nt: f32) -> Option<Vector> {
    let unit_v = v.normalize();
    let dt = unit_v.dot(*normal);
    let discriminant = 1.0 - ni_over_nt * ni_over_nt * (1.0 - dt * dt);
    if discriminant > 0.0 {
        Some(ni_over_nt * (unit_v - normal * dt) - normal * discriminant.sqrt())
    } else {
        None
    }
}

fn schlick(cosine: f32, refractive_index: f32) -> f32 {
    let r0 = (1.0 - refractive_index) / (1.0 + refractive_index);
    let r1 = r0 * r0;
    return r1 + (1.0 - r1) * (1.0 - cosine).powf(5.0)
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
}

#[derive(Debug)]
pub struct Dialectric {
    pub refractive_index: f32,
}

impl Material for Dialectric {
    fn scatter(&self, ray: &Ray, point: &Point, normal: &Vector) -> (Color, Ray, f32) {
        let outward_normal: Vector;
        let ni_over_nt: f32;
        let cosine: f32;

        if ray.direction.dot(*normal) > 0.0 {
            outward_normal = -(*normal);
            ni_over_nt = self.refractive_index;
            cosine = self.refractive_index * ray.direction.dot(*normal) / ray.direction.magnitude();
        } else {
            outward_normal = *normal;
            ni_over_nt = 1.0 / self.refractive_index;
            cosine = -(ray.direction.dot(*normal)) / ray.direction.magnitude();
        }

        let mut dir = reflect(&ray.direction, normal);
        if let Some(refracted) = refract(&ray.direction, &outward_normal, ni_over_nt) {
            let reflect_probability = schlick(cosine, self.refractive_index);
            let mut rng = rand::thread_rng();
            if rng.gen::<f32>() > reflect_probability {
                dir = refracted;
            }
        }

        (Color::grey(1.0), Ray{point: *point, direction: dir}, 1.0)
    }
}

#[derive(Debug)]
pub struct DiffuseLight {
    pub color: Color,
}

impl Material for DiffuseLight {
    fn scatter(&self, ray: &Ray, point: &Point, normal: &Vector) -> (Color, Ray, f32) {
        let dir = reflect(&ray.direction, normal);
        (Color::grey(0.0), Ray {point: *point, direction: dir}, normal.dot(dir) / std::f32::consts::PI)
    }

    fn emitted(&self, _uvw: &Vector, _ray: &Vector) -> Color {
        self.color
    }
}

#[derive(Debug)]
pub struct DebugDepth {}

impl Material for DebugDepth {
    fn scatter(&self, ray: &Ray, point: &Point, normal: &Vector) -> (Color, Ray, f32) {
        let value = 1.0 - (point - ray.point).magnitude().log(1000.0);
        (Color::grey(value).saturate(),
         Ray {point: *point, direction: *normal},
         0.0)
    }
}

#[derive(Debug)]
pub struct DebugNormal {}

impl Material for DebugNormal {
    fn scatter(&self, _ray: &Ray, point: &Point, normal: &Vector) -> (Color, Ray, f32) {
        (Color::new(normal.x, normal.y, normal.z, 1.0).saturate(),
         Ray {point: *point, direction: *normal},
         0.0)
    }
}
