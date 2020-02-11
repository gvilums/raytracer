use crate::library::object::{Ray, Object};
use crate::library::vec3::Vec3;

pub fn trace(ray: &Ray, scene: Vec<Box<dyn Object>>) -> Vec3 {
    unimplemented!()
}

pub fn trace_all(
    pos: Vec3, view: Vec3, rotation: Vec3, fov: f64,
    width: u32, height: u32,
    scene: Vec<Box<dyn Object>>
) -> () {
    unimplemented!()
}