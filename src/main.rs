extern crate cgmath;

use std::path::Path;
use std::fs::File;
use std::io::BufWriter;

use png::EncodingError;
use png::HasParameters;

use cgmath::prelude::*;
use cgmath::{Deg, Point3, Rad, Vector3};

type Point = Point3<f32>;
type Vector = Vector3<f32>;

#[derive(Debug, Copy, Clone)]
struct Color {
    r: f32,
    g: f32,
    b: f32,
    a: f32
}

impl Color {
    pub fn black() -> Color {
        Color { r: 0.0, g: 0.0, b: 0.0, a: 1.0 }
    }
    pub fn white() -> Color {
        Color { r: 1.0, g: 1.0, b: 1.0, a: 1.0 }
    }

    pub fn new(r: f32, g: f32, b: f32, a: f32) -> Color {
        Color { r, g, b, a }
    }

    pub fn rgb(self) -> [u8; 3] {
        [(self.r * 255.0) as u8, (self.g * 255.0) as u8, (self.b * 255.0) as u8]
    }
}

#[derive(Debug)]
struct Ray {
    point: Point,
    direction: Vector
}

impl Ray {
    pub fn new(point: Point, direction: Vector) -> Ray {
        Ray { point, direction }
    }
}

#[derive(Debug)]
struct Light {
    position: Point,
}

#[derive(Debug)]
struct Object {
    id: i32,
    position: Point,
    radius: f32,
}

fn main() {
    let lights = vec![Light {position: Point::new(0.0, 100.0, 0.0) }];
    let objects = vec![Object {id: 0, position: Point::new(0.0, 0.0, -100.0), radius: 20.0}];
    let image_width = 640u32;
    let image_height = 480u32;
    let f_image_width = image_width as f32;
    let f_image_height = image_height as f32;

    let inverse_width = 1.0 / f_image_width;
    let inverse_height = 1.0 / f_image_height;
    let fov = Deg(0.5 * 30.0);
    let aspect_ratio = f_image_width * inverse_height;
    let angle = Rad::from(fov).tan();

    if let Some(size) = (image_width as usize).checked_mul(image_height as usize) {
        println!("Size is {:?}", size);
        let mut image: Vec<Color> = vec![Color::black(); size];
        for y in 0 .. image_height {
            let f_y = y as f32;
            let debug_g = f_y / image_height as f32;
            for x in 0 .. image_width {
                let f_x = x as f32;
                let pixel_index = (y * image_width + x) as usize;
//                let primary_ray = calculate_primary_ray(x, y);
                let xx = (2.0 * ((f_x + 0.5) * inverse_width) - 1.0) * angle * aspect_ratio;
                let yy = (1.0 - 2.0 * ((f_y + 0.5) * inverse_height)) * angle;
                let primary_ray = Ray { point: Point::origin(), direction: Vector::new(xx, yy, -1.0).normalize() };

                if let Some((obj, point, normal)) = intersect_objects(&primary_ray, &objects) {
                    // hit
                    let direction = (&lights[0].position - &point).normalize();
//                    let shadow_ray = Ray::new(point, direction);

//                    if let Some((shadow_obj, shadow_point, shadow_normal)) = intersect_objects(&shadow_ray, &objects) {
//                    } else {
                        image[pixel_index].r = (normal.x + 1.0) * 0.5;
                        image[pixel_index].g = (normal.y + 1.0) * 0.5;
                        image[pixel_index].b = (normal.z + 1.0) * 0.5;
//                    }
                } else {
                    let debug_r = f_x / f_image_width;
                    image[pixel_index].r = debug_r;
                    image[pixel_index].g = debug_g;
                }
            }
        }
        write_png(image_width, image_height, &convert_to_rgb8(&image));
    }
}

fn convert_to_rgb8(data: &Vec<Color>) -> Vec<u8> {
    let mut out = Vec::with_capacity(data.len() * 4);
    for c in data {
        out.push((c.r * 255.0) as u8);
        out.push((c.g * 255.0) as u8);
        out.push((c.b * 255.0) as u8);
    }
    out
}

fn calculate_primary_ray(_x: u16, _y: u16) -> Ray {
    Ray { point: Point::origin(), direction: Vector::new(0.0, 0.0, 1.0) }
}

/// Returns point and normal of the intersection of ray with sphere, or None
fn intersect_sphere(ray: &Ray, sphere: &Object) -> Option<(Point, Vector)> {
    let oc = ray.point - sphere.position;
    let a = ray.direction.dot(ray.direction);
    let b = 2.0 * oc.dot(ray.direction);
    let c = oc.dot(oc) - sphere.radius * sphere.radius;
    let discriminant = b * b - 4.0 * a * c;
    if discriminant > 0.0 {
        let t = (-b - discriminant.sqrt()) / (2.0 * a);
        let hit_point = ray.point + (ray.direction * t);
        Some((hit_point, (hit_point - sphere.position).normalize()))
    } else {
        None
    }
}

/// Returns object, point, and normal of the closest intersection of ray with objects, or None
fn intersect_objects<'a>(ray: &Ray, objects: &'a Vec<Object>) -> Option<(&'a Object, Point, Vector)> {
    let mut closest = None;
    for obj in objects {
        if let Some(i) = intersect_sphere(ray, obj) {
            if let Some(c) = closest {
            } else {
                closest = Some((obj, i.0, i.1));
            }
        }
    }
    closest
}

fn write_png(width: u32, height: u32, img: &Vec<u8>) {
    let path = Path::new(r"image.png");
    let file = File::create(path).unwrap();
    let ref mut w = BufWriter::new(file);

    let mut encoder = png::Encoder::new(w, width.into(), height.into());
    encoder.set(png::ColorType::RGB).set(png::BitDepth::Eight);
    let mut writer = encoder.write_header().unwrap();

    writer.write_image_data(img.as_slice()).unwrap();
}
