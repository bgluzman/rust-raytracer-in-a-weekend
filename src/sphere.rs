use vec3::{ElemT, Vec3};
use ray::Ray;
use hitable::{HitRecord, Hitable};

pub struct Sphere<T: ElemT> {
    center: Vec3<T>,
    radius: T
}

impl<T: ElemT> Sphere<T> {
    #[allow(dead_code)]
    pub fn default() -> Sphere<T> {
        Sphere {
            center: Vec3::new(T::zero(), T::zero(), T::zero()),
            radius: T::zero()
        }
    }

    pub fn new(cen: Vec3<T>, r: T) -> Sphere<T> {
        Sphere {
            center: cen,
            radius: r
        }
    }
}

impl<T: ElemT> Hitable<T> for Sphere<T> {
    fn hit(&self, r: &Ray<T>, t_min: T, t_max: T, rec: &mut HitRecord<T>) -> bool {
        let oc = r.origin() - &self.center;
        let a = r.direction().dot(&r.direction());
        let b = oc.dot(&r.direction());
        let c = oc.dot(&oc) - self.radius*self.radius;
        let d = b*b - a*c;
        if d > T::zero() {
            let mut temp = (-b - (b*b - a*c).sqrt())/a;
            if temp < t_max && temp > t_min {
                rec.t = temp;
                rec.p = r.point_at_parameter(rec.t);
                rec.normal = &(&rec.p - &self.center) / self.radius;
                return true;
            }
            temp = (-b + (b*b - a*c).sqrt())/a;
            if temp < t_max && temp > t_min {
                rec.t = temp;
                rec.p = r.point_at_parameter(rec.t);
                rec.normal = &(&rec.p - &self.center) / self.radius;
                return true;
            }
        }
        false
    }
}
