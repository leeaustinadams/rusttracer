extern crate rand;
use rand::distributions::{Distribution, Uniform};

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
use crate::geo::{Point, Sphere, Plane, Ray, Vector, Triangle, Intersectable};
use crate::material::{Lambertian, Metal, Dialectric, DiffuseLight};

#[derive(Debug)]
enum ParseError {
    TooFewArgs,
    InvalidInteger(String),
}

impl std::fmt::Display for ParseError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ParseError::TooFewArgs => write!(f, "Too Few Args"),
            ParseError::InvalidInteger(s) => write!(f, "Not an integer: {}", s)
        }
    }
}

struct ParseArgs(std::env::Args);

impl ParseArgs {
    fn new() -> ParseArgs {
        ParseArgs(std::env::args())
    }

    fn require_arg(&mut self) -> Result<String, ParseError> {
        match self.0.next() {
            Some(s) => Ok(s),
            None => Err(ParseError::TooFewArgs)
        }
    }
}

fn parse_u32(s: String) -> Result<u32, ParseError> {
    match s.parse() {
        Ok(val) => Ok(val),
        Err(_) => Err(ParseError::InvalidInteger(s))
    }
}

struct Options {
    pub width: u32,
    pub height: u32,
    pub sample_count: u32,
    pub output_name: String
}

fn parse_options() -> Result<Options, ParseError> {
    let mut parse_args = ParseArgs::new();

    parse_args.require_arg()?;
    let width_str = parse_args.require_arg()?;
    let width = parse_u32(width_str)?;
    let height_str = parse_args.require_arg()?;
    let height = parse_u32(height_str)?;
    let sample_count_str = parse_args.require_arg()?;
    let sample_count = parse_u32(sample_count_str)?;
    let output_name = parse_args.require_arg()?;
    Ok(Options { width, height, sample_count, output_name })
}

fn main() {
    let options = match parse_options() {
        Ok(args) => args,
        Err(err) => {
            println!("{}", err);
            return;
        }
    };

    let chrome = Metal { albedo: Color::new(0.7, 0.7, 0.7, 1.0), shinyness: 0.5};
    let green = Lambertian { albedo: Color::new(0.0, 0.5, 0.0, 1.0) };
    let red = Lambertian { albedo: Color::new(0.5, 0.0, 0.0, 1.0) };
    let blue_metal = Metal { albedo: Color::new(0.0, 0.0, 0.5, 1.0), shinyness: 0.5 };
    let glass = Dialectric { refractive_index: 1.1 };
    let diffuse_light = DiffuseLight { color: Color::new(0.5, 0.5, 0.5, 1.0) };
    let light = Sphere {position: Point::new(0.0, 5.0, 25.0), radius: 5.0, material: &diffuse_light};
    let red_sphere = Sphere {position: Point::new(-10.0, 5.0, 25.0), radius: 5.0, material: &red};
    let blue_sphere = Sphere {position: Point::new(10.0, 5.0, 25.0), radius: 5.0, material: &blue_metal};
    let glass_sphere = Sphere {position: Point::new(5.0, 5.0, 35.0), radius: 5.0, material: &glass};
    let ground = Plane {point: Point::new(0.0, 0.0, 0.0), normal: Vector::unit_y(), material: &green};
    let mirror1 = Triangle::new(Point::new(-20.0, 0.0, 0.0), Point::new(20.0, 0.0, 0.0), Point::new(-20.0, 40.0, 0.0), &chrome);
    let mirror2 = Triangle::new(Point::new(-20.0, 40.0, 0.0), Point::new(20.0, 0.0, 0.0), Point::new(2.0, 40.0, 0.0), &chrome);
    let objects: Vec<&dyn Intersectable> = vec![&ground, &light, &red_sphere, &blue_sphere, &glass_sphere, &mirror1, &mirror2];
    let image_width = options.width;
    let image_height = options.height;
    let f_image_width =  image_width as f32;
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
        let mut image: Vec<Color> = vec![Color::grey(0.0); size];
        for y in 0 .. image_height {
            let f_y = y as f32;
            for x in 0 .. image_width {
                let f_x = x as f32;
                let pixel_index = (y * image_width + x) as usize;
                let mut color = Color::grey(0.0);
                for _s in 0 .. options.sample_count {
                    let u = (f_x + between.sample(&mut rng)) * inverse_width;
                    let v = (f_y + between.sample(&mut rng)) * inverse_height;
                    let ray = cam.get_ray(u, v);
                    color += calc(&ray, 0.0, std::f32::MAX, &objects, 0).saturate();
                }
                image[pixel_index] = color / options.sample_count as f32;
            }
        }
        write_png(image_width, image_height, &convert_to_rgb8(&image), &options.output_name);
    }
}

fn calc(ray: &Ray, t_min: f32, t_max: f32, objects: &Vec<&dyn Intersectable>, depth: u8) -> Color {
    if let Some((obj, point, normal, uvw)) = intersect_objects(ray, t_min, t_max, objects) {
        let mat = obj.material();
        let emitted = mat.emitted(&uvw, &normal);
        let (albedo, scatter, _pdf) = mat.scatter(ray, &point, &normal);

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
//     Color::grey(0.0) * t + Color::new(0.7, 0.2, 0.0, 1.0) * (1.0 - t)
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
