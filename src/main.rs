extern crate cgmath;

use std::path::Path;
use std::fs::File;
use std::io::BufWriter;

use png::HasParameters;

use cgmath::prelude::*;
use cgmath::{Deg, Rad};

mod geo;
mod color;
mod material;

use crate::color::Color;
use crate::geo::{Point, Sphere, Plane, Ray, Vector, Intersectable};
use crate::material::Lambertian;

fn main() {
    // let lights = vec![Light {position: Point::new(0.0, 100.0, 0.0) }];
    let material_a: Lambertian = Lambertian { albedo: Color::new(1.0, 1.0, 0.5, 1.0) };
    let material_b: Lambertian = Lambertian { albedo: Color::new(1.0, 0.0, 0.0, 1.0) };
    let sphere1: Sphere = Sphere {position: Point::new(0.0, 10.0, -100.0), radius: 20.0, material: &material_a};
    let plane1 = Plane {point: Point::new(0.0, -10.0, 0.0), normal: Vector::new(0.0, 1.0, 0.0), material: &material_b};
    let objects: Vec<&Intersectable> = vec![&sphere1];
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
            for x in 0 .. image_width {
                let f_x = x as f32;
                let pixel_index = (y * image_width + x) as usize;
                let xx = (2.0 * ((f_x + 0.5) * inverse_width) - 1.0) * angle * aspect_ratio;
                let yy = (1.0 - 2.0 * ((f_y + 0.5) * inverse_height)) * angle;
                let primary_ray = Ray { point: Point::origin(), direction: Vector::new(xx, yy, -1.0).normalize() };
                image[pixel_index] = calc(&primary_ray, 0.0, std::f32::MAX, &objects, 0).saturate();
            }
        }
        write_png(image_width, image_height, &convert_to_rgb8(&image));
    }
}

fn calc(ray: &Ray, t_min: f32, t_max: f32, objects: &Vec<&Intersectable>, depth: u8) -> Color {
    if let Some((obj, point, normal)) = intersect_objects(ray, t_min, t_max, objects) {
        let (attenuation, scatter) = obj.color(ray, &point, &normal);

        if depth < 4 {
            attenuation * 0.9 * calc(&scatter, 0.001, 1.0, objects, depth + 1).saturate()
        } else {
            Color::black()
        }
        // Color {r: (normal.x),
        //        g: (normal.y),
        //        b: (normal.z),
        //        a: 1.0}
    } else {
        let t = 0.5 * (ray.direction.y + 1.0);
        Color::white() * t + Color::black() * (1.0 - t)
    }
}

fn gamma2(color: &Color) -> (u8, u8, u8) {
    ((color.r.sqrt() * 255.99) as u8,
     (color.g.sqrt() * 255.99)as u8,
     (color.b.sqrt() * 255.99) as u8)
}

fn convert_to_rgb8(data: &Vec<Color>) -> Vec<u8> {
    let mut out = Vec::with_capacity(data.len() * 4);
    for c in data {
        let (r, g, b) = gamma2(c);
        out.push(r);
        out.push(g);
        out.push(b);
    }
    out
}

/// Returns object, point, and normal of the closest intersection of ray with objects, or None
fn intersect_objects<'a>(ray: &Ray, t_min: f32, t_max: f32, objects: &'a Vec<&'a Intersectable>) -> Option<(&'a Intersectable, Point, Vector)> {
    let mut closest = None;
    let mut closest_distance_squared = std::f32::MAX;
    for obj in objects {
        if let Some((point, normal)) = obj.intersect(ray, t_min, t_max) {
            let distance_squared = (point - ray.point).magnitude2();
            if distance_squared < closest_distance_squared {
                closest = Some((*obj, point, normal));
                closest_distance_squared = distance_squared;
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
