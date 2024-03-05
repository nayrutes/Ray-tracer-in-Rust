use std::ops::{Range, RangeInclusive};
use std::sync::Arc;
use crate::vec3d::Vec3d;
use crate::hit::Hittable;
use crate::hit::HitRecord;
use crate::material::Material;
use crate::ray::Ray;

pub(crate) struct Sphere{
    center: Vec3d,
    radius: f64,
    material: Arc<Material>,
}

impl Sphere{
    pub(crate) fn new(center: Vec3d, radius: f64, material: Arc<Material>) -> Self{
        Self{
            center,
            radius,
            material
        }
    }
}

impl Hittable for Sphere{
    fn hit(&self, ray: &Ray, interval: Range<f64>) -> Option<HitRecord> {
        let oc = ray.origin - self.center;
        //let a = ray.direction_no_unit.dot(ray.direction_no_unit);
        let a = ray.direction_no_unit.length_squared();
        //let b = 2. * oc.dot(ray.direction_no_unit);
        let half_b = oc.dot(&ray.direction_no_unit);
        //let c = oc.dot(oc) - radius * radius;
        let c = oc.length_squared() - self.radius * self.radius;
        //let discriminant = b*b - 4.*a*c;
        let discriminant = half_b*half_b - a*c;
        if (discriminant < 0.) {
            return None;
        }

        let sqrt_discriminant = f64::sqrt(discriminant);
        let mut root = (-half_b -sqrt_discriminant) / a;
        if(!interval.contains(&root)){
            root = (-half_b + sqrt_discriminant) / a;
            if(!interval.contains(&root)){
                return None;
            }
        }
        let pos = ray.at(root);
        let out_dir = (pos - self.center) / self.radius;
        let hit_record = HitRecord::with_unit_normal(pos, out_dir, root, ray.direction_no_unit, self.material.clone());
        return Some(hit_record);
    }
}