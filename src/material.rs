use crate::ray::Ray;
use crate::vec3d::Vec3d;

pub(crate) struct Material{
    pub albedo_color : Vec3d,
    pub smoothness: f64,
    pub reflectivity: f64,
    pub absorption: f64,
    pub reflection_fuzz: f64,
    pub(crate) refractioness: f64,
    pub refraction_index: f64,
}

impl Material {
    pub(crate) fn builder() -> MaterialBuilder {
        MaterialBuilder{
            albedo_color: Vec3d::new(0.5,0.,0.5),
            smoothness: 0.,
            reflectivity: 0.,
            absorption: 1.,
            reflection_fuzz: 0.,
            refractioness: 0.,
            refraction_index: 1.,
        }
    }
}


pub(crate) struct MaterialBuilder{
    albedo_color : Vec3d,
    smoothness: f64,
    reflectivity: f64,
    absorption: f64,
    reflection_fuzz: f64,
    refractioness: f64,
    refraction_index: f64,
}

impl MaterialBuilder {

    pub(crate) fn new() -> MaterialBuilder {
        MaterialBuilder{
            albedo_color: Vec3d::new(0.5,0.,0.5),
            smoothness: 0.,
            reflectivity: 0.,
            absorption: 0.,
            reflection_fuzz: 0.,
            refractioness: 0.,
            refraction_index: 1.,
        }
    }

    pub(crate) fn albedo(mut self, color: Vec3d) -> MaterialBuilder {
        self.albedo_color = color;
        self
    }

    pub(crate) fn smoothness(mut self, smoothness: f64) -> MaterialBuilder {
        let smoothness_clamped = smoothness.clamp(0.,1.);
        self.smoothness = smoothness_clamped;
        self
    }

    pub(crate) fn reflectivity(mut self, reflectivity: f64) -> MaterialBuilder {
        let reflectivity_clamped = reflectivity.clamp(0.,1.);
        self.reflectivity = reflectivity_clamped;
        self
    }

    pub(crate) fn absorption(mut self, absorption: f64) -> MaterialBuilder {
        let absorption_clamped = absorption.clamp(0.,1.);
        self.absorption = absorption_clamped;
        self
    }

    pub(crate) fn reflection_fuzz(mut self, reflection_fuzz: f64) -> MaterialBuilder {
        let reflection_fuzz_clamped = reflection_fuzz.clamp(0.,1.);
        self.reflection_fuzz = reflection_fuzz_clamped;
        self
    }

    pub(crate) fn refraction_index(mut self, refraction_index: f64) -> MaterialBuilder {
        self.refraction_index = refraction_index;
        self
    }

    pub(crate) fn refractioness(mut self, refractioness: f64) -> MaterialBuilder {
        self.refractioness = refractioness;
        self
    }

    pub(crate) fn build(self) -> Material {
        Material{
            albedo_color: self.albedo_color,
            smoothness: self.smoothness,
            reflectivity: self.reflectivity,
            absorption: self.absorption,
            reflection_fuzz: self.reflection_fuzz,
            refractioness: self.refractioness,
            refraction_index: self.refraction_index,
        }
    }
}