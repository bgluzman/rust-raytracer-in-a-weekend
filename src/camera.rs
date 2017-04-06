extern crate rand;

use vec3::{ElemT, Vec3};
use ray::Ray;

use std::f64::consts;
use rand::Rng;

pub struct Camera<T: ElemT> {
    origin: Vec3<T>,
    lower_left_corner: Vec3<T>,
    horizontal: Vec3<T>,
    vertical: Vec3<T>,
    u: Vec3<T>,
    v: Vec3<T>,
    lens_radius: T
}

fn random_in_unit_disk<T: ElemT>() -> Vec3<T> {
    let mut rng = rand::thread_rng();
    let two = T::from_f64(2.).unwrap();
    loop {
        let r1 = T::from_f64(rng.next_f64()).unwrap();
        let r2 = T::from_f64(rng.next_f64()).unwrap();
        let p = Vec3::new(r1, r2, T::zero())*two - Vec3::new(T::one(), T::one(), T::zero());
        if p.dot(&p) < T::one() { return p; }
    }
}

impl<T: ElemT> Camera<T> {
    pub fn new(lookfrom: Vec3<T>, lookat: Vec3<T>, vup: Vec3<T>, vfov: T, aspect: T, aperature: T, focus_dist: T) -> Camera<T> {
        let two = T::from_f64(2.).unwrap();

        let lens_radius = aperature / two;
        let theta = vfov * T::from_f64(consts::PI/180.).unwrap();
        let half_height = (theta / two).tan();
        let half_width = aspect * half_height;
        let w = (&lookfrom - &lookat).unit_vector();
        let u = vup.cross(&w).unit_vector();
        let v = w.cross(&u);
        Camera::<T> {
            lower_left_corner: &lookfrom - &u*focus_dist*half_width - &v*focus_dist*half_height - &w*focus_dist,
            horizontal: &u*half_width*focus_dist*two,
            vertical: &v*half_height*focus_dist*two,
            u: u,
            v: v,
            lens_radius: lens_radius,
            origin: lookfrom
        }
    }

    pub fn get_ray(&self, s: T, t: T) -> Ray<T> {
        let rd = random_in_unit_disk()*self.lens_radius;
        let offset = &self.u * rd.x() + &self.v * rd.y();
        Ray::<T>::new(&self.origin + &offset,
                      &self.lower_left_corner
                          + &self.horizontal*s
                          + &self.vertical*t
                          - &self.origin
                          - offset)
    }
}
