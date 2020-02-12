use crate::library::object::{sphere::Sphere, Object};
use nalgebra::Vector3;
use crate::library::tracer::trace_all;
use crate::library::light::Light;
use crate::library::object::plane::Plane;
use image::Rgb;

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

    let width = 600;
    let height = 400;

    let pixel_colors = trace_all(
        Vector3::new(0f64, 0f64, -50f64),
        Vector3::new(1f64, 0f64, 0f64),
        Vector3::new(0f64, 1f64, 0f64),
        std::f64::consts::PI * 1. / 3.,
        width,
        height,
        objects,
        lights,
    );

    // temporary write to image
    let mut imgbuf = image::ImageBuffer::new(width, height);
    for pixel in imgbuf.pixels_mut() {
        *pixel = Rgb([255u8, 255u8, 255u8]);
    }

    // write the pixel colors to the image
    pixel_colors.iter().for_each(|(x, y, color_vec)| {
        let color = Rgb([
            (255. * color_vec[0]) as u8,
            (255. * color_vec[1]) as u8,
            (255. * color_vec[2]) as u8
        ]);
        imgbuf.put_pixel(*x, *y, color);
    });
    imgbuf.save("out.png").unwrap();
}


