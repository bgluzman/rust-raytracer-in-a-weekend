extern crate rand;

use vec3::{ElemT, Vec3};
use ray::Ray;
use hitable::HitRecord;
use material::Material;
use lambertian::random_in_unit_sphere;

#[derive(Clone)]
pub struct Metal<T: ElemT> {
    albedo: Vec3<T>,
    fuzz: Option<T>
}

impl<T: ElemT> Metal<T> {
    pub fn new(a: Vec3<T>, fuzz: T) -> Metal<T> {
        Metal::<T> {
            albedo: a,
            fuzz: if fuzz > T::one() { Some(T::one()) }
                  else if fuzz <= T::zero() { None }
                  else { Some(fuzz) }
        }
    }
}

pub fn reflect<T: ElemT>(v: &Vec3<T>, n: &Vec3<T>) -> Vec3<T> {
    let two = T::from_f64(2.).unwrap();
    v - n*(v.dot(n)*two)
}

impl<T: ElemT> Material<T> for Metal<T> {
    fn scatter(&self, r_in: &Ray<T>, rec: &HitRecord<T>) -> Option<(Vec3<T>, Ray<T>)> {
        let reflected = reflect(&r_in.direction().unit_vector(), &rec.normal);
        let (attenuation, scattered) = (self.albedo.clone(), Ray::new(rec.p.clone(), &reflected + &random_in_unit_sphere()*self.fuzz.unwrap_or(T::zero())));
        if scattered.direction().dot(&rec.normal) > T::zero() { Some((attenuation, scattered)) } else { None }
    }
}
