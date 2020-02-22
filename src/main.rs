extern crate rand;
use rand::distributions::{Distribution, Uniform};

use std::env;
use std::path::Path;
use std::fs::File;
use std::io::BufWriter;

use png::HasParameters;

extern crate cgmath;
use cgmath::prelude::*;
use cgmath::Deg;

mod geo;
mod color;
mod material;
mod camera;

use crate::camera::Camera;
use crate::color::Color;
use crate::geo::{Point, Sphere, Plane, Ray, Vector, Intersectable};
use crate::material::{Lambertian, Metal, DiffuseLight};

fn main() {
    let args: Vec<String> = env::args().collect();

    // let lights = vec![Light {position: Point::new(0.0, 100.0, 0.0) }];
    let material_a: Metal = Metal { albedo: Color::new(0.7, 0.7, 0.7, 1.0), shinyness: 0.5};
    let material_b: Lambertian = Lambertian { albedo: Color::new(0.0, 0.5, 0.0, 1.0) };
    let material_c: Lambertian = Lambertian { albedo: Color::new(0.5, 0.0, 0.0, 1.0) };
    let material_d: Metal = Metal { albedo: Color::new(0.0, 0.0, 0.5, 1.0), shinyness: 0.5 };
    let diffuse_light: DiffuseLight = DiffuseLight { color: Color::new(0.5, 0.5, 0.5, 1.0) };
    let sphere1: Sphere = Sphere {position: Point::new(0.0, 20.0, 0.0), radius: 20.0, material: &material_a};
    let sphere2: Sphere = Sphere {position: Point::new(5.0, 5.0, 25.0), radius: 5.0, material: &material_c};
    let sphere3: Sphere = Sphere {position: Point::new(25.0, 5.0, 5.0), radius: 5.0, material: &diffuse_light};
    let plane1 = Plane {point: Point::new(0.0, 0.0, 0.0), normal: Vector::unit_y(), material: &material_b};
    let objects: Vec<&dyn Intersectable> = vec![&sphere1, &sphere2, &sphere3, &plane1];
    let image_width = 640u32;
    let image_height = 480u32;
    let sample_count: i32 = args[1].parse().unwrap();
    let name = &args[2];
    let f_image_width = image_width as f32;
    let f_image_height = image_height as f32;

    let inverse_width = 1.0 / f_image_width;
    let inverse_height = 1.0 / f_image_height;
    let aspect_ratio = f_image_width * inverse_height;
    let cam = Camera::look(Point::new(0.0, 25.0, 75.0), Point::new(0.0, 10.0, 20.0), -Vector::unit_y(), Deg(45.0), aspect_ratio);

    let between = Uniform::from(0.0..1.0);
    let mut rng = rand::thread_rng();

    for obj in &objects {
        println!("{:?}", obj);
    }

    if let Some(size) = (image_width as usize).checked_mul(image_height as usize) {
        println!("Size is {:?}", size);
        let mut image: Vec<Color> = vec![Color::black(); size];
        for y in 0 .. image_height {
            let f_y = y as f32;
            for x in 0 .. image_width {
                let f_x = x as f32;
                let pixel_index = (y * image_width + x) as usize;
                let mut color = Color::black();
                for _s in 0 .. sample_count {
                    let u = (f_x + between.sample(&mut rng)) * inverse_width;
                    let v = (f_y + between.sample(&mut rng)) * inverse_height;
                    let ray = cam.get_ray(u, v);
                    color += calc(&ray, 0.0, std::f32::MAX, &objects, 0).saturate();
                }
                image[pixel_index] = color / sample_count as f32;
            }
        }
        write_png(image_width, image_height, &convert_to_rgb8(&image), name);
    }
}

fn calc(ray: &Ray, t_min: f32, t_max: f32, objects: &Vec<&dyn Intersectable>, depth: u8) -> Color {
    if let Some((obj, point, normal, uvw)) = intersect_objects(ray, t_min, t_max, objects) {
        let mat = obj.material();
        let emitted = mat.emitted(&uvw, &normal);
        let (albedo, scatter, _pdf) = mat.scatter(ray, &point, &normal);

        // Debug depth
        //Color::grey(1.0 - (point - ray.point).magnitude().log(1000.0)).saturate()
        // Debug normal
        //Color::new(normal.x, normal.y, normal.z, 1.0).saturate()

        if depth < 10 {
            emitted + albedo * calc(&scatter, 0.001, std::f32::MAX, objects, depth + 1)
        } else {
            emitted + albedo
        }
    } else {
        sky(ray)
    }
}

// fn sunset(ray: &Ray) -> Color {
//     let t = 0.5 * (ray.direction.normalize().y + 1.0);
//     Color::black() * t + Color::new(0.7, 0.2, 0.0, 1.0) * (1.0 - t)
// }

fn sky(ray: &Ray) -> Color {
    let t = 0.5 * (ray.direction.normalize().y + 1.0);
    Color::new(0.02, 0.02, 0.1, 1.0) * t + Color::new(0.1, 0.1, 0.1, 1.0) * (1.0 - t)
}

fn gamma2(color: &Color) -> (u8, u8, u8) {
    ((color.r.sqrt() * 255.99) as u8,
     (color.g.sqrt() * 255.99) as u8,
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
fn intersect_objects<'a>(ray: &Ray, t_min: f32, t_max: f32, objects: &'a Vec<&'a dyn Intersectable>) -> Option<(&'a dyn Intersectable, Point, Vector, Vector)> {
    let mut closest = None;
    let mut closest_distance_squared = std::f32::MAX;
    for obj in objects {
        if let Some((point, normal, uvw)) = obj.intersect(ray, t_min, t_max) {
            let distance_squared = (point - ray.point).magnitude2();
            if distance_squared < closest_distance_squared {
                closest = Some((*obj, point, normal, uvw));
                closest_distance_squared = distance_squared;
            }
        }
    }
    closest
}

fn write_png(width: u32, height: u32, img: &Vec<u8>, name: &String) {
    let path = Path::new(name);
    let file = File::create(path).unwrap();
    let ref mut w = BufWriter::new(file);

    let mut encoder = png::Encoder::new(w, width.into(), height.into());
    encoder.set(png::ColorType::RGB).set(png::BitDepth::Eight);
    let mut writer = encoder.write_header().unwrap();

    writer.write_image_data(img.as_slice()).unwrap();
}
