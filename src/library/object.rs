use nalgebra::{Vector3, Unit};

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
    fn intersects(&self, ray: &Ray) -> Option<(Vector3<f64>, Unit<Vector3<f64>>)>;
    fn properties(&self) -> Properties;
}

#[derive(Copy, Clone)]
pub struct Properties {
    pub specular: Vector3<f64>,
    pub albedo: Vector3<f64>,
}

impl Default for Properties {
    fn default() -> Self {
        Properties {
            specular: Vector3::new(1f64, 0.78f64, 0.34f64),
            albedo: Vector3::new(0f64, 0f64, 0f64),
        }
    }
}