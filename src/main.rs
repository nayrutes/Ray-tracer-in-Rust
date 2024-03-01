//https://raytracing.github.io/books/RayTracingInOneWeekend.html

mod vec3d;
mod ray;
mod hit;
mod sphere;
mod camera;

use std::fs::File;
use std::io::{BufWriter, Write};
use itertools::Itertools;
use indicatif::ProgressIterator;
use crate::hit::Hittable;
use crate::ray::Ray;
use crate::vec3d::Vec3d;
use crate::sphere::Sphere;
use crate::camera::Camera;

fn main() -> std::io::Result<()> {
    println!("Hello, world!");

    let image_width : usize = 400;
    let image_height: usize = 225;

    //let mut image : Image = Image::sample_image(image_height, image_width);
    let mut image : Image;// = Image::new_with_color(image_height, image_width, Vec3d::new(0.,1.,0.));



    let sphere = Sphere::new(Vec3d::new(0.,0., -1.), 0.5);
    let sphere2 = Sphere::new(Vec3d::new(6.,1.,-5.), 2.2);
    let mut world_objects: Vec<Box<dyn Hittable>> = Vec::new();
    world_objects.push(Box::new(sphere));
    world_objects.push(Box::new(sphere2));
    world_objects.push(Box::new(Sphere::new(Vec3d::new(0., -100.5, -1.), 100.)));

    let camera = Camera::new(image_width, image_height, 1.);
    image = camera.render(&world_objects);
    image.write_to_file_bmp("output/sample.bmp")?;
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

        let factor = 255.999;
        let ir = (factor * color.x) as u8;
        let ig = (factor * color.y) as u8;
        let ib = (factor * color.z) as u8;
        self.pixels[row][col].r = ir;
        self.pixels[row][col].g = ig;
        self.pixels[row][col].b = ib;
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