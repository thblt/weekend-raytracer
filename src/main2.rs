use lib::*;
use std::f64;

fn ray_color(r: &Ray) -> Color {
    let unit_direction = r.direction.unit_vector();
    let t = 0.5 * (unit_direction.y + 1.0);
    let white = Color::new(1.0, 1.0, 1.0);
    let blue = Color::new(0.3, 0.5, 1.0);
    (1.0-t)*white + t*blue
}

fn main() {
    println!("Raytraced background.");

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

    // Where the camera is?
    let origin = Point3::new(0.0, 0.0, 0.0);

    // Vector to what?  Rightmost horizontal position?
    let horizontal = Vec3::new(viewport_width, 0.0, 0.0);
    // Vector to what? 2
    let vertical = Vec3::new(0.0, viewport_height, 0.0);

    let lower_left_corner = origin - horizontal/2.0 - vertical/2.0 - Vec3::new(0.0, 0.0, focal_length);

    // Render

    for y in 0..image_height { //
        for x in 0..image_width {
            // u and v are width and height as part of the total image height.
            let u = x as f64 / (image_width as f64 - 1.0);
            let v = y as f64 / (image_height as f64 - 1.0);
            // The ray is the ray pointing to the lowest left corner +
            // a part of horizontal + a part of vertical (minus
            // origin, but here the origin is 0,0,0)
            let ray = Ray::new(origin, lower_left_corner + u*horizontal + v*vertical - origin);
            let pixel_color = ray_color(&ray);
            // We reverse the y position because our reference point
            // is the lower-left corner, but we encode image top to
            // bottom.
            image[(x, image_height-y-1)] = pixel_color
        }
    }

    let _ = image.write_ppm("image2.ppm");
}
