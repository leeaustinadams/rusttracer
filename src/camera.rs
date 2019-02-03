extern crate cgmath;
use cgmath::{Angle, InnerSpace, Deg, Rad};

use crate::geo::{Point, Ray, Vector};

pub struct Camera {
    origin: Point,
    top_left_corner: Point,
    horizontal: Vector,
    vertical: Vector
}

impl Camera {
    pub fn look(from: Point, to: Point, up: Vector, fov: Deg<f32>, aspect_ratio: f32) -> Camera {
        let half_height = (Rad::from(fov) / 2.0).tan();
        let half_width = aspect_ratio * half_height;
        let w = (from - to).normalize();
        let u = up.cross(w).normalize();
        let v = w.cross(u);
        Camera { origin: from,
                 top_left_corner: from - half_width * u - half_height * v - w,
                 horizontal: 2.0 * half_width * u,
                 vertical: 2.0 * half_height * v }
    }

    pub fn get_ray(&self, u: f32, v: f32) -> Ray {
        Ray { point: self.origin,
              direction: self.top_left_corner + u * self.horizontal + v * self.vertical - self.origin }
    }
}
