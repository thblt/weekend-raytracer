use lib::image::Image;
use std::f64;

fn main() {
    println!("Just producing a stupid image.");

    let mut image = Image::new(1000,1000);

    for x in 0..image.width() {
        for y in 0..image.height() {
            image[(x,y)].x = x as f64 / image.width() as f64;
            image[(x,y)].y = y as f64 / image.height() as f64;
        }
    }

    let _ = image.write_ppm("image1.ppm");
}
