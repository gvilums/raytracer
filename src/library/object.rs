use super::vec3::Vec3;

pub struct Ray {
    origin: Vec3,
    pub(crate) dir: Vec3,
}

impl Ray {
    pub fn new(origin: Vec3, direction: Vec3) -> Self {
        let mut ray = Ray { origin, dir: direction };
        ray.dir.norm();
        ray
    }
}

pub trait Object {
    fn intersects(&self, ray: &Ray) -> Option<Vec3>;
}

pub struct Sphere {
    center: Vec3,
    radius: f64,
}

impl Sphere {
    pub fn new(center: Vec3, radius: f64) -> Self {
        Sphere { center, radius }
    }
}

impl Object for Sphere {
    fn intersects(&self, ray: &Ray) -> Option<Vec3> {
        let discriminant =
            ray.dir.dot(&(ray.origin - self.center)).powi(2)
                - (ray.origin - self.center).dot_self()
                + self.radius.powi(2);

        if discriminant >= 0. {
            let base = -ray.dir.dot(&(ray.origin - self.center));
            Some(ray.origin + ray.dir.scaled(base - discriminant))
        } else {
            None
        }
    }
}