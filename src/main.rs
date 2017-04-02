#[allow(unused_imports)]
#[macro_use]
extern crate assert_approx_eq;

mod vec3;
mod ray;

type Vec3 = vec3::Vec3<f64>;
type Ray = ray::Ray<f64>;

fn color(r: Ray) -> Vec3 {
    let unit_direction = r.direction().unit_vector();
    let t = 0.5 * (unit_direction.y() + 1.0);
    Vec3::new(1.0, 1.0, 1.0) * (1.0 - t) + Vec3::new(0.5, 0.7, 1.0) * t
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
            let col = color(r);
            let col_scaled = 255.99 * col;
            let (ir, ig, ib) = (col_scaled.r(), col_scaled.g(), col_scaled.b());
            println!("{} {} {}", ir as i32, ig as i32, ib as i32);
        }
    }
}
