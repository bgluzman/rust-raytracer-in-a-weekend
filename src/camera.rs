use vec3::{ElemT, Vec3};
use ray::Ray;

use std::f64::consts;

pub struct Camera<T: ElemT> {
    origin: Vec3<T>,
    lower_left_corner: Vec3<T>,
    horizontal: Vec3<T>,
    vertical: Vec3<T>
}

impl<T: ElemT> Camera<T> {
    pub fn new(vfov: T, aspect: T) -> Camera<T> {
        let two = T::from_f64(2.).unwrap();
        let theta = vfov * T::from_f64(consts::PI/180.).unwrap();
        let half_height = (theta / two).tan();
        let half_width = aspect * half_height;
        Camera::<T> {
            lower_left_corner: Vec3::new(-half_width, -half_height, -T::one()),
            horizontal: Vec3::new(two * half_width, T::zero(), T::zero()),
            vertical: Vec3::new(T::zero(), two * half_height, T::zero()),
            origin: Vec3::default()
        }
    }

    pub fn get_ray(&self, u: T, v: T) -> Ray<T> {
        Ray::<T>::new(self.origin.clone(),
                      &self.lower_left_corner
                          + &(&self.horizontal*u + &self.vertical*v)
                          - &self.origin)
    }
}
