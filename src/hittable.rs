use crate::ray::Ray;
use crate::vec3::{Point3, Vec3};

#[derive(PartialEq,PartialOrd,Copy,Clone)]
pub struct Hit {
    pub p: Point3,
    pub normal: Vec3,
    pub t: f64,
    front_face: bool,
}

impl Hit {
    fn set_face_normal(&mut self, r: &Ray, outward_normal: &Vec3) {
        self.front_face = r.direction.dot(*outward_normal) < 0.0;
        self.normal = if self.front_face { *outward_normal } else {-*outward_normal} ;
    }
}

pub trait Hittable {
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<Hit>;
}

pub struct Sphere {
    center: Point3,
    radius: f64,
}

impl Sphere {
    pub fn new(center: Point3, radius: f64) -> Sphere {
        Sphere {center, radius }
    }
}

impl<T: Hittable> Hittable for Vec<T> {
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<Hit> {
        self.iter()
            .filter_map(|x| T::hit(x, ray, t_min, t_max))
            .min_by(|a,b| a.t.partial_cmp(&b.t).expect("Hit.t should compare."))
    }
}

impl Hittable for Sphere {
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<Hit> {
        let oc = ray.origin - self.center;
        let a = ray.direction.length_squared();
        let half_b = oc.dot(ray.direction);
        let c = oc.length_squared() - self.radius * self.radius;

        let discriminant = half_b * half_b - a * c;
        if discriminant < 0.0 {
            return None;
        }
        let sqrtd = discriminant.sqrt();

        // Find the nearest root that lies in the acceptable range.
        let mut root = (-half_b - sqrtd) / a;

        if root < t_min || t_max < root {
            root = (-half_b + sqrtd) / a;
            if root < t_min || t_max < root {
                return None;
            }
        }

        let point = ray.at(root);

        // @FIXME This is quite ugly.  We can probably move
        // everything, incl the call to set_face_normal, into
        // a Hit::new() constructor and avoid the `let mut`.
        let mut ret = Hit {
            t: root,
            p: point,
            normal: (point - self.center) / self.radius,
            front_face: true,
        };
        let outward_normal = (point - self.center) / self.radius;
        ret.set_face_normal(ray, &outward_normal);
        Some(ret)
    }
}
