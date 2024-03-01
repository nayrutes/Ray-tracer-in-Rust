use std::iter::Sum;
use std::ops::{Add, Sub, Mul, Div, Range, Neg};
use rand::Rng;

#[derive(Debug, Copy, Clone, PartialEq)]
pub(crate) struct Vec3d {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

impl Vec3d{
    pub(crate) fn new(x:f64, y:f64, z:f64) -> Self{
        Self{
            x,y,z
        }
    }

    pub(crate) fn zero() -> Self {
        return Self::new(0., 0., 0.);
    }

    pub(crate) fn up() -> Self {
        return Self::new(0.,1.,0.);
    }
    pub(crate) fn down() -> Self {
        return Self::new(0.,-1.,0.);
    }

    pub(crate) fn right() -> Self {
        return Self::new(1.,0.,0.);
    }

    pub(crate) fn left() -> Self {
        return Self::new(-1.,0.,0.);
    }

    pub(crate) fn forward() -> Self{
        return Self::new(0.,0.,-1.);
    }

    pub(crate) fn backward() -> Self{
        return Self::new(0.,0.,1.);
    }

    pub(crate) fn length_squared(self) -> f64{
        //return self.dot(self);
        return self.x * self.x + self.y * self.y + self.z * self.z
    }

    fn length(self) -> f64{
        return f64::sqrt(self.length_squared());
    }

    pub(crate) fn dot(self, other: &Vec3d) -> f64{
        return self.x * other.x + self.y * other.y + self.z * other.z;
    }

    fn cross(self, other: Vec3d) -> Self{
        Self{
            x: self.y * other.z - self.z * other.y,
            y: self.z * other.x - self.x * other.z,
            z: self.x * other.y - self.y * other.x,
        }
    }

    pub(crate) fn unit(self) -> Self{
        return self / self.length();
    }

    // pub(crate) fn lerp(self, other: Vec3d, t: f64) -> Vec3d{
    //     return self * (1. - t) + other * t;
    // }
    //
    // pub(crate) fn lerp_clamped(self, other: Vec3d, t: f64) -> Vec3d{
    //     let t = t.max(0.).min(1.);
    //     return self.lerp(other, t);
    // }
    //
    // pub(crate) fn reflect(self, normal: Vec3d) -> Vec3d{
    //     return self - normal * 2. * self.dot(normal);
    // }
    //
    // pub(crate) fn refract(self, normal: Vec3d, etai_over_etat: f64) -> Vec3d{
    //     let cos_theta = (-self).dot(normal).min(1.);
    //     let r_out_perp = (self + normal * cos_theta) * etai_over_etat;
    //     let r_out_parallel = normal * -f64::sqrt(f64::abs(1. - r_out_perp.length_squared()));
    //     return r_out_perp + r_out_parallel;
    // }
    //
    // pub(crate) fn near_zero(self) -> bool{
    //     let s = 1e-8;
    //     return f64::abs(self.x) < s && f64::abs(self.y) < s && f64::abs(self.z) < s;
    // }

    pub(crate) fn random_01() -> Vec3d{
        let mut rng = rand::thread_rng();
        return Vec3d::new(rng.gen(), rng.gen(), rng.gen());
    }

    pub(crate) fn random_from_range(range: Range<f64>) -> Vec3d{
        let mut rng = rand::thread_rng();
        return Vec3d::new(rng.gen_range(range.clone()), rng.gen_range(range.clone()), rng.gen_range(range));
    }

    fn random_in_unit_sphere() -> Vec3d{
        loop{
            let p = Vec3d::random_from_range(-1.0..1.0);
            if p.length_squared() < 1.0{
                return p;
            }
        }
    }

    pub(crate) fn random_unit_vector() -> Vec3d{
        return Self::random_in_unit_sphere().unit();
    }

    pub(crate) fn random_on_hemisphere(normal: &Vec3d) -> Vec3d{
        let on_unit_sphere = Self::random_in_unit_sphere();
        return if(on_unit_sphere.dot(normal)>0.){
            on_unit_sphere
        }else{
            -on_unit_sphere
        }
    }

}

impl Add for Vec3d{
    type Output = Self;

    fn add(self, rhs: Self) -> Self{
        Self{
            x: self.x + rhs.x,
            y: self.y + rhs.y,
            z: self.z + rhs.z,
        }
    }
}

impl Sub for Vec3d{
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Self{
            x: self.x - rhs.x,
            y: self.y - rhs.y,
            z: self.z - rhs.z,
        }
    }
}

