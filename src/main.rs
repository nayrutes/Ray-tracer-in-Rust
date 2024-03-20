//https://raytracing.github.io/books/RayTracingInOneWeekend.html

mod vec3d;
mod ray;
mod hit;
mod sphere;
mod camera;
mod material;

use std::fs::File;
use std::io::{BufWriter, Write};
use std::sync::Arc;
use itertools::Itertools;
use indicatif::ProgressIterator;
use rand::{Rng, SeedableRng};
use crate::hit::Hittable;
use crate::ray::Ray;
use crate::vec3d::Vec3d;
use crate::sphere::Sphere;
use crate::camera::Camera;
use crate::material::Material;

fn main() -> std::io::Result<()> {
    println!("Hello, world!");

    let image_width : usize = 400;
    let image_height: usize = 225;

    let image_width : usize = 1920;
    let image_height: usize = 1080;
    let samples_per_pixel: usize = 10000;
    let max_bounces: usize = 7;

    //let mut image : Image = Image::sample_image(image_height, image_width);
    let mut image : Image;// = Image::new_with_color(image_height, image_width, Vec3d::new(0.,1.,0.));

    let material1 = Arc::new(Material::builder().albedo(Vec3d::new(1.,1.,1.), 0.3).reflection(0.9,0.2).build());
    let m_albedo_blue = Arc::new(Material::builder().albedo(Vec3d::new(0.1, 0.4, 0.9), 0.3).build());
    let m_albedo_red = Arc::new(Material::builder().albedo(Vec3d::new(0.9, 0.1, 0.1), 0.3).build());
    let material3 = Arc::new(Material::builder().reflection(1.,0.01).albedo(Vec3d::new(0.,0.,0.),1.0).build());
    let material4 = Arc::new(Material::builder().albedo(Vec3d::new(0.1,0.9,0.3),0.3).reflection(1.0, 0.).build());
    let material5 = Arc::new(Material::builder().refraction(1.5, 1.).build());
    let emission_white = Arc::new(Material::builder().emission(Vec3d::new(1., 1., 1.), 80.).build());
    let emission_green = Arc::new(Material::builder().emission(Vec3d::new(0.1, 1., 0.1), 20.).build());

    let mut world_objects: Vec<Box<dyn Hittable + Sync>> = Vec::new();
    world_objects.push(Box::new(Sphere::new(Vec3d::new(0., -100.5, -1.), 100., m_albedo_blue.clone())));
    world_objects.push(Box::new(Sphere::new(Vec3d::new(0., 210.5, -1.), 200., m_albedo_red.clone())));
    world_objects.push(Box::new(Sphere::new(Vec3d::new(-3.5,0., -3.), 1., material1.clone())));
    world_objects.push(Box::new(Sphere::new(Vec3d::new(-1.0,-0.3, -1.3), 0.3, material5.clone())));
    world_objects.push(Box::new(Sphere::new(Vec3d::new(-1.,0., -5.0), 2.25, material4.clone())));

    world_objects.push(Box::new(Sphere::new(Vec3d::new(5.,2., -8.0), 2.5, emission_white.clone())));
    world_objects.push(Box::new(Sphere::new(Vec3d::new(-0.3,-0.4, -0.8), 0.1, emission_green.clone())));

    //world_objects.push(Box::new(Sphere::new(Vec3d::new(6.,1.,-5.), 2.2, material3.clone())));
    //world_objects.push(Box::new(Sphere::new(Vec3d::new(1.,-0.5, -1.), 0.25, material4.clone())));
    //world_objects.push(Box::new(Sphere::new(Vec3d::new(1.,0.5, -1.), 0.25, material5.clone())));
    world_objects.push(Box::new(Sphere::new(Vec3d::new(0.7,0.,-0.7), -0.2, material5.clone())));
    //world_objects.push(Box::new(Sphere::new(Vec3d::new(1.5,0.,-2.), -0.25, material6.clone())));

    //generate random spheres with seed
    let seed = [0; 32];
    let mut rng = rand::prelude::StdRng::from_seed(seed);
    //let mut rng = rand::thread_rng();

    for _ in 0..100{
        let x = rng.gen_range(-5.0..5.0);
        let y = rng.gen_range(0.0..0.1)- 0.5;
        let z = rng.gen_range(-4.0..-0.5);
        let r = rng.gen_range(0.05..0.25);
        let material = Arc::new(Material::builder().albedo(Vec3d::new(rng.gen_range(0.0..1.0),rng.gen_range(0.0..1.0),rng.gen_range(0.0..1.0)),0.3).build());
        world_objects.push(Box::new(Sphere::new(Vec3d::new(x,y,z), r, material)));
    }

    let camera = Camera::new(image_width, image_height, 1., samples_per_pixel, max_bounces, false);
    image = camera.render(&world_objects);
    image.write_to_file_bmp("output/sample.bmp")?;
    //copy file and name it with render settings
    std::fs::copy("output/sample.bmp", format!("output/sample_{}_{}_{}_{}.bmp", image_width, image_height, samples_per_pixel, max_bounces))?;

    //image.write_to_file_ppm("output/sample.ppm")?;

    Ok(())
}

