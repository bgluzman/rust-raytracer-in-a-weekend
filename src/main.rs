#[allow(unused_imports)]
#[macro_use]
extern crate assert_approx_eq;

mod vec3;
use vec3::Vec3;

fn main() {
	let nx = 200;
	let ny = 100;
    println!("P3\n {} {} \n255", nx, ny);
    
    for j in (0..ny).rev() {
        for i in 0..nx {
            let col = Vec3::<f64>::new(i as f64 / nx as f64,
                                       j as f64 / ny as f64,
                                       0.2);
            let col_scaled = 255.99 * col;
            let (ir, ig, ib) = (col_scaled.r(), col_scaled.g(), col_scaled.b());
            println!("{} {} {}", ir as i32, ig as i32, ib as i32);
        }
    }
}
