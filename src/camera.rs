use std::ops::Mul;
use indicatif::ProgressIterator;
use itertools::Itertools;
use rand::{random, Rng, thread_rng};
use crate::hit::Hittable;
use crate::{Image, lerp_vec3d};
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
    max_bounces: usize,
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
            samples_per_pixel : 1000,
            max_bounces: 7,
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
                let ray = &self.generate_rng_offset_ray(row, col);
                return Self::ray_color(ray, self.max_bounces+1, &world) * scale;
            }).sum::<Vec3d>();

            image.set_pixel_color(row, col, multisample_color);
        }

        return image;
    }

    pub(crate) fn ray_color<T>(ray: &Ray, bounces_left: usize, hittable: &T) -> Vec3d where T:Hittable {
        if bounces_left == 0{
            return Vec3d::zero();
        }
        if let Some(hit_record) = hittable.hit(ray, (0.001)..f64::INFINITY){
            //let diffuse_direction_random = Vec3d::random_on_hemisphere(&hit_record.normal);
            let mut color: Vec3d = Vec3d::zero();

            //trace diffuse and reflective rays
            let attenuation = hit_record.material.albedo_color;

            // if(hit_record.material.reflectivity > 0.){
            //     let reflect_direction = ray.direction_no_unit.unit().reflect(hit_record.normal);
            //     let ray_bounced = Ray::new(hit_record.pos, reflect_direction);
            //     let bounced_color = Self::ray_color(&ray_bounced, bounces_left -1, hittable);
            //     color = color + hit_record.material.reflectivity * bounced_color;
            // }
            //
            // if(hit_record.material.reflectivity < 1.){
            //     let diffuse_direction_lambertian= hit_record.normal + Vec3d::random_unit_vector().near_zero_alt(hit_record.normal);
            //     let ray_bounced = Ray::new(hit_record.pos, diffuse_direction_lambertian);
            //     let bounced_color = Self::ray_color(&ray_bounced, bounces_left -1, hittable);
            //     color = color + (1. - hit_record.material.reflectivity) * 0.7 * attenuation.comp_vise(bounced_color);
            // }

            //decision witch ray to trace
            let sum = ((hit_record.material.absorption) + hit_record.material.reflectivity + hit_record.material.refractioness);
            let chance = random::<f64>() * sum;
            //reflective
            if chance < hit_record.material.reflectivity{
                let reflect_direction = ray.direction_no_unit.unit().reflect(&hit_record.normal);
                let mut fuzz_vector = Vec3d::zero();
                if(hit_record.material.reflection_fuzz > 0.) {
                    fuzz_vector = hit_record.material.reflection_fuzz * Vec3d::random_in_unit_sphere();
                }
                let ray_bounced = Ray::new(hit_record.pos, reflect_direction + fuzz_vector);
                let bounced_color = Self::ray_color(&ray_bounced, bounces_left -1, hittable);
                if(ray_bounced.direction_no_unit.dot(&hit_record.normal) > 0.){
                    color = color + hit_record.material.reflectivity * bounced_color;
                }
                //color = color + hit_record.material.reflectivity * bounced_color;
            }
            //refractive
            else if (chance < hit_record.material.reflectivity + hit_record.material.refractioness){
                let refraction_ratio = if hit_record.front_face {1. / hit_record.material.refraction_index} else {hit_record.material.refraction_index};
                let unit_direction = ray.direction_no_unit.unit();
                let cos_theta = (-unit_direction).dot(&hit_record.normal).min(1.);
                let sin_theta = f64::sqrt(1. - cos_theta * cos_theta);
                let cannot_refract = refraction_ratio * sin_theta > 1.;
                let mut direction = Vec3d::zero();
                if cannot_refract || Self::reflectance(cos_theta, refraction_ratio) > random::<f64>(){
                    direction = unit_direction.reflect(&hit_record.normal);
                }
                else{
                    direction = unit_direction.refract(&hit_record.normal, refraction_ratio);
                }
                let ray_bounced = Ray::new(hit_record.pos, direction);
                let bounced_color = Self::ray_color(&ray_bounced, bounces_left -1, hittable);
                color = color + (1. -hit_record.material.absorption) * bounced_color;
            }
            //diffuse
            else if(chance < hit_record.material.reflectivity + hit_record.material.refractioness + (hit_record.material.absorption)){
                let diffuse_direction_lambertian= hit_record.normal + Vec3d::random_unit_vector().near_zero_alt(hit_record.normal);
                let ray_bounced = Ray::new(hit_record.pos, diffuse_direction_lambertian);
                let bounced_color = Self::ray_color(&ray_bounced, bounces_left -1, hittable);
                color = color + (1.-hit_record.material.absorption) * attenuation.comp_vise(bounced_color);
            }
            else {
                println!("Error: no ray was traced");
            }


            return color;
        }

        //Background color lerp
        let t = 0.5*(ray.direction_unit().y + 1.0);
        let pixel_color = lerp_vec3d(Vec3d::new(1.,1.,1.),Vec3d::new(0.5,0.7,1.0),t);
        return pixel_color;
    }

    pub(crate) fn reflectance(cosine: f64, refraction_index: f64) -> f64 {
        let r0 = ((1. - refraction_index) / (1. + refraction_index));
        let r0_squared = r0 * r0;
        return r0_squared + (1. - r0_squared) * (1. - cosine).powi(5);
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