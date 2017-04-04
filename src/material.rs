use vec3::{ElemT, Vec3};
use ray::Ray;
use hitable::HitRecord;

pub trait Material<T: ElemT> : MaterialClone<T> {
    fn scatter(&self,
               r_in: &Ray<T>,
               rec: &HitRecord<T>,
               attenuation: &mut Vec3<T>,
               scattered: &mut Ray<T>) -> bool;
}

pub trait MaterialClone<T: ElemT> {
    fn clone_box(&self) -> Box<Material<T>>;
}

impl<T: ElemT, U> MaterialClone<T> for U where U: 'static + Material<T> + Clone {
    fn clone_box(&self) -> Box<Material<T>> {
        Box::new(self.clone())
    }
}

impl<T: ElemT> Clone for Box<Material<T>> {
    fn clone(&self) -> Box<Material<T>> {
        self.clone_box()
    }
}
