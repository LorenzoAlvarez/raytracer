
use crate::vec3::Vec3;
use crate::ray::Ray;

pub fn write_color(pixel_color : &Vec3) {
    let ir : f64 = (255.999 * pixel_color[0]).floor();
    let ig : f64 = (255.999 * pixel_color[1]).floor();
    let ib : f64 = (255.999 * pixel_color[2]).floor();

    println!("{} {} {}", ir, ig, ib);
}

pub fn ray_color(ray : &Ray) -> Vec3 {
    let unit_direction = Vec3::unit_vector(&ray.dir);
    let t = 0.5*(unit_direction[1] + 1.0);
    
    (1.0 - t)*Vec3::with_values(1.0, 1.0, 1.0) + t*Vec3::with_values(0.5, 0.7, 1.0)
}