fn lerp(v1: f64, v2:f64, t:f64) -> f64{
    return (1. - t) * v1 + t * v2;
}
fn lerp_vec3d(v1: Vec3d, v2:Vec3d, t:f64) -> Vec3d{
    return (1. - t) * v1 + t * v2;
}



struct Pixel {
    r: u8,
    g: u8,
    b: u8,
}

impl Pixel {
    pub fn new() -> Self{
        Self{
            r: 255,
            g: 0,
            b: 255
        }
    }
}

struct Image{
    width: usize,
    height: usize,
    pixels: Vec<Vec<Pixel>>
}

impl Image {
    pub fn new(height: usize, width: usize) -> Self{
        assert!(height > 0 && width > 0);
        let mut pixels = Vec::with_capacity(height);
        for _ in 0..height{
            let mut row =  Vec::with_capacity(width);
            for _ in 0..width{
                row.push(Pixel::new())
            }
            pixels.push(row);
        }

        Self {
            width,
            height,
            pixels
        }
    }

    pub fn new_with_color(height: usize, width: usize, color: Vec3d) -> Self{
        let mut image = Image::new(height,width);
        for it in (0..image.height)
            .cartesian_product(0..image.width)
            .progress_count(image.height as u64 * image.width as u64){
            let row = it.0;
            let col = it.1;

            image.set_pixel_color(row, col, color);
        }
        return image;
    }

    pub fn sample_image(height: usize, width: usize) -> Self{
        let mut image = Image::new(height,width);
        for row in 0..image.height{
            for col in 0..image.width {
                let r:f64 = col as f64 / (image.width as f64 - 1.0);
                let g:f64 = row as f64 / (image.height as f64 - 1.0);
                let b:f64 = 0.;
                let color = Vec3d::new(r,g,b);

                image.set_pixel_color(row,col,color);
            }
        }
        return image;
    }

    pub(crate) fn set_pixel_color(&mut self, row: usize, col: usize, color: Vec3d) {
        assert!(row >= 0 && row < self.height);
        assert!(col >= 0 && col < self.width);

        let r = Self::linear_to_gamma(color.x);
        let g = Self::linear_to_gamma(color.y);
        let b = Self::linear_to_gamma(color.z);

        let factor = 255.999;
        let ir = (factor * r) as u8;
        let ig = (factor * g) as u8;
        let ib = (factor * b) as u8;
        self.pixels[row][col].r = ir;
        self.pixels[row][col].g = ig;
        self.pixels[row][col].b = ib;
    }
    pub(crate) fn set_pixels(&mut self, pixel_vector: Vec<Vec3d>) {
        assert_eq!(pixel_vector.len(), self.height * self.width);
        for row in 0..self.height{
            for col in 0..self.width {
                self.set_pixel_color(row, col, pixel_vector[row * self.width + col]);
            }
        }
    }

    fn linear_to_gamma(linear: f64) -> f64{
        return f64::sqrt(linear);
    }

