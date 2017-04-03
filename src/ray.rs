#![allow(dead_code)]

use vec3::ElemT;
use vec3::Vec3;

pub struct Ray<T: ElemT> {
    a: Vec3<T>,
    b: Vec3<T>
}

impl<T: ElemT> Ray<T> {
    pub fn default() -> Ray<T> {
        Ray::<T> {
            a: Vec3::<T>::default(),
            b: Vec3::<T>::default()
        }
    }

    pub fn new(a: Vec3<T>, b: Vec3<T>) -> Ray<T> {
        Ray::<T> {
            a: a,
            b: b
        }
    }
    pub fn origin(&self) -> Vec3<T> { self.a.clone() }
    pub fn direction(&self) -> Vec3<T> { self.b.clone() }
    pub fn point_at_parameter(&self, t: T) -> Vec3<T> { &self.a + &self.b * t }
}