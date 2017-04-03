use vec3::ElemT;
use vec3::Vec3;
use ray::Ray;

#[derive(Clone)]
pub struct HitRecord<T: ElemT> {
    pub t: T,
    pub p: Vec3<T>,
    pub normal: Vec3<T>
}

impl<T: ElemT> HitRecord<T> {
    pub fn new() -> HitRecord<T> {
        HitRecord::<T> {
            t: T::zero(),
            p: Vec3::<T>::new(T::zero(), T::zero(), T::zero()),
            normal: Vec3::<T>::new(T::zero(), T::zero(), T::zero())
        }
    }
}

pub trait Hitable<T: ElemT> {
    fn hit(&self, r: &Ray<T>, t_min: T, t_max: T, rec: &mut HitRecord<T>) -> bool;
}