    pub fn display(&self){
        println!("P3");
        println!("{} {}",self.width, self.height);
        println!("255");
        for row in 0..self.height{
            for col in 0..self.width {
                //println!("{:>3} {:>3} {:>3}", self.pixels[row][col].r, self.pixels[row][col].g, self.pixels[row][col].b);
                println!("{} {} {}", self.pixels[row][col].r, self.pixels[row][col].g, self.pixels[row][col].b);
            }
        }
    }

    pub(crate) fn write_to_file_ppm(&self, path: &str) -> std::io::Result<()> {
        //create directory if needed
        if let Some(parent) = std::path::Path::new(&path).parent(){
            std::fs::create_dir_all(parent)?;
        }

        let mut buffer: BufWriter<File> = BufWriter::new(File::create(path)?);
        buffer.write_all(format!("P3\n{} {}\n255\n", self.width, self.height).as_bytes()).unwrap();
        for row in 0..self.height{
            for col in 0..self.width {
                buffer.write_all(format!("{} {} {}\n", self.pixels[row][col].r, self.pixels[row][col].g, self.pixels[row][col].b).as_bytes()).unwrap();
            }
        }
        buffer.flush()?;
        Ok(())
    }

    pub(crate) fn write_to_file_bmp(&self, path: &str) -> std::io::Result<()> {
        //create directory if needed
        if let Some(parent) = std::path::Path::new(&path).parent(){
            std::fs::create_dir_all(parent)?;
        }

        let mut buffer: BufWriter<File> = BufWriter::new(File::create(path)?);

        let padding_amount = (4 - (self.width * 3) % 4) % 4;
        let file_header_size = 14;
        let info_header_size = 40;
        let file_size = (file_header_size + info_header_size + (3 * self.width + padding_amount) * self.height) as u32;
        let reserved: u16 = 0;
        let offset_to_pixel_array = (file_header_size + info_header_size) as u32;

        buffer.write_all("BM".as_bytes()).unwrap();
        buffer.write_all(&file_size.to_le_bytes()).unwrap();
        buffer.write_all(&reserved.to_le_bytes()).unwrap();
        buffer.write_all(&reserved.to_le_bytes()).unwrap();
        buffer.write_all(&offset_to_pixel_array.to_le_bytes()).unwrap();

        let info_header_size: u32 = 40;
        let width: i32 = self.width as i32;
        let height: i32 = self.height as i32;
        let planes: u16 = 1;
        let bits_per_pixel: u16 = 24;
        let compression: u32 = 0;
        let image_size: u32 = (3 * self.width + padding_amount) as u32 * self.height as u32;
        let x_pixels_per_meter: i32 = 0;
        let y_pixels_per_meter: i32 = 0;
        let total_colors: u32 = 0;
        let important_colors: u32 = 0;

        buffer.write_all(&info_header_size.to_le_bytes()).unwrap();
        buffer.write_all(&width.to_le_bytes()).unwrap();
        buffer.write_all(&height.to_le_bytes()).unwrap();
        buffer.write_all(&planes.to_le_bytes()).unwrap();
        buffer.write_all(&bits_per_pixel.to_le_bytes()).unwrap();
        buffer.write_all(&compression.to_le_bytes()).unwrap();
        buffer.write_all(&image_size.to_le_bytes()).unwrap();
        buffer.write_all(&x_pixels_per_meter.to_le_bytes()).unwrap();
        buffer.write_all(&y_pixels_per_meter.to_le_bytes()).unwrap();
        buffer.write_all(&total_colors.to_le_bytes()).unwrap();
        buffer.write_all(&important_colors.to_le_bytes()).unwrap();

        for row in 0..self.height{
            for col in 0..self.width {
                let reversed_row = self.height - row - 1;
                buffer.write_all(&self.pixels[reversed_row][col].b.to_le_bytes()).unwrap();
                buffer.write_all(&self.pixels[reversed_row][col].g.to_le_bytes()).unwrap();
                buffer.write_all(&self.pixels[reversed_row][col].r.to_le_bytes()).unwrap();
            }
            for _ in 0..padding_amount{
                buffer.write_all(&0u8.to_le_bytes()).unwrap();
            }
        }
        buffer.flush()?;
        Ok(())
    }
}