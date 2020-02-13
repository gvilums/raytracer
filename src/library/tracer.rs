use crate::library::object::{Ray, Object};
use crate::library::light::Light;
use nalgebra::{Vector3, Rotation3, Unit};
use rayon::prelude::*;

/// returns the color to be assigned to @param ray
pub fn trace(
    ray: &Ray,
    scene: &Vec<Box<dyn Object + Sync>>,
    bounce_limit: usize,
    lights: &Vec<Light>,
) -> Vector3<f64> {
    let res = find_closest_intersect(ray, scene);
    if let Some((intersect, normal, obj)) = res {
        // reflection color, calculated only if the object has reflective properties
//        let reflect_color =if bounce_limit > 0 {
//            let out_dir =
//                Rotation3::from_axis_angle(&normal, std::f64::consts::PI) * ray.dir.scale(-1.);
//            let out_ray = Ray::new(intersect, out_dir);
//            Some(trace(&out_ray, scene, bounce_limit - 1, lights))
//        } else { None };

        // diffuse color
        let brightness: f64 = lights.par_iter().map(|light| {
            let l_ray = Ray::new(intersect, light.pos - intersect);
            let mut blocked = false;
            for obj in scene {
                if let Some(_) = obj.intersects(&l_ray) {
                    blocked = true;
                    break;
                }
            }
            if blocked { 0. } else { light.brightness }
        }).sum();
        let potential: f64 = lights.par_iter().map(|light| light.brightness).sum();
        let opaque_col = obj.properties().color.scale(0.3 + 0.7 * brightness / potential);

//        if let Some(ref_col) = reflect_color {
//            opaque_col.scale(obj.properties().light_interaction[0])
//                + ref_col.scale(obj.properties().light_interaction[1])
//        } else {
            opaque_col
//        }
    } else {
        // no intersects, ray is sky color
        Vector3::new(110f64, 181f64, 190f64).scale(1. / 255.)
    }
}

/// Returns the closest point of intersection between @param ray and an object in @param scene
/// which is the closest to the ray's origin, ignoring any intersect which is very close to the
/// ray's origin
fn find_closest_intersect<'a>(
    ray: &Ray,
    scene: &'a Vec<Box<dyn Object + Sync>>
) -> Option<(Vector3<f64>, Unit<Vector3<f64>>, &'a Box<dyn Object + Sync>)> {
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
    pixel_vectors: Vec<(u32, u32, Vector3<f64>)>,
    pos: Vector3<f64>,
    scene: Vec<Box<dyn Object + Sync>>,
    lights: Vec<Light>,
) -> Vec<(u32, u32, Vector3<f64>)> {
    pixel_vectors
        .par_iter()
        .map(|(x, y, v)| {
            let ray = Ray::new(pos, *v);
            (*x, *y, trace(&ray, &scene, 5, &lights))
        })
        .collect()
}

/// generates a view based on a sub-sphere of vision
pub fn spherical_view(
    view_horizontal: Vector3<f64>,
    view_vertical: Vector3<f64>,
    fov: f64,
    width: u32,
    height: u32
) -> Vec<(u32, u32, Vector3<f64>)> {
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

    // the horizontal rotation matrix for moving the scan along the current scan line
    let h_rot_mat = Rotation3::from_axis_angle(&v_unit, fov_horizontal / (width - 1) as f64);

    // calculate the view vector for each pixel
    let mut pixel_vectors: Vec<(u32, u32, Vector3<f64>)> = Vec::new();
    for y in 0..height {
        let mut scan = curr_scan_start;
        for x in 0..width {
            pixel_vectors.push((x, y, scan));
            scan = h_rot_mat * scan;
        }
        curr_scan_start = v_rot_mat * curr_scan_start;
    }
    pixel_vectors
}