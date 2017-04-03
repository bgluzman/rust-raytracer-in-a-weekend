#[allow(unused_imports)]
#[macro_use]
extern crate assert_approx_eq;
extern crate num;
extern crate num_traits;

mod vec3;
mod ray;
mod hitable;
mod sphere;
mod hitablelist;

use std::f64;
use vec3::unit_vector;

type Vec3 = vec3::Vec3<f64>;
type Ray = ray::Ray<f64>;
type Sphere = sphere::Sphere<f64>;
type Hitable = hitable::Hitable<f64>;
type HitRecord = hitable::HitRecord<f64>;
type HitableList = hitablelist::HitableList<f64>;

// fn hit_sphere(center: &Vec3, radius: f64, r: &Ray) -> f64 {
//     let oc = r.origin() - center;
//     let a = r.direction().dot(&r.direction());
//     let b = 2.0 * oc.dot(&r.direction());
//     let c = oc.dot(&oc) - radius * radius;
//     let d = b*b - 4.0*a*c;
//     if d < 0.0 { -1.0 } else { (-b - d.sqrt()) / (2.0 * a) }
// }
// 
// fn color(r: &Ray) -> Vec3 {
//     let t = hit_sphere(&Vec3::new(0.0, 0.0, -1.0), 0.5, &r);
//     if t > 0.0 {
//         let ref n0 = r.point_at_parameter(t) - Vec3::new(0.0, 0.0, -1.0);
//         let n = unit_vector(n0);
//         return Vec3::new(n.x() + 1.0, n.y() + 1.0, n.z() + 1.0) * 0.5;
//     }
//     let unit_direction = r.direction().unit_vector();
//     let t = 0.5*(unit_direction.y() + 1.0);
//     (1.0 - t)*Vec3::new(1.0, 1.0, 1.0) + t*Vec3::new(0.5, 0.7, 1.0)
// }

fn color(r: &Ray, world: &Hitable) -> Vec3 {
    let mut rec = HitRecord::new();
    if (*world).hit(&r, 0.0, f64::MAX, &mut rec) {
        0.5 * Vec3::new(rec.normal.x()+1., rec.normal.y()+1., rec.normal.z()+1.)
    }
    else {
        let unit_direction = r.direction().unit_vector();
        let t = 0.5*(unit_direction.y() + 1.);
        return (1.-t)*Vec3::new(1., 1., 1.) + t*Vec3::new(0.5, 0.7, 1.0);
    }
}

fn main() {
	let nx = 200;
	let ny = 100;

    println!("P3\n {} {} \n255", nx, ny);

    let lower_left_corner = Vec3::new(-2.0, -1.0, -1.0);
    let horizontal = Vec3::new(4.0, 0.0, 0.0);
    let vertical   = Vec3::new(0.0, 2.0, 0.0);
    let origin     = Vec3::default();

    let list: Vec<Box<Hitable>> = vec![
        Box::new(Sphere::new(Vec3::new(0., 0., -1.), 0.5)),
        Box::new(Sphere::new(Vec3::new(0., -100.5, -1.), 100.))
    ];
    let world = HitableList::new(list);

    for j in (0..ny).rev() {
        for i in 0..nx {
            let u = i as f64 / nx as f64;
            let v = j as f64 / ny as f64;
            let r = Ray::new(origin.clone(),
                             &lower_left_corner + &horizontal*u + &vertical*v);

            let p = r.point_at_parameter(2.);
            let col = color(&r, &world);
            let col_scaled = 255.99 * col;
            let (ir, ig, ib) = (col_scaled.r(), col_scaled.g(), col_scaled.b());

            println!("{} {} {}", ir as i32, ig as i32, ib as i32);
        }
    }
}
