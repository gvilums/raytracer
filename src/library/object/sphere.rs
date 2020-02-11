use crate::library::vec3::Vec3;
use crate::library::object::{Object, Ray, Properties};

pub struct Sphere {
    center: Vec3,
    radius: f64,
    properties: Properties,
}

impl Sphere {
    pub fn new(center: Vec3, radius: f64) -> Self {
        let properties = Properties {
            color: Vec3::new(1f64, 0f64, 0f64),
            reflection: 0f64,
            refraction: 0f64
        };
        Sphere { center, radius, properties }
    }
}

impl Object for Sphere {
    fn intersects(&self, ray: &Ray) -> Option<(Vec3, Vec3)> {
        let discriminant =
            ray.dir.dot(&(ray.origin - self.center)).powi(2)
                - (ray.origin - self.center).dot_self()
                + self.radius.powi(2);

        if discriminant >= 0. {
            let base = -ray.dir.dot(&(ray.origin - self.center));
            let intersection = ray.origin + ray.dir.scaled(base - discriminant);
            let mut normal = intersection - self.center;
            normal.norm();
            Some((intersection, normal))
        } else {
            None
        }
    }

    fn properties(&self) -> Properties {
        unimplemented!();
    }
}