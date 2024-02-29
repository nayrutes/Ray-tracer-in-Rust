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

    pub(crate) fn at(self, distance: f64) -> Vec3d{
        return self.origin + self.direction_no_unit * distance;
    }

    pub(crate) fn direction_unit(self) -> Vec3d {
        return self.direction_no_unit.unit();
    }
}