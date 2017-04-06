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
    pub fn new(lookfrom: Vec3<T>, lookat: Vec3<T>, vup: Vec3<T>, vfov: T, aspect: T) -> Camera<T> {
        let two = T::from_f64(2.).unwrap();
        let theta = vfov * T::from_f64(consts::PI/180.).unwrap();
        let half_height = (theta / two).tan();
        let half_width = aspect * half_height;
        let w = (&lookfrom - &lookat).unit_vector();
        let u = vup.cross(&w).unit_vector();
        let v = w.cross(&u);
        Camera::<T> {
            lower_left_corner: &lookfrom - &u*half_width - &v*half_height - &w,
            horizontal: &u*half_width*two,
            vertical: &v*half_height*two,
            origin: lookfrom
        }
    }

    pub fn get_ray(&self, u: T, v: T) -> Ray<T> {
        Ray::<T>::new(self.origin.clone(),
                      &self.lower_left_corner
                          + &(&self.horizontal*u + &self.vertical*v)
                          - &self.origin)
    }
}
