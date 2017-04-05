extern crate rand;

use vec3::{ElemT, Vec3};
use ray::Ray;
use hitable::HitRecord;
use material::Material;

#[derive(Clone)]
pub struct Metal<T: ElemT> {
    albedo: Vec3<T>
}

impl<T: ElemT> Metal<T> {
    pub fn new(a: Vec3<T>) -> Metal<T> {
        Metal::<T> {
            albedo: a
        }
    }
}

fn reflect<T: ElemT>(v: &Vec3<T>, n: &Vec3<T>) -> Vec3<T> {
    let two = T::from_f64(2.).unwrap();
    v - n*(v.dot(n)*two)
}

impl<T: ElemT> Material<T> for Metal<T> {
    #[allow(unused_variables)]
    fn scatter(&self,
               r_in: &Ray<T>,
               rec: &HitRecord<T>,
               attenuation: &mut Vec3<T>,
               scattered: &mut Ray<T>) -> bool {
        let reflected = reflect(&r_in.direction().unit_vector(), &rec.normal);
        *scattered = Ray::new(rec.p.clone(), reflected);
        *attenuation = self.albedo.clone();
        true
    }
}
