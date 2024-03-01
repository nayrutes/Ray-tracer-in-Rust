use rand::prelude::*;
use crate::hit::Hittable;
use crate::lerp_vec3d;
use crate::vec3d::Vec3d;

#[derive(Debug, Copy, Clone, PartialEq)]
pub(crate) struct Ray{
    pub(crate) origin : Vec3d,
    pub(crate) direction_no_unit: Vec3d,
}

impl Ray{
    pub(crate) fn new(origin: Vec3d, direction_no_unit: Vec3d) -> Self{
        Self{
            origin,
            direction_no_unit
        }
    }
    //
    // pub(crate) fn new_rng_offset(mut origin: Vec3d, direction_no_unit: Vec3d) -> Self{
    //     Self{
    //         origin,
    //         direction_no_unit
    //     }
    // }

    pub(crate) fn at(&self, distance: f64) -> Vec3d{
        return self.origin + self.direction_no_unit * distance;
    }

    pub(crate) fn direction_unit(self) -> Vec3d {
        return self.direction_no_unit.unit();
    }

    pub(crate) fn ray_color<T>(&self, hittable: &T) -> Vec3d where T:Hittable {
        if let Some(rec) = hittable.hit(&self, (0.)..f64::INFINITY){
            return 0.5 * (rec.normal + Vec3d::new(1.,1.,1.))
        }

        //Background color lerp
        let t = 0.5*(self.direction_unit().y + 1.0);
        let pixel_color = lerp_vec3d(Vec3d::new(1.,1.,1.),Vec3d::new(0.5,0.7,1.0),t);
        return pixel_color;
    }
}