#[allow(unused_imports)]
#[macro_use]
extern crate assert_approx_eq;

mod vec3;
mod ray;

use vec3::unit_vector;

type Vec3 = vec3::Vec3<f64>;
type Ray = ray::Ray<f64>;

fn hit_sphere(center: &Vec3, radius: f64, r: &Ray) -> f64 {
    let oc = r.origin() - center;
    let a = r.direction().dot(&r.direction());
    let b = 2.0 * oc.dot(&r.direction());
    let c = oc.dot(&oc) - radius * radius;
    let d = b*b - 4.0*a*c;
    if d < 0.0 { -1.0 } else { (-b - d.sqrt()) / (2.0 * a) }
}

fn color(r: &Ray) -> Vec3 {
    let t = hit_sphere(&Vec3::new(0.0, 0.0, -1.0), 0.5, &r);
    if t > 0.0 {
        let ref n0 = r.point_at_parameter(t) - Vec3::new(0.0, 0.0, -1.0);
        let n = unit_vector(n0);
        Vec3::new(n.x() + 1.0, n.y() + 1.0, n.z() + 1.0) * 0.5
    }
    else {
        let unit_direction = r.direction().unit_vector();
        let t = 0.5*(unit_direction.y() + 1.0);
        (1.0 - t)*Vec3::new(1.0, 1.0, 1.0) + t*Vec3::new(0.5, 0.7, 1.0)
    }
}

fn main() {
	let nx = 200;
	let ny = 100;
    println!("P3\n {} {} \n255", nx, ny);
    let lower_left_corner = Vec3::new(-2.0, -1.0, -1.0);
    let horizontal = Vec3::new(4.0, 0.0, 0.0);
    let vertical   = Vec3::new(0.0, 2.0, 0.0);
    let origin     = Vec3::new(0.0, 0.0, 0.0);
    for j in (0..ny).rev() {
        for i in 0..nx {
            let u = i as f64 / nx as f64;
            let v = j as f64 / ny as f64;
            let p = &lower_left_corner + &horizontal*u + &vertical*v;
            let r = Ray::new(&origin, &p);
            let col = color(&r);
            let col_scaled = 255.99 * col;
            let (ir, ig, ib) = (col_scaled.r(), col_scaled.g(), col_scaled.b());
            println!("{} {} {}", ir as i32, ig as i32, ib as i32);
        }
    }
}
