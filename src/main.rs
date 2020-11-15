mod vec3;

const IMAGE_WIDTH : u32 = 256;
const IMAGE_HEIGHT : u32 = 256;

fn main() {
    //Renderer
    println!("P3");
    println!("{} {}", IMAGE_WIDTH, IMAGE_WIDTH);
    println!("255");

    for j in (0..IMAGE_HEIGHT).rev() {
        for i in 0..IMAGE_HEIGHT {
            let r : f64 = i as f64 / (IMAGE_WIDTH-1) as f64;
            let g : f64 = j as f64 / (IMAGE_HEIGHT-1) as f64;
            let b : f64 = 0.25;

            let ir = (255.999 * r).floor();
            let ig = (255.999 * g).floor();
            let ib = (255.999 * b).floor();

            println!("{} {} {}", ir, ig, ib);
        }
    }
    
}
