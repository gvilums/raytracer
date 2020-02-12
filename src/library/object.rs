use nalgebra::Vector3;

pub mod sphere;
pub mod cuboid;
pub mod plane;

#[derive(Copy, Clone)]
pub struct Ray {
    pub origin: Vector3<f64>,
    pub dir: Vector3<f64>,
}

impl Ray {
    pub fn new(origin: Vector3<f64>, direction: Vector3<f64>) -> Self {
        let mut ray = Ray { origin, dir: direction };
        ray.dir.normalize_mut();
        ray
    }
}

pub trait Object {
    fn intersects(&self, ray: &Ray) -> Option<(Vector3<f64>, Vector3<f64>)>;
    fn properties(&self) -> Properties;
}

#[derive(Copy, Clone)]
pub struct Properties {
    pub color: Vector3<f64>,
    pub refraction: f64,
    pub reflection: f64,
}