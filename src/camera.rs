use indicatif::ProgressIterator;
use itertools::Itertools;
use rand::{Rng, thread_rng};
use crate::hit::Hittable;
use crate::Image;
use crate::ray::Ray;
use crate::vec3d::Vec3d;

pub(crate) struct Camera {
    image_width: usize,
    image_height: usize,
    focal_length: f64,

    camera_origin: Vec3d,
    camera_direction: Vec3d,
    viewport_pos: Vec3d,
    aspect_ratio: f64,
    viewport_height: f64,
    viewport_width: f64,
    viewport_u: Vec3d,
    viewport_v: Vec3d,
    pixel_delta_u: Vec3d,
    pixel_delta_v: Vec3d,
    pixel00_pos: Vec3d,
    samples_per_pixel: usize,
}

impl Camera {
    pub(crate) fn new(image_width: usize, image_height: usize, focal_length: f64) -> Self {

        let camera_origin = Vec3d::zero();
        let camera_direction = Vec3d::forward();
        let viewport_pos = camera_origin + camera_direction * focal_length;

        let aspect_ratio = image_width as f64 / image_height as f64;
        let viewport_height = 2f64;
        let viewport_width = viewport_height * aspect_ratio;
        let viewport_u = Vec3d::right() * viewport_width;
        let viewport_v = Vec3d::down() * viewport_height;
        let pixel_delta_u = viewport_u / image_width as f64;
        let pixel_delta_v = viewport_v / image_height as f64;

        let viewport_upper_left = viewport_pos - viewport_u/2. - viewport_v/2.;
        let pixel00_pos = viewport_upper_left + (pixel_delta_u + pixel_delta_v) * 0.5;

        Self{
            image_width,
            image_height,
            focal_length,
            camera_origin,
            camera_direction,
            aspect_ratio,
            viewport_height,
            viewport_width,
            viewport_u,
            viewport_v,
            pixel_delta_u,
            pixel_delta_v,
            viewport_pos,
            pixel00_pos,
            samples_per_pixel : 100,
        }
    }

    pub(crate) fn render(&self, world: &Vec<Box<dyn Hittable>>) -> Image{
        let mut image: Image = Image::new_with_color(self.image_height, self.image_width, Vec3d::new(0.,1.,0.));
        println!("Rendering image with width {} and height {} ...",self.image_width, self.image_height);
        for it in (0..image.height)
            .cartesian_product(0..image.width)
            .progress_count(image.height as u64 * image.width as u64){
            let row = it.0;
            let col = it.1;

            //sampling inside a pixel
            let scale = (self.samples_per_pixel as f64).recip();
            let multisample_color = (0..self.samples_per_pixel)
                .into_iter().map(|_|{
                return self.generate_rng_offset_ray(row, col).ray_color(&world) * scale;
            }).sum::<Vec3d>();

            image.set_pixel_color(row, col, multisample_color);
        }

        return image;
    }

    fn generate_rng_offset_ray(&self, row: usize, col: usize) -> Ray {
        let mut rng = thread_rng();
        let px: f64 = -0.5 + rng.gen::<f64>();
        //let px: f64 = 0.;
        let py: f64 = -0.5 + rng.gen::<f64>();
        //let py: f64 = 0.;

        let pixel_center = self.pixel00_pos + ((col as f64 + px) * (self.pixel_delta_u)) + ((row as f64 + py) * self.pixel_delta_v);
        let ray_direction_no_unit = pixel_center - self.camera_origin;
        let ray = Ray::new(self.camera_origin, ray_direction_no_unit);
        return ray;
    }
}