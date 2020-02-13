use nalgebra::{Vector3, Unit};
use crate::library::object::{Object, Ray, Properties};

pub struct Plane {
    pub origin: Vector3<f64>,
    pub normal: Unit<Vector3<f64>>,
    properties: Properties,
}

impl Plane {
    pub fn new(pos: Vector3<f64>, norm_vec: Vector3<f64>) -> Self {
        Plane {
            origin: pos,
            normal: Unit::new_normalize(norm_vec),
            properties: Properties::default()
        }
    }

    pub fn set_properties(&mut self, specular: Vector3<f64>, albedo: Vector3<f64>) {
        self.properties = Properties { specular, albedo }
    }
}

impl Object for Plane {
    fn intersects(&self, ray: &Ray) -> Option<(Vector3<f64>, Unit<Vector3<f64>>)> {
        let num = (self.origin - ray.origin).dot(&self.normal);
        let denom = ray.dir.dot(&self.normal);
        if denom.abs() > 0.001 && num / denom > 0.001 {
            let intersection = ray.origin + ray.dir.scale(num / denom);
            Some((intersection, self.normal))
        } else {
            None
        }
    }

    fn properties(&self) -> Properties {
        self.properties
    }
}