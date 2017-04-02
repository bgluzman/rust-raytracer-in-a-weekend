#![allow(dead_code)]

use vec3::ElemT;
use vec3::Vec3;

pub struct Ray<T: ElemT> {
    a: Vec3<T>,
    b: Vec3<T>
}

impl Ray<f32> {
    pub fn default() -> Ray<f32> {
        Ray::<f32> {
            a: Vec3::<f32>::new(0.0, 0.0, 0.0),
            b: Vec3::<f32>::new(0.0, 0.0, 0.0)
        }
    }
}

impl Ray<f64> {
    pub fn default() -> Ray<f64> {
        Ray::<f64> {
            a: Vec3::<f64>::new(0.0, 0.0, 0.0),
            b: Vec3::<f64>::new(0.0, 0.0, 0.0)
        }
    }
}

impl<T: ElemT> Ray<T> {
    pub fn new(a: &Vec3<T>, b: &Vec3<T>) -> Ray<T> {
        Ray::<T> {
            a: a.clone(),
            b: b.clone()
        }
    }
    pub fn origin(&self) -> Vec3<T> { self.a.clone() }
    pub fn direction(&self) -> Vec3<T> { self.b.clone() }
    pub fn point_at_parameter(&self, t: T) -> Vec3<T> { &self.a + &self.b * t }
}
