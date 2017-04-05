use vec3::{ElemT, Vec3};
use ray::Ray;
use hitable::{HitRecord, Hitable};
use material::Material;

pub struct Sphere<T: ElemT> {
    center: Vec3<T>,
    radius: T,
    material: Box<Material<T>>
}

impl<T: ElemT> Sphere<T> {
    pub fn new(cen: Vec3<T>, r: T, mat: Box<Material<T>>) -> Sphere<T> {
        Sphere {
            center: cen,
            radius: r,
            material: mat
        }
    }

    fn material(&self) -> &Box<Material<T>> {
        &self.material
    }
}

impl<T: ElemT> Hitable<T> for Sphere<T> {
    fn hit(&self, r: &Ray<T>, t_min: T, t_max: T) -> Option<HitRecord<T>>
{
        let oc = r.origin() - &self.center;
        let a = r.direction().dot(&r.direction());
        let b = oc.dot(&r.direction());
        let c = oc.dot(&oc) - self.radius*self.radius;
        let d = b*b - a*c;
        if d > T::zero() {
            let mut temp = (-b - (b*b - a*c).sqrt())/a;
            if temp < t_max && temp > t_min {
                let mut rec = HitRecord::<T>::default();
                rec.t = temp;
                rec.p = r.point_at_parameter(rec.t);
                rec.normal = &(&rec.p - &self.center) / self.radius;
                rec.mat = Some(&*self.material);
                return Some(rec);
            }
            temp = (-b + (b*b - a*c).sqrt())/a;
            if temp < t_max && temp > t_min {
                let mut rec = HitRecord::<T>::default();
                rec.t = temp;
                rec.p = r.point_at_parameter(rec.t);
                rec.normal = &(&rec.p - &self.center) / self.radius;
                rec.mat = Some(&*self.material);
                return Some(rec);
            }
        }
        None
    }
}
