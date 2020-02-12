use crate::library::object::{Ray, Object};
use nalgebra::{Vector3, Rotation3, Unit};

use image::{GenericImage, GenericImageView, ImageBuffer, RgbImage, Rgb};
use crate::library::light::Light;
use rayon::prelude::*;

/// returns the color to be assigned to @param ray
///
pub fn trace(
    ray: &Ray,
    scene: &Vec<Box<dyn Object + Sync>>,
    bounce_limit: usize,
    lights: &Vec<Light>,
    log: bool,
) -> Vector3<f64> {
    let res = find_closest_intersect(ray, scene);
    if let Some((intersect, normal, obj)) = res {
        let mut brightness = 0f64;
        let mut total_brightness_pot = 0f64;

        let b: f64 = lights.par_iter().map(|light| {
            let l_ray = Ray::new(intersect, light.pos - intersect);
            let mut blocked = false;
            for obj in scene {
                if let Some((block_point, _)) = obj.intersects(&l_ray) {
                    blocked = true;
                    break;
                }
            }
            if blocked {
                0.
            } else {
                light.brightness
            }
        }).sum();
        let total_brightness: f64 = lights.iter().map(|light| light.brightness).sum();

        for light in lights {
            let l_ray = Ray::new(intersect, light.pos - intersect);
            let mut blocked = false;
            for obj in scene {
                if let Some((block_point, _)) = obj.intersects(&l_ray) {
                    blocked = true;
                    break;
                }
            }
            if !blocked {
                brightness += light.brightness;
            }

            total_brightness_pot += light.brightness;
        }
        obj.properties().color.scale(brightness / total_brightness_pot)
    } else {
        Vector3::new(1f64, 1f64, 1f64)
    }
}

/// Returns the closest point of intersection between @param ray and an object in @param scene
/// which is the closest to the ray's origin, ignoring any intersect which is very close to the
/// ray's origin
fn find_closest_intersect<'a>(
    ray: &Ray,
    scene: &'a Vec<Box<dyn Object + Sync>>
) -> Option<(Vector3<f64>, Vector3<f64>, &'a Box<dyn Object + Sync>)> {
    let mut intersect_opt = None;
    let mut normal_opt = None;
    let mut closest_obj_opt = None;
    let mut distance = std::f64::INFINITY;

    for obj in scene {
        if let Some((c_intersect, c_normal)) = obj.intersects(&ray) {
            let current_dist = (ray.origin - c_intersect).norm();
            if current_dist < distance && current_dist > 0.01 {
                intersect_opt = Some(c_intersect);
                normal_opt = Some(c_normal);
                distance = current_dist;
                closest_obj_opt = Some(&*obj);
            }
        }
    }

    if let Some(_) = intersect_opt {
        Some((intersect_opt.unwrap(), normal_opt.unwrap(), closest_obj_opt.unwrap()))
    } else {
        None
    }
}

/// returns the color for every pixel in a grid sized width * height
///
/// @param view_horizontal: The x-axis of the resulting projection
/// @param view_vertical: The y-axis of the resulting projection (note that positive y-coordinates
/// move upwards in space)
///
/// example: with the x- and y- unit vectors used as view_horizontal and view_vertical, the resulting
/// camera angle will be looking in the positive z-direction, with the x-axis extending to the left,
/// and the y-axis extending upwards
///
pub fn trace_all(
    pos: Vector3<f64>,
    view_horizontal: Vector3<f64>,
    view_vertical: Vector3<f64>,
    fov: f64, // angle in radians
    width: u32, height: u32,
    scene: Vec<Box<dyn Object + Sync>>,
    lights: Vec<Light>,
) -> () {
    let fov_horizontal = fov;
    let fov_vertical = fov * height as f64 / width as f64;

    let view_dir = view_horizontal.cross(&view_vertical);

    let v_unit = Unit::new_normalize(view_vertical);
    let h_unit = Unit::new_normalize(view_horizontal);

    // variable for keeping track of the start of the current scan line
    let mut curr_scan_start = Rotation3::from_axis_angle(&v_unit, -fov_horizontal / 2f64)
        * Rotation3::from_axis_angle(&h_unit, -fov_vertical / 2f64)
        * view_dir;

    // the vertical rotation matrix for moving the current scan line start to the next scan line
    let v_rot_mat = Rotation3::from_axis_angle(&v_unit, -fov_horizontal / 2f64)
        * Rotation3::from_axis_angle(&h_unit, fov_vertical / (height - 1) as f64)
        * Rotation3::from_axis_angle(&v_unit, fov_horizontal / 2f64);


    // temporary write to image
    let mut imgbuf = image::ImageBuffer::new(width, height);
    for pixel in imgbuf.pixels_mut() {
        *pixel = Rgb([255u8, 255u8, 255u8]);
    }

    // loop for iterating through all scan lines
    for y in 0..height {
        let mut scan_line = RayIter::new(
            curr_scan_start,
            Unit::new_normalize(view_vertical),
            fov_horizontal,
            width
        );
        curr_scan_start = v_rot_mat * curr_scan_start;

        for (x, v) in scan_line.enumerate() {
            let ray = Ray::new(pos, v);

            let color_vec = trace(&ray, &scene, 5, &lights, x == 40 && y == 25);
            let color = Rgb([
                (255. * color_vec[0]) as u8,
                (255. * color_vec[1]) as u8,
                (255. * color_vec[2]) as u8
            ]);
            imgbuf.put_pixel(x as u32, y, color);
        }
    }

    imgbuf.save("out.png").unwrap();
}

pub struct RayIter {
    mat: Rotation3<f64>,
    current: Vector3<f64>,
    count: u32,
    limit: u32,
}

impl RayIter {
    pub fn new(initial: Vector3<f64>, rot_axis: Unit<Vector3<f64>>, total_angle: f64, limit: u32) -> Self {
        let mat = Rotation3::from_axis_angle(&rot_axis, total_angle / (limit - 1) as f64);
        RayIter { mat, current: initial, count: 0, limit }
    }
}

impl Iterator for RayIter {
    type Item = Vector3<f64>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.count == self.limit {
            None
        } else {
            self.count += 1;
            let out = self.current;
            self.current = self.mat * self.current;
            Some(out)
        }
    }
}