impl Mul<f64> for Vec3d{
    type Output = Self;

    fn mul(self, rhs: f64) -> Self::Output {
        Self{
            x: self.x * rhs,
            y: self.y * rhs,
            z: self.z * rhs,
        }
    }
}

impl Mul<Vec3d> for f64{
    type Output = Vec3d;

    fn mul(self, rhs: Vec3d) -> Vec3d {
        Vec3d{
            x: self * rhs.x,
            y: self * rhs.y,
            z: self * rhs.z,
        }
    }
}

impl Div<f64> for Vec3d{
    type Output = Self;

    //more accurate?
    // fn div(self, rhs: f64) -> Self::Output {
    //     Self{
    //         x: self.x / rhs,
    //         y: self.y / rhs,
    //         z: self.z / rhs,
    //     }
    // }

    //more efficient by using only 1 div?
    fn div(self, rhs: f64) -> Self::Output {
        let d = 1./rhs;
        return self * d;
    }
}

//implement negation
impl Neg for Vec3d{
    type Output = Self;

    fn neg(self) -> Self::Output {
        return Vec3d{
            x: -self.x,
            y: -self.y,
            z: -self.z,
        }
    }
}

impl<'a> Neg for &'a Vec3d{
    type Output = Vec3d;

    fn neg(self) -> Self::Output {
        return Vec3d{
            x: -self.x,
            y: -self.y,
            z: -self.z,
        }
    }
}

impl Sum for Vec3d {
    fn sum<I: Iterator<Item=Self>>(iter: I) -> Self {
        iter.fold(Vec3d{x:0.,y:0.,z:0.}, Add::add)
    }
}

impl<'a> Sum<&'a Vec3d> for Vec3d {
    fn sum<I: Iterator<Item=&'a Vec3d>>(iter: I) -> Self {
        iter.fold(Vec3d{x:0.,y:0.,z:0.}, |acc,&item| acc+item.clone())
    }
}

#[cfg(test)]
mod tests{
    use super::*;

    #[test]
    fn test_squared_length(){
        assert_eq!((4 + 9 + 16) as f64, Vec3d {x:2., y:3., z:4.}.length_squared());
    }

    #[test]
    fn test_length(){
        assert_eq!(2f64, Vec3d {x:2., y:0., z:0.}.length());
    }

    #[test]
    fn test_add() {
        assert_eq!(Vec3d {x:3., y:3., z:3.}, Vec3d {x:1., y:2., z:3.} + Vec3d {x:2., y:1., z:0.});
    }

    #[test]
    fn test_sub() {
        assert_eq!(Vec3d {x:-1., y:1., z:0.}, Vec3d {x:1., y:2., z:3.} - Vec3d {x:2., y:1., z:3.});
    }

    #[test]
    fn test_mul() {
        assert_eq!(Vec3d {x:2., y:4., z:6.}, Vec3d {x:1., y:2., z:3.} * 2.);
        assert_eq!(Vec3d {x:2., y:4., z:6.}, 2. * Vec3d {x:1., y:2., z:3.});
    }

    #[test]
    fn test_div(){
        assert_eq!(Vec3d {x:1., y:2., z:3.}, Vec3d{x:2., y:4., z:6.} / 2.);
    }

    #[test]
    fn test_neg() { assert_eq!(Vec3d {x:-1., y:0., z:1.}, -Vec3d{x:1., y:0., z:-1.})}

    #[test]
    fn test_cross(){
        assert_eq!(Vec3d {x:-8., y:-8., z:12.}, Vec3d::new(3.,0.,2.).cross(Vec3d::new(-1.,4.,2.)))
    }

    #[test]
    fn test_unit(){
        assert_eq!(Vec3d {x:0., y:0., z:1.}, Vec3d::new(0.,0.,9.).unit())
    }

    #[test]
    fn test_sum_value(){
        let v = vec![Vec3d::new(1.,2.,3.), Vec3d::new(4.,5.,6.), Vec3d::new(7.,8.,9.)];
        assert_eq!(Vec3d::new(12.,15.,18.), v.into_iter().sum());
    }

    #[test]
    fn test_sum_ref(){
        let v = vec![Vec3d::new(1.,2.,3.), Vec3d::new(4.,5.,6.), Vec3d::new(7.,8.,9.)];
        assert_eq!(Vec3d::new(12.,15.,18.), v.iter().sum());
    }
}