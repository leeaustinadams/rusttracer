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

    pub fn rgb(self) -> [u8; 3] {
        [255, 255, 255]
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
    pos: Point,
}

#[derive(Debug)]
struct Object {
    id: i32
}

fn main() {
    let lights = vec![Light {pos: Point::new(0.0, 100.0, 0.0) }];
    let objects = vec![Object {id: 0}];
    let image_height = 480u16;
    let image_width = 640u16;

    let inverse_width = 1.0 / f32::from(image_width);
    let inverse_height = 1.0 / f32::from(image_height);
    let fov = Deg(0.5 * 30.0);
    let aspect_ratio = f32::from(image_width) * inverse_height;
    let angle = Rad::from(fov).tan();

    if let Some(size) = usize::from(image_width).checked_mul(usize::from(image_height)) {
        let mut image: Vec<Color> = vec![Color::black(); size];
        for y in 0 .. image_height {
            for x in 0 .. image_width {
//                let primary_ray = calculate_primary_ray(x, y);
                let xx = (2.0 * ((f32::from(x) + 0.5) * inverse_width) - 1.0) * angle * aspect_ratio;
                let yy = (1.0 - 2.0 * ((f32::from(y) + 0.5) * inverse_height)) * angle;
                let primary_ray = Ray { point: Point::origin(), direction: Vector::new(xx, yy, -1.0).normalize() };

                if let Some((point, normal)) = intersect_objects(&objects, &primary_ray) {
                    // hit
                    let direction = &lights[0].pos - &point;
                    let shadow_ray = Ray::new(point, direction);

                    if let Some((shadow_point, shadow_normal)) = intersect_objects(&objects, &shadow_ray) {

                    } else {
                        image[usize::from(y * image_width + x)] = Color::white();
                    }
                }
            }
        }
        write_png(image_width, image_height, &convert_to_rgb8(&image));
    }
}

fn convert_to_rgb8(data: &Vec<Color>) -> Vec<u8> {
    let mut out = Vec::with_capacity(data.len() * 4);
    for c in data {
        out.push(255);
        out.push(255);
        out.push(255);
    }
    out
}

fn calculate_primary_ray(_x: u16, _y: u16) -> Ray {
    Ray { point: Point::origin(), direction: Vector::new(0.0, 0.0, 1.0) }
}

fn intersect_objects(_objects: &Vec<Object>, _ray: &Ray) -> Option<(Point, Vector)> {
    None
}

fn write_png(width: u16, height: u16, img: &Vec<u8>) {
    let path = Path::new(r"image.png");
    let file = File::create(path).unwrap();
    let ref mut w = BufWriter::new(file);

    let mut encoder = png::Encoder::new(w, width.into(), height.into());
    encoder.set(png::ColorType::RGB).set(png::BitDepth::Eight);
    let mut writer = encoder.write_header().unwrap();

    writer.write_image_data(img.as_slice()).unwrap();
}
