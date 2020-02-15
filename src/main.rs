use crate::library::object::{sphere::Sphere, Object};
use nalgebra::Vector3;
use crate::library::tracer::{trace_all, spherical_view, planar_view};
use crate::library::light::{PointLight, Lighting};
use crate::library::object::plane::Plane;
use image::{Rgb, GenericImageView};

mod library;


fn main() {
    let skybox = image::open("skybox.png").unwrap();

    let mut objects: Vec<Box<dyn Object + Sync>> = Vec::new();
    let mut s1 = Sphere::new(Vector3::new(-15f64, -20f64, 100f64), 20.);
    let mut s2 = Sphere::new(Vector3::new(60f64, -25f64, 150f64), 15.);
    let mut s3 = Sphere::new(Vector3::new(25f64, -30f64, 75f64), 10.);
    s2.set_properties(Vector3::new(0.03, 0.03, 0.03), Vector3::new(0., 0.78, 0.34));
    s1.set_properties(Vector3::new(0.7, 0.7, 0.7), Vector3::new(0., 0., 0.));
    s3.set_properties(Vector3::new(0.08, 0.08, 0.08), Vector3::new(1., 0.7, 0.2));
    objects.push(Box::new(s1));
    objects.push(Box::new(s2));
    objects.push(Box::new(s3));

    let mut p = Plane::new(
        Vector3::new(0f64, -40f64, 0f64),
        Vector3::new(0f64, 1f64, 0f64)
    );
    p.set_properties(
        Vector3::new(0.06f64, 0.06f64, 0.06f64),
        Vector3::new(0.6f64, 0.6f64, 0.6f64),
    );

    objects.push(Box::new(p));


    let mut lights = Lighting::new(0.3);
    // local lighting around the scene
    for i in 1..10 {
        for j in -3..3 {
            lights.add_point(
                    Vector3::new(6. * j as f64, 50f64, 6. * i as f64),
                    Vector3::new(1f64, 0f64, 0f64)
            );
        }
    }
    // global lighting
    for i in -4..=4 {
        for j in 1..6 {
            for k in -4..=4 {
                lights.add_global(
                    Vector3::new(0.05 * i as f64, -0.1 * j as f64, 0.05 * k as f64),
                    Vector3::new(1f64, 1f64, 1f64),
                )
            }
        }
    }

    let width = 3000;
    let height = 2000;

    // get a view
    let view = planar_view(
        Vector3::new(1f64, 0f64, 0f64),
        Vector3::new(0f64, 1f64, 0.2f64),
        std::f64::consts::PI * 1. / 4.,
        width,
        height,
    );

    // calculate the colors of each pixel in the view
    let pixel_colors = trace_all(
        view,
        Vector3::new(0f64, 10f64, -80f64),
        objects,
        lights,
        &skybox,
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


