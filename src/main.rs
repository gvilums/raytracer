use crate::library::object::{sphere::Sphere, Ray, Object};
use nalgebra::Vector3;
use nalgebra::geometry::Rotation3;
use nalgebra::Unit;
use crate::library::tracer::trace_all;
use crate::library::light::Light;
use crate::library::object::plane::Plane;

mod library;

fn main() {
    let mut objects: Vec<Box<dyn Object + Sync>> = Vec::new();
    objects.push(Box::new(Sphere::new(Vector3::new(-15f64, -5f64, 60f64), 10.)));
    objects.push(Box::new(Sphere::new(Vector3::new(60f64, 0f64, 150f64), 10.)));
    objects.push(Box::new(Sphere::new(Vector3::new(40f64, -20f64, 100f64), 10.)));

    objects.push(Box::new(Plane::new(
        Vector3::new(0f64, -40f64, 0f64),
        Vector3::new(0f64, 1f64, 0f64)
    )));


    let mut lights: Vec<Light> = Vec::new();
    for i in 1..30 {
        for j in -6..6 {
            lights.push(Light::new(Vector3::new(3. * j as f64, 50f64, 2. * i as f64), 1f64));
        }
    }

    trace_all(
        Vector3::new(0f64, 0f64, -50f64),
        Vector3::new(1f64, 0f64, 0f64),
        Vector3::new(0f64, 1f64, 0f64),
        std::f64::consts::PI * 1. / 3.,
        600,
        400,
        objects,
        lights,
    )
}


