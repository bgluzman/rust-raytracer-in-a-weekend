use vec3::{ElemT, Vec3};
use ray::Ray;
use material::Material;

#[derive(Clone)]
pub struct HitRecord<'a, T>
    where T: ElemT + 'a {
    pub t: T,
    pub p: Vec3<T>,
    pub normal: Vec3<T>,
    pub mat: Option<&'a Material<T>>
}

impl<'a, T> Default for HitRecord<'a, T>
    where T: ElemT + 'a {
    fn default() -> HitRecord<'a, T> {
        HitRecord::<T> {
            t: T::zero(),
            p: Vec3::<T>::new(T::zero(), T::zero(), T::zero()),
            normal: Vec3::<T>::new(T::zero(), T::zero(), T::zero()),
            mat: None
        }
    }
}

pub trait Hitable<'a, T>
    where T: ElemT + 'a {
    fn hit(&'a self, r: &Ray<T>, t_min: T, t_max: T) -> Option<HitRecord<'a, T>>;
}
