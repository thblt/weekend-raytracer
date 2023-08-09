use lib::*;

fn main() {
    println!("Antialiasing");
    let world: Vec<Sphere> = vec![
        Sphere::new(Point3::new(0.0, 0.0, -1.0), 0.5),
        Sphere::new(Point3::new(0.0, -100.5, -1.0), 100.0),
    ];

    let cam = Camera::new(1000, 16.0 / 9.0, 10);

    let _ = cam.render(&world).write_ppm("image8.ppm");
}
