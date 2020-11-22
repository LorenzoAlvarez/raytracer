mod vec3;
mod color;
mod ray;




fn main() {

    let ASPECT_RATIO :f64 = 16.0 / 9.0;
    let IMAGE_WIDTH : u32 = 400;
    let IMAGE_HEIGHT : u32 = (IMAGE_WIDTH as f64 / ASPECT_RATIO).floor() as u32;

    // Camera
    let viewport_height = 2.0;
    let viewport_width = ASPECT_RATIO * viewport_height;
    let focal_length = 1.0;

    let origin = vec3::Vec3::new();
    let horizontal = vec3::Vec3::with_values(viewport_width, 0.0, 0.0);
    let vertical =vec3::Vec3::with_values(0.0,viewport_height, 0.0);
    let lower_left_corner = origin - horizontal/2.0 - vertical/2.0 - vec3::Vec3::with_values(0.0, 0.0, focal_length);

    //Renderer
    println!("P3");
    println!("{} {}", IMAGE_WIDTH, IMAGE_HEIGHT);
    println!("255");

    for j in (0..IMAGE_HEIGHT).rev() {
        for i in 0..IMAGE_WIDTH {
            let u: f64 = i as f64 / (IMAGE_WIDTH-1) as f64;
            let v: f64 = j as f64 / (IMAGE_HEIGHT-1) as f64;
            
            let r = ray::Ray::from(origin, lower_left_corner + u*horizontal + v*vertical - origin);

            let pixel_color = color::ray_color(&r);
            color::write_color(&pixel_color);
        }
    }
    
}
