mod vec3;
mod color;

const IMAGE_WIDTH : u32 = 256;
const IMAGE_HEIGHT : u32 = 256;

fn main() {
    //Renderer
    println!("P3");
    println!("{} {}", IMAGE_WIDTH, IMAGE_WIDTH);
    println!("255");

    for j in (0..IMAGE_HEIGHT).rev() {
        for i in 0..IMAGE_HEIGHT {
            let pixel_color : vec3::Vec3 = vec3::Vec3::with_values(
                i as f64 / (IMAGE_WIDTH-1) as f64,
                j as f64 / (IMAGE_HEIGHT-1) as f64,
                0.25,
            );
            color::write_color(&pixel_color);
        }
    }
    
}
