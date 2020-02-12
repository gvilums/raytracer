use nalgebra::{Vector3, Matrix, U1, Vector};
use crate::library::object::{Object, Ray, Properties};

pub struct Plane {
    pub origin: Vector3<f64>,
    pub normal: Vector3<f64>
}

impl Plane {
    pub fn new(pos: Vector3<f64>, normal: Vector3<f64>) -> Self {
        Plane { origin: pos, normal }
    }
}

impl Object for Plane {
    fn intersects(&self, ray: &Ray) -> Option<(Vector3<f64>, Vector3<f64>)> {
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
        Properties {
            color: Vector3::new(0.65, 0.8, 0.8),
            refraction: 0.0,
            reflection: 0.0
        }
    }
}