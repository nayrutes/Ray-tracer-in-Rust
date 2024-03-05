use std::ops::Range;
use std::sync::Arc;
use crate::ray::Ray;
use crate::vec3d::Vec3d;
use crate::material::Material;

pub(crate) trait Hittable {
    fn hit(
        &self,
        ray: &Ray,
        interval: Range<f64>,
    ) -> Option<HitRecord>;
}

pub(crate) struct HitRecord {
    pub(crate) pos: Vec3d,
    pub(crate) normal: Vec3d,
    t: f64,
    pub(crate) material: Arc<Material>,
    //front_face: bool,
    pub(crate) front_face: bool,
}

impl HitRecord {
    pub(crate) fn with_unit_normal(pos: Vec3d, unit_normal: Vec3d, t: f64, in_dir: Vec3d, material: Arc<Material>) -> HitRecord {
        let front_face = unit_normal.dot(&in_dir) < 0.;
        let normal = if front_face {unit_normal} else {-unit_normal};
        Self{
            pos,
            normal,
            front_face,
            t,
            material,
        }
    }
}

impl Hittable for &Vec<Box<dyn Hittable>> {
    fn hit(&self, ray: &Ray, interval: Range<f64>) -> Option<HitRecord> {
        let mut closest_so_far = interval.end;
        let mut temp_rec = None;
        for hittable in self.iter(){
            if let Some(rec) = hittable.hit(ray, interval.start..closest_so_far){
                closest_so_far = rec.t;
                temp_rec = Some(rec);
            }
        }
        return temp_rec;
    }
}