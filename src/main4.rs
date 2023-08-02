use lib::*;
use std::f64;

fn ray_color(ray: &Ray) -> Color {
    let grad_start = Color::new(0.3, 0.3, 0.3);
    let grad_end = Color::new(0.1, 0.1, 0.1);
    let sph_center = Point3::new(0.0, 0.0, -1.0);
    let sph_radius = 0.5;

    if let Some(t) = hit_sphere(&sph_center, sph_radius, ray) {
        let n = (ray.at(t) - sph_center).unit_vector();
        0.5 * Color::new(n.x + 1.0, n.y + 1.0, n.z + 1.0)
    } else {
        let unit_direction = ray.direction.unit_vector();
        let t = 0.5 * (unit_direction.y + 1.0);
        (1.0 - t) * grad_end + t * grad_start
    }
}

/// Determine if ray r hits the sphere at `center` with `radius`.
fn hit_sphere(center: &Point3, radius: f64, ray: &Ray) -> Option<f64> {
    /* The sphere equation for the sphere of radius R is: x² + y² + z²
    = R².  Given a ray r, we want to know if there's a point on that
    ray that satisfies this equation. */
    let oc = ray.origin - *center;
    let a = ray.direction.dot(ray.direction);
    let b = 2.0 * oc.dot(ray.direction);
    let c = oc.dot(oc) - radius * radius;
    let discriminant = b * b - 4.0 * a * c;

    if discriminant >= 0.0 {
        // wat?
        Some((-b - discriminant.sqrt() ) / (2.0*a))
    } else {
        None
    }
}

fn main() {
    println!("Sphere with normals.");

    // Image

    let aspect_ratio = 16.0 / 9.0;
    let image_width = 1000;
    let image_height = (image_width as f64 / aspect_ratio) as usize;
    let mut image = Image::new(image_width, image_height);
    println!("Rendering on {}×{}", image_width, image_height);

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
            // println!("At {}, {}", j, i);
            let u = i as f64 / (image_width as f64 - 1.0);
            let v = j as f64 / (image_height as f64 - 1.0);
            let r = Ray::new(
                origin,
                lower_left_corner + u * horizontal + v * vertical - origin,
            );
            let pixel_color = ray_color(&r);
            image[(i, image_height - j - 1)] = pixel_color
        }
    }

    let _ = image.write_ppm("image4.ppm");
}
