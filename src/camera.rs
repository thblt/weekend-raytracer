use crate::{Color, Hittable, Image, Interval, Point3, Ray, Vec3};

pub struct Camera {
    // /// Ratio of image width over height
    // aspect_ratio: f64,
    // ^ We don't initialize the struct the same way the book
    // initalizes the class, so we don't need this.
    /// Rendered image width in pixel count
    image_width: usize,
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
    pub fn new(image_width: usize, aspect_ratio: f64) -> Self {
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
                let pixel_center =
                    self.pixel00_loc + (i as f64 * self.pixel_delta_u) + (j as f64 * self.pixel_delta_v);
                let ray_direction = pixel_center - self.center;
                let ray = Ray::new(self.center, ray_direction);
                let pixel_color = Camera::ray_color(&ray, world);
                image[(i, j)] = pixel_color
            }
        }
        image
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
}
