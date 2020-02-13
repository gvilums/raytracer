use crate::library::object::{Object, Ray, Properties};

use nalgebra::{Vector3, Unit};

pub struct Sphere {
    center: Vector3<f64>,
    radius: f64,
    properties: Properties,
}

impl Sphere {
    pub fn new(center: Vector3<f64>, radius: f64) -> Self {
        Sphere { center, radius, properties: Properties::default() }
    }

    pub fn set_properties(&mut self, specular: Vector3<f64>, albedo: Vector3<f64>) {
        self.properties = Properties { specular, albedo };
    }
}

impl Object for Sphere {
    fn intersects(&self, ray: &Ray) -> Option<(Vector3<f64>, Unit<Vector3<f64>>)> {
        let discriminant = ray.dir.dot(&(ray.origin - self.center)).powi(2)
                - (ray.origin - self.center).norm_squared()
                + self.radius.powi(2);

        if discriminant >= 0. {
            let base = -ray.dir.dot(&(ray.origin - self.center));
            let factor_close = base - discriminant.sqrt();
            let factor_far = base + discriminant.sqrt();

            if factor_close > 0.001 {
                let close_intersect = ray.origin + ray.dir.scale(factor_close);
                let close_normal = Unit::new_normalize(close_intersect - self.center);
                Some((close_intersect, close_normal))
            } else if factor_far > 0.001 {
                let far_intersect = ray.origin + ray.dir.scale(factor_far);
                let far_normal = Unit::new_normalize(far_intersect - self.center);
                Some((far_intersect, far_normal))
            } else {
                None
            }
        } else {
            None
        }
    }

    fn properties(&self) -> Properties {
        self.properties
    }
}