use vec3;
use ray::Ray;

type Vec3 = vec3::Vec3<f64>;

pub struct Camera {
    origin: Vec3,
    lower_left_corner: Vec3,
    horizontal: Vec3,
    vertical: Vec3
}

impl Camera {
    pub fn new() -> Camera {
        Camera {
            lower_left_corner: Vec3::new(-2., -1., -1.),
            horizontal: Vec3::new(4., 0., 0.),
            vertical: Vec3::new(0., 2., 0.),
            origin: Vec3::default()
        }
    }

    pub fn get_ray(&self, u: f64, v: f64) -> Ray<f64> {
        Ray::<f64>::new(self.origin.clone(),
                        &self.lower_left_corner
                            + &(&self.horizontal*u + &self.vertical*v)
                            - &self.origin)
    }
}
