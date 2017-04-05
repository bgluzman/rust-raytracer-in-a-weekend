use vec3::{ElemT, Vec3};
use ray::Ray;
use hitable::HitRecord;

pub trait Material<T: ElemT> {
    fn scatter(&self, r_in: &Ray<T>, rec: &HitRecord<T>) -> Option<(Vec3<T>, Ray<T>)>;
}
