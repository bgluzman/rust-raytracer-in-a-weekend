extern crate rand;

use vec3::{ElemT, Vec3};
use ray::Ray;
use hitable::HitRecord;
use material::Material;

use rand::Rng;

#[derive(Clone)]
pub struct Lambertian<T: ElemT> {
    albedo: Vec3<T>
}

impl<T: ElemT> Lambertian<T> {
    pub fn new(a: Vec3<T>) -> Lambertian<T> {
        Lambertian::<T> {
            albedo: a
        }
    }
}

// TODO: could we make the random number generator generic (f32, f64, etc)?
fn random_in_unit_sphere<T: ElemT>() -> Vec3<T> {
    let mut rng = rand::thread_rng();

    let mut p;
    loop {
        let r1 = T::from_f64(rng.next_f64()).unwrap();
        let r2 = T::from_f64(rng.next_f64()).unwrap();
        let r3 = T::from_f64(rng.next_f64()).unwrap();
        let two = T::from_f64(2.).unwrap();
        p = Vec3::<T>::new(r1, r2, r3)*two
                - Vec3::new(T::one(), T::one(), T::one());
        if p.squared_length() >= T::one() { break; }
    }
    p
}

impl<T: ElemT + 'static> Material<T> for Lambertian<T> {
    #[allow(unused_variables)]
    fn scatter(&self,
               r_in: &Ray<T>,
               rec: &HitRecord<T>,
               attenuation: &mut Vec3<T>,
               scattered: &mut Ray<T>) -> bool {
        let target = &rec.p + &rec.normal + &random_in_unit_sphere();
        *scattered = Ray::new(rec.p.clone(), &target-&rec.p);
        *attenuation = self.albedo.clone();
        true
    }
}
