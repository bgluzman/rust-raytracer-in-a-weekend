extern crate rand;

use vec3::{ElemT, Vec3};
use ray::Ray;
use hitable::HitRecord;
use material::Material;
use metal::reflect;

use rand::Rng;

fn refract<T: ElemT>(v: &Vec3<T>, n: &Vec3<T>, ni_over_nt: T) -> Option<Vec3<T>> {
    let ref uv = v.unit_vector();
    let dt = uv.dot(n);
    let descriminant = T::one() - ni_over_nt.powi(2)*(T::one()-dt.powi(2));
    if descriminant > T::zero() {
        Some((uv - n*dt)*ni_over_nt - n*descriminant.sqrt())
    } else {
        None
    }
}

fn schlick<T: ElemT>(cosine: T, ref_idx: T) -> T {
    let r0 = ((T::one()-ref_idx)/(T::one()+ref_idx)).powi(5);
    r0+(T::one()-r0)*(T::one()-cosine).powi(5)
}

#[derive(Clone)]
pub struct Dielectric<T: ElemT> {
    ref_idx: T
}

impl<T: ElemT> Dielectric<T> {
    pub fn new(ri: T) -> Dielectric<T> {
        Dielectric::<T> {
            ref_idx: ri
        }
    }
}

impl<T: ElemT> Material<T> for Dielectric<T> {
    fn scatter(&self, r_in: &Ray<T>, rec: &HitRecord<T>) -> Option<(Vec3<T>, Ray<T>)> {
        let attenuation = Vec3::new(T::one(), T::one(), T::one());
        let reflected = reflect(&r_in.direction(), &rec.normal);
        let (outward_normal, ni_over_nt, cosine) = if r_in.direction().dot(&rec.normal) > T::zero() {
            (-rec.normal.clone(),
             self.ref_idx,
             self.ref_idx*r_in.direction().dot(&rec.normal) / r_in.direction().length())
        }
        else {
            (rec.normal.clone(),
             T::one() / self.ref_idx,
             -r_in.direction().dot(&rec.normal) / r_in.direction().length())
        };

        // TODO: this how the book wrote it...but i think it could be written better...
        let (refracted_opt, reflect_prob) = if let Some(refracted) = refract(&r_in.direction(), &outward_normal, ni_over_nt) { (Some(refracted), schlick(cosine, self.ref_idx)) }
        else { (None, T::one()) };
        if T::from_f64(rand::thread_rng().next_f64()).unwrap() < reflect_prob {
            Some((attenuation, Ray::new(rec.p.clone(), reflected.clone())))
        }
        else {
            Some((attenuation, Ray::new(rec.p.clone(), refracted_opt.unwrap().clone())))
        }
    }
}
