extern crate cgmath;

use cgmath::prelude::*;
use cgmath::{Point3, Vector3};

use crate::color::Color;
use crate::material::Material;

pub type Point = Point3<f32>;
pub type Vector = Vector3<f32>;

#[derive(Debug)]
pub struct Ray {
    pub point: Point,
    pub direction: Vector
}

pub trait Intersectable {
    /// Returns point and normal of the intersection with ray, or None
    fn intersect(&self, ray: &Ray, t_min: f32, t_max: f32) -> Option<(Point, Vector)>;
    fn color(&self, ray: &Ray, point: &Point, normal: &Vector) -> (Color, Ray);
}

pub struct Sphere<'a> {
    pub position: Point,
    pub radius: f32,
    pub material: &'a Material,
}

impl<'a> Intersectable for Sphere<'a> {
    fn intersect(&self, ray: &Ray, t_min: f32, t_max: f32) -> Option<(Point, Vector)> {
        let oc = ray.point - self.position;
        let a = ray.direction.dot(ray.direction);
        let b = 2.0 * oc.dot(ray.direction);
        let c = oc.dot(oc) - self.radius * self.radius;
        let discriminant = b * b - 4.0 * a * c;
        if discriminant > 0.0 {
            let discriminant_sqrt = discriminant.sqrt();
            let t1 = (-b - discriminant_sqrt) / a;
            if t1 <= t_max && t1 >= t_min {
                let hit_point = ray.point + (ray.direction * t1);
                Some((hit_point, (hit_point - self.position).normalize()))
            } else {
                let t2 = (-b + discriminant_sqrt) / a;
                if t2 <= t_max && t2 >= t_min {
                    let hit_point = ray.point + (ray.direction * t2);
                    Some((hit_point, (hit_point - self.position).normalize()))
                } else {
                    None
                }
            }
        } else {
            None
        }
    }

    fn color(&self, ray: &Ray, point: &Point, normal: &Vector) -> (Color, Ray) {
        self.material.scatter(ray, point, normal)
    }
}

pub struct Plane<'a> {
    pub point: Point,
    pub normal: Vector,
    pub material: &'a Material
}

impl<'a> Intersectable for Plane<'a> {
    fn intersect(&self, ray: &Ray, t_min: f32, t_max: f32) -> Option<(Point, Vector)> {
        let denominator = self.normal.dot(ray.direction);
        if denominator < 0.0 {
            let t = (self.point - ray.point).dot(self.normal) / denominator;
            if t <= t_max && t >= t_min {
                Some((ray.point + ray.direction * t, self.normal))
            } else {
                None
            }
        } else {
            None
        }
    }

    fn color(&self, ray: &Ray, point: &Point, normal: &Vector) -> (Color, Ray) {
        self.material.scatter(ray, point, normal)
    }
}
