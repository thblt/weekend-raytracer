use crate::{Color, Hittable, Image, Interval, Point3, Ray, Vec3};
use rand::prelude::*;

pub struct Camera {
    // /// Ratio of image width over height
    // aspect_ratio: f64,
    // ^ We don't initialize the struct the same way the book
    // initalizes the class, so we don't need this.
    /// Rendered image width in pixel count
    image_width: usize,
    // Count of random samples for each pixel
    samples_per_pixel: usize,
    /// Rendered image height
    image_height: usize,
    /// Camera center
    center: Point3,
    /// Location of pixel 0, 0
    pixel00_loc: Point3,
    /// Offset to pixel to the right
    pixel_delta_u: Vec3,
    /// Offset to pixel below
    pixel_delta_v: Vec3,
}

impl Camera {
    pub fn new(image_width: usize, aspect_ratio: f64, samples_per_pixel: usize) -> Self {
        let image_height = (image_width as f64 / aspect_ratio) as usize;

        // Camera
        let viewport_height = 2.0;
        let viewport_width = aspect_ratio * viewport_height;
        let focal_length = 1.0;

        let viewport_u = Vec3::new(viewport_width, 0.0, 0.0);
        let viewport_v = Vec3::new(0.0, -viewport_height, 0.0);

        let pixel_delta_u = viewport_u / image_width as f64;
        let pixel_delta_v = viewport_v / image_height as f64;

        // Calculate the location of the upper left pixel.
        let center = Point3::zero();
        let viewport_upper_left =
            center - Vec3::new(0.0, 0.0, focal_length) - viewport_u / 2.0 - viewport_v / 2.0;

        Camera {
            image_width,
            image_height,
            samples_per_pixel,
            center,
            pixel00_loc: viewport_upper_left + 0.5 * (pixel_delta_u + pixel_delta_v),
            pixel_delta_u,
            pixel_delta_v,
        }
    }

    pub fn render<T: Hittable>(&self, world: &T) -> Image {
        let mut image = Image::new(self.image_width, self.image_height);

        for j in 0..self.image_height {
            for i in 0..self.image_width {
                let mut pixel_color = Vec3::default();
                for _ in 0..self.samples_per_pixel {
                    let ray = self.get_ray(i,j);
                    pixel_color += Camera::ray_color(&ray, world);
                }
                image[(i, j)] = pixel_color
            }
        }
        image.normalize(self.samples_per_pixel);
        image
    }

    fn get_ray(&self, i: usize, j: usize) -> Ray {
        let pixel_center =
            self.pixel00_loc + (i as f64 * self.pixel_delta_u) + (j as f64 * self.pixel_delta_v);
        let pixel_sample = pixel_center + self.pixel_sample_square();
        let ray_direction = pixel_sample - self.center;
        Ray::new(self.center, ray_direction)
    }

    fn ray_color<T: Hittable>(ray: &Ray, world: &T) -> Color {
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

    fn pixel_sample_square(&self) -> Vec3 {
        let mut rng = rand::thread_rng();

        let px: f64 = -0.5 + rng.gen::<f64>();
        let py: f64 = -0.5 + rng.gen::<f64>();
        (px * self.pixel_delta_u) + (py * self.pixel_delta_v)
    }


}

#[test]
fn aspect_ratio_test() {
    let cam = Camera::new(1000, 16.0/9.0);
    assert!(cam.image_height == 562);
}
