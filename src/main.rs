//https://raytracing.github.io/books/RayTracingInOneWeekend.html

mod vec3d;

use std::fs::File;
use std::io::{BufWriter, Write};
use itertools::Itertools;
use indicatif::ProgressIterator;

fn main() -> std::io::Result<()> {
    println!("Hello, world!");

    let image_width : usize = 256;
    let image_height: usize = 256;

    let image : Image = Image::sample_image(image_height, image_width);

    println!("Rendering image with width {} and height {} ...",image_width, image_height);

    for it in (0..image.height)
        .cartesian_product(0..image.width)
        .progress_count(image.height as u64 * image.width as u64){
        let row = it.0;
        let col = it.1;


    }


    //image.display();
    image.write_to_file_bmp("output/sample.bmp")?;
    //image.write_to_file_ppm("output/sample.ppm")?;

    Ok(())
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

    pub fn sample_image(height: usize, width: usize) -> Self{
        let mut image = Image::new(height,width);
        for row in 0..image.height{
            for col in 0..image.width {
                let r:f64 = col as f64 / (image.width as f64 - 1.0);
                let g:f64 = row as f64 / (image.height as f64 - 1.0);
                let b:f64 = 0.;

                let factor = 255.999;
                let ir = (factor * r) as u8;
                let ig = (factor * g) as u8;
                let ib = (factor * b) as u8;

                image.pixels[row][col].r = ir;
                image.pixels[row][col].g = ig;
                image.pixels[row][col].b = ib;
            }
        }
        return image;
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