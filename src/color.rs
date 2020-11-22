
use crate::vec3::Vec3;

pub fn write_color(pixel_color : &Vec3) {
    let ir : f64 = (255.999 * pixel_color[0]).floor();
    let ig : f64 = (255.999 * pixel_color[1]).floor();
    let ib : f64 = (255.999 * pixel_color[2]).floor();

    println!("{} {} {}", ir, ig, ib);
}