use vec3::{ElemT, Vec3};
use ray::Ray;
use material::Material;

#[derive(Clone)]
pub struct HitRecord<'a, T: ElemT + 'a> {
    pub t: T,
    pub p: Vec3<T>,
    pub normal: Vec3<T>,
    pub mat_opt: Option<&'a Material<T>>
}

impl<'a, T: ElemT + 'a> Default for HitRecord<'a, T> {
    fn default() -> HitRecord<'a, T> {
        HitRecord::<'a, T> {
            t: T::zero(),
            p: Vec3::<T>::new(T::zero(), T::zero(), T::zero()),
            normal: Vec3::<T>::new(T::zero(), T::zero(), T::zero()),
            mat_opt: None
        }
    }
}

pub trait Hitable<T>
    where T: ElemT {
    fn hit(&self, r: &Ray<T>, t_min: T, t_max: T) -> Option<HitRecord<T>>;
}
