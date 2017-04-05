#[allow(unused_imports)]
#[macro_use]
extern crate assert_approx_eq;
extern crate num;
extern crate num_traits;
extern crate rand;

mod vec3;
mod ray;
mod hitable;
mod sphere;
mod hitablelist;
mod camera;
mod material;
mod lambertian;
mod metal;

use std::f64;
use rand::Rng;
use camera::Camera;

type Vec3 = vec3::Vec3<f64>;
type Ray = ray::Ray<f64>;
type Sphere = sphere::Sphere<f64>;
type Hitable = hitable::Hitable<f64>;
type HitRecord = hitable::HitRecord<f64>;
type HitableList = hitablelist::HitableList<f64>;
type Lambertian = lambertian::Lambertian<f64>;
type Metal = metal::Metal<f64>;

fn random_in_unit_sphere() -> Vec3 {
    let mut rng = rand::thread_rng();

    let mut p;
    loop {
        p = 2.0*Vec3::new(rng.next_f64(), rng.next_f64(), rng.next_f64())
                - Vec3::new(1., 1., 1.);
        if p.squared_length() >= 1. { break; }
    }
    p
}

fn color(r: &Ray, world: &Hitable) -> Vec3 {
    let mut rec = HitRecord::default();
    if world.hit(&r, 0.001, f64::MAX, &mut rec) {
        // 0.5 * Vec3::new(rec.normal.x()+1., rec.normal.y()+1., rec.normal.z()+1.)
        let target = &rec.p + &rec.normal + &random_in_unit_sphere();
        0.5*color(&Ray::new(rec.p.clone(), &target-&rec.p), world)
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
    let ns = 100;

    println!("P3\n {} {} \n255", nx, ny);

    let list: Vec<Box<Hitable>> = vec![
        // Box::new(Sphere::new(Vec3::new(0., 0., -1.), 0.5)),
        Box::new(Sphere::new(Vec3::new(0., -100.5, -1.), 100., Box::new(Lambertian::new(Vec3::new(0.8, 0.3, 0.3)))))
    ];
    let world = HitableList::new(list);

    let cam = Camera::new();

    let mut rng = rand::thread_rng();
    for j in (0..ny).rev() {
        for i in 0..nx {
            let mut col = Vec3::default();
            for _ in 0..ns {
                let u = (i as f64 + rng.next_f64()) / (nx as f64);
                let v = (j as f64 + rng.next_f64()) / (ny as f64);
                let r = cam.get_ray(u, v);
                col += color(&r, &world);
            }
            col /= ns as f64;
            col = Vec3::new(col.x().sqrt(), col.y().sqrt(), col.z().sqrt());
            let (ir, ig, ib) = ((255.99*col.x()) as i32,
                                (255.99*col.y()) as i32,
                                (255.99*col.z()) as i32);
            println!("{} {} {}", ir, ig, ib);
        }
    }
}
