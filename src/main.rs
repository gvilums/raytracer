use crate::library::object::{sphere::Sphere, Object};
use nalgebra::Vector3;
use crate::library::tracer::{trace_all, spherical_view};
use crate::library::light::Light;
use crate::library::object::plane::Plane;
use image::Rgb;

mod library;

fn main() {
    let mut objects: Vec<Box<dyn Object + Sync>> = Vec::new();
    let mut s1 = Sphere::new(Vector3::new(-15f64, -5f64, 100f64), 10.);
    let mut s2 = Sphere::new(Vector3::new(60f64, 0f64, 150f64), 10.);
    let mut s3 = Sphere::new(Vector3::new(35f64, -10f64, 75f64), 10.);
    s3.set_properties(Vector3::new(0.03, 0.03, 0.03), Vector3::new(0., 0.78, 0.34));
    s2.set_properties(Vector3::new(1., 0.78, 0.34), Vector3::new(0., 0., 0.));
    s1.set_properties(Vector3::new(0.08, 0.08, 0.08), Vector3::new(0.5, 0.8, 0.8));
    objects.push(Box::new(s1));
    objects.push(Box::new(s2));
    objects.push(Box::new(s3));

    let mut p = Plane::new(
        Vector3::new(0f64, -40f64, 0f64),
        Vector3::new(0f64, 1f64, 0f64)
    );
    p.set_properties(
        Vector3::new(0.01f64, 0.01f64, 0.01f64),
        Vector3::new(0.6f64, 0.6f64, 0.6f64),
    );

    objects.push(Box::new(p));


    let mut lights: Vec<Light> = Vec::new();
    for i in 1..20 {
        for j in -6..6 {
            lights.push(
                Light::new(
                    Vector3::new(3. * j as f64, 50f64, 3. * i as f64),
                    Vector3::new(1f64, 1f64, 1f64)));
        }
    }

    let width = 1600;
    let height = 1200;

    // get a spherical view
    let view = spherical_view(
        Vector3::new(1f64, 0f64, 0f64),
        Vector3::new(0f64, 1f64, 0.2f64),
        std::f64::consts::PI * 1. / 4.,
        width,
        height,
    );

    // calculate the colors of each pixel in the view
    let pixel_colors = trace_all(
        view,
        Vector3::new(20f64, 30f64, -80f64),
        objects,
        lights,
    );

    // write the pixel colors to an image
    let mut imgbuf = image::ImageBuffer::new(width, height);
    for (x, y, color_vec) in &pixel_colors {
        let color = Rgb([
            (255. * color_vec[0]) as u8,
            (255. * color_vec[1]) as u8,
            (255. * color_vec[2]) as u8
        ]);
        imgbuf.put_pixel(*x, *y, color);
    }
    imgbuf.save("out.png").unwrap();
}


