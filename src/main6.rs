use lib::*;
use std::f64;

/// Compute the color of ray given world.
fn ray_color(ray: &Ray, world: &Vec<Sphere>) -> Color {
    let white = Color::new(1.0, 1.0, 1.0);
    let grad_end = Color::new(0.5, 0.7, 1.0);

    if let Some(hit) = world.hit(ray, Interval::positive_or_null()) {
        0.5 * (hit.normal + white)
    } else {
        let unit_direction = ray.direction.unit_vector();
        let t = 0.5 * (unit_direction.y + 1.0);
        (1.0 - t) * white + t * grad_end
    }
}

/// Usually print hello world
fn main() {
    println!("A world of Sphere.");

    // Image
    let aspect_ratio = 16.0 / 9.0;
    let image_width = 1000;
    let image_height = (image_width as f64 / aspect_ratio) as usize;
    let mut image = Image::new(image_width, image_height);
    println!("Rendering on {}×{}", image_width, image_height);

    // World
    let world: Vec<Sphere> = vec!(
        Sphere::new(Point3::new(0.0,0.0,-1.0), 0.5),
        Sphere::new(Point3::new(0.0,-100.5,-1.0), 100.0),
    );

    // Camera
    let viewport_height = 2.0;
    let viewport_width = aspect_ratio * viewport_height;
    let focal_length = 1.0;

    let origin = Point3::new(0.0, 0.0, 0.0);
    let horizontal = Vec3::new(viewport_width, 0.0, 0.0);
    let vertical = Vec3::new(0.0, viewport_height, 0.0);
    let lower_left_corner =
        origin - horizontal / 2.0 - vertical / 2.0 - Vec3::new(0.0, 0.0, focal_length);

    // Render

    for j in 0..image_height {
        for i in 0..image_width {
            let u = i as f64 / (image_width as f64 - 1.0);
            let v = j as f64 / (image_height as f64 - 1.0);
            let r = Ray::new(
                origin,
                lower_left_corner + u * horizontal + v * vertical - origin,
            );
            let pixel_color = ray_color(&r, &world);
            image[(i, image_height - j - 1)] = pixel_color
        }
    }

    let _ = image.write_ppm("image6.ppm");
}
