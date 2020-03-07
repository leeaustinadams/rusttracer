extern crate cgmath;

use std::fmt;

use cgmath::prelude::*;
use cgmath::{Point3, Vector3};

use crate::material::Material;

pub type Point = Point3<f32>;
pub type Vector = Vector3<f32>;

#[derive(Debug)]
pub struct Ray {
    pub point: Point,
    pub direction: Vector
}

pub trait Intersectable: fmt::Debug {
    /// Returns point and normal of the intersection with ray, or None
    fn intersect(&self, ray: &Ray, t_min: f32, t_max: f32) -> Option<(Point, Vector, Vector)>;

    fn material(&self) -> &dyn Material;
}

#[derive(Debug)]
pub struct Sphere<'a> {
    pub position: Point,
    pub radius: f32,
    pub material: &'a dyn Material,
}

fn sphere_uvw(p: &Point) -> Vector {
    let phi = p.z.atan2(p.x);
    let theta = p.y.asin();
    Vector::new(1.0 - (phi + std::f32::consts::PI) / (2.0 * std::f32::consts::PI),
                (theta + std::f32::consts::PI / 2.0) / std::f32::consts::PI,
                0.0)
}

impl<'a> Intersectable for Sphere<'a> {
    fn intersect(&self, ray: &Ray, t_min: f32, t_max: f32) -> Option<(Point, Vector, Vector)> {
        let oc = ray.point - self.position;
        let a = ray.direction.dot(ray.direction);
        let b = oc.dot(ray.direction);
        let c = oc.dot(oc) - self.radius * self.radius;
        let discriminant = b * b - a * c;
        if discriminant > 0.0 {
            let discriminant_sqrt = discriminant.sqrt();
            let t1 = (-b - discriminant_sqrt) / a;
            if t1 <= t_max && t1 >= t_min {
                let hit_point = ray.point + (ray.direction * t1);
                let uvw = sphere_uvw(&hit_point);
                Some((hit_point, (hit_point - self.position).normalize(), uvw))
            } else {
                let t2 = (-b + discriminant_sqrt) / a;
                if t2 <= t_max && t2 >= t_min {
                    let hit_point = ray.point + (ray.direction * t2);
                    let uvw = sphere_uvw(&hit_point);
                    Some((hit_point, (hit_point - self.position).normalize(), uvw))
                } else {
                    None
                }
            }
        } else {
            None
        }
    }

    fn material(&self) -> &dyn Material {
        self.material
    }
}

#[derive(Debug)]
pub struct Plane<'a> {
    pub point: Point,
    pub normal: Vector,
    pub material: &'a dyn Material
}

impl<'a> Intersectable for Plane<'a> {
    fn intersect(&self, ray: &Ray, t_min: f32, t_max: f32) -> Option<(Point, Vector, Vector)> {
        let denominator = self.normal.dot(ray.direction);
        if denominator < 0.0 {
            let t = (self.point - ray.point).dot(self.normal) / denominator;
            if t <= t_max && t >= t_min {
                Some((ray.point + ray.direction * t, self.normal, Vector::new(0.0, 0.0, 0.0)))
            } else {
                None
            }
        } else {
            None
        }
    }

    fn material(&self) -> &dyn Material {
        self.material
    }
}

#[derive(Debug)]
pub struct Triangle<'a> {
    pub a: Point,
    pub b: Point,
    pub c: Point,
    pub ab: Vector,
    pub ac: Vector,
    pub bc: Vector,
    pub normal: Vector,
    pub material: &'a dyn Material
}

// Counter clockwise winding order
impl<'a> Triangle<'a> {
    pub fn new(a: Point, b: Point, c: Point, material: &dyn Material) -> Triangle {
        let ab = b - a;
        let ac = c - a;
        let bc = c - b;
        let normal = (ab.cross(ac)).normalize();
        Triangle { a, b, c, ab: ab, ac: ac, bc: bc, normal, material }
    }
}

impl<'a> Intersectable for Triangle<'a> {
    fn intersect(&self, ray: &Ray, t_min: f32, t_max: f32) -> Option<(Point, Vector, Vector)> {
        let denominator = self.normal.dot(ray.direction);
        if denominator < 0.0 {
            let t = (self.a - ray.point).dot(self.normal) / denominator;
            if t <= t_max && t >= t_min {
                let point_on_plane = ray.point + ray.direction * t;
                let ap = point_on_plane - self.a;
                let bp = point_on_plane - self.b;
                let ab_dot = ap.dot(self.ab.normalize());
                let ac_dot = ap.dot(self.ac.normalize());
                let bc_dot = bp.dot(self.bc.normalize());
                if ab_dot >= 0.0 && ab_dot  <= self.ab.magnitude() &&
                    ac_dot >= 0.0 && ac_dot <= self.ac.magnitude() &&
                    bc_dot >= 0.0 && bc_dot <= self.bc.magnitude() {
                    Some((point_on_plane, self.normal, Vector::new(0.0, 0.0, 0.0)))
                } else {
                    None
                }
            } else {
                None
            }
        } else {
            None
        }
    }

    fn material(&self) -> &dyn Material {
        self.material
    }
}
