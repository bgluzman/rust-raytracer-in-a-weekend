use vec3::{ElemT, Vec3};
use ray::Ray;
use material::Material;

#[derive(Clone)]
pub struct HitRecord<T: ElemT> {
    pub t: T,
    pub p: Vec3<T>,
    pub normal: Vec3<T>,
    pub mat: Option<Box<Material<T>>>
}

impl<T: ElemT> HitRecord<T> {
    pub fn new() -> HitRecord<T> {
        HitRecord::<T> {
            t: T::zero(),
            p: Vec3::<T>::new(T::zero(), T::zero(), T::zero()),
            normal: Vec3::<T>::new(T::zero(), T::zero(), T::zero()),
            mat: None
        }
    }
}

pub trait Hitable<T: ElemT> {
    fn hit(&self, r: &Ray<T>, t_min: T, t_max: T, rec: &mut HitRecord<T>) -> bool;
}
