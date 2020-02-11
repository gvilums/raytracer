use crate::library::object::{sphere::Sphere, Ray, Object};
use crate::library::vec3::Vec3;

mod library;

fn main() {
    let mut spheres = Vec::new();
    spheres.push(Sphere::new(Vec3::new(2f64, 2f64, 2f64), 1f64));

    let ray = Ray::new(
        Vec3::new(0f64, 0f64, 1f64),
        Vec3::new(2f64, 2f64, 1f64)
    );

    let intersection = spheres[0].intersects(&ray).unwrap();
    println!("{:?}", intersection);
}


