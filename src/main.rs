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
mod dielectric;

use std::f64;
use rand::Rng;

type Vec3 = vec3::Vec3<f64>;
type Ray = ray::Ray<f64>;
type Camera = camera::Camera<f64>;
type Sphere = sphere::Sphere<f64>;
type Hitable = hitable::Hitable<f64>;
type HitableList = hitablelist::HitableList<f64>;
type Lambertian = lambertian::Lambertian<f64>;
type Metal = metal::Metal<f64>;
type Dielectric = dielectric::Dielectric<f64>;

fn color(r: &Ray, world: &Hitable, depth: i32) -> Vec3 {
    if let Some(rec) = world.hit(&r, 0.001, f64::MAX) {
        if depth >= 50 { // stop recursion
            return Vec3::new(0., 0., 0.);
        }

        if let Some((attenuated, scattered)) = rec.clone().mat_opt.unwrap().scatter(r, &rec) {
            attenuated * color(&scattered, world, depth+1)
        }
        else {
            Vec3::new(0., 0., 0.)
        }
    }
    else {
        let unit_direction = r.direction().unit_vector();
        let t = 0.5*(unit_direction.y() + 1.);
        return (1.-t)*Vec3::new(1., 1., 1.) + t*Vec3::new(0.5, 0.7, 1.0);
    }
}

fn random_scene() -> HitableList {
    let mut rng = rand::thread_rng();

    let mut list = Vec::<Box<Hitable>>::new();
    list.push(Box::new(Sphere::new(Vec3::new(0., -1000., 0.), 1000., Box::new(Lambertian::new(Vec3::new(0.5, 0.5, 0.5))))));
    for a in -11..12 { // TODO: better way to write inclusive?
        for b in -11..12 {
            let a = a as f64;
            let b = b as f64;
            let choose_mat = rng.next_f64();
            let center = Vec3::new(a+0.9*rng.next_f64(), 0.2, b+0.9*rng.next_f64());
            if (&center - Vec3::new(4., 0.2, 0.)).length() > 0.9 {
                if choose_mat < 0.8 { // diffuse
                    let r1 = rng.next_f64();
                    let r2 = rng.next_f64();
                    let r3 = rng.next_f64();
                    let r4 = rng.next_f64();
                    let r5 = rng.next_f64();
                    let r6 = rng.next_f64();
                    list.push(Box::new(Sphere::new(center, 0.2, Box::new(Lambertian::new(Vec3::new(r1*r2, r3*r4, r5*r6))))));
                }
                else if choose_mat < 0.95 { // metal
                    let r1 = rng.next_f64();
                    let r2 = rng.next_f64();
                    let r3 = rng.next_f64();
                    let r4 = rng.next_f64();
                    list.push(Box::new(Sphere::new(center, 0.2, Box::new(Metal::new(Vec3::new(0.5*(1. + r1), 0.5*(1.+r2), 0.5*(1.+r3)), 0.5*r4)))));
                }
                else { // glass
                    list.push(Box::new(Sphere::new(center, 0.2, Box::new(Dielectric::new(1.5)))));
                }
            }
        }
    }
    list.push(Box::new(Sphere::new(Vec3::new(0., 1., 0.), 1., Box::new(Dielectric::new(1.5)))));
    list.push(Box::new(Sphere::new(Vec3::new(-4., 1., 0.), 1., Box::new(Lambertian::new(Vec3::new(0.4, 0.2, 0.1)))))); 
    list.push(Box::new(Sphere::new(Vec3::new(4., 1., 0.), 1., Box::new(Metal::new(Vec3::new(0.7, 0.6, 0.5), 0.))))); 

    HitableList::new(list)
}

fn main() {
	let nx = 1200;
	let ny = 800;
    let ns = 10;

    println!("P3\n {} {} \n255", nx, ny);

    let world = random_scene();

    let lookfrom = Vec3::new(13.,2.,3.);
    let lookat = Vec3::new(0.,0.,0.);
    // let dist_to_focus = (&lookfrom-&lookat).length();
    let dist_to_focus = 10.;
    let aperture = 0.1;
    let cam = Camera::new(lookfrom, lookat,
                          Vec3::new(0., 1., 0.),
                          20., (nx as f64) / (ny as f64),
                          aperture, dist_to_focus);

    let mut rng = rand::thread_rng();
    for j in (0..ny).rev() {
        for i in 0..nx {
            let mut col = Vec3::default();
            for _ in 0..ns {
                let u = (i as f64 + rng.next_f64()) / (nx as f64);
                let v = (j as f64 + rng.next_f64()) / (ny as f64);
                let r = cam.get_ray(u, v);
                col += color(&r, &world, 0);
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
