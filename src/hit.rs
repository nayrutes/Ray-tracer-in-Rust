use std::ops::Range;
use crate::ray::Ray;
use crate::vec3d::Vec3d;

pub(crate) trait Hittable {
    fn hit(
        &self,
        ray: &Ray,
        interval: Range<f64>,
    ) -> Option<HitRecord>;
}

pub(crate) struct HitRecord {
    pos: Vec3d,
    pub(crate) normal: Vec3d,
    t: f64,
    //front_face: bool,
}

impl HitRecord{
    pub(crate) fn with_unit_normal(pos: Vec3d, unit_normal: Vec3d, t: f64) -> Self{
        Self{
            pos,
            normal: unit_normal,
            t,
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