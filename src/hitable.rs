use vec3::{ElemT, Vec3};
use ray::Ray;
use material::Material;

#[derive(Clone)]
#[derive(Default)]
pub struct HitRecord<'a, T: ElemT + 'a> {
    pub t: T,
    pub p: Vec3<T>,
    pub normal: Vec3<T>,
    pub mat_opt: Option<&'a Material<T>>
}

pub trait Hitable<T>
    where T: ElemT {
    fn hit(&self, r: &Ray<T>, t_min: T, t_max: T) -> Option<HitRecord<T>>;
}
