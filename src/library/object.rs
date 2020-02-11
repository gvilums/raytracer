use crate::library::vec3::Vec3;


pub mod sphere;
pub mod cuboid;

pub struct Ray {
    pub origin: Vec3,
    pub dir: Vec3,
}

impl Ray {
    pub fn new(origin: Vec3, direction: Vec3) -> Self {
        let mut ray = Ray { origin, dir: direction };
        ray.dir.norm();
        ray
    }
}

pub trait Object {
    fn intersects(&self, ray: &Ray) -> Option<(Vec3, Vec3)>;
    fn properties(&self) -> Properties;
}

pub struct Properties {
    pub color: Vec3,
    pub refraction: f64,
    pub reflection: f64,
}