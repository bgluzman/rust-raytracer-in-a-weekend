#![allow(dead_code)]

extern crate num;
use self::num::Float;

use std::ops::*;
use std::fmt;
use std::fmt::Display;

// Alias ElemT
pub trait ElemT : Float
                + AddAssign
                + SubAssign
                + MulAssign
                + DivAssign
                + Display {}
impl<T: Float
      + AddAssign
      + SubAssign
      + MulAssign
      + DivAssign
      + Display
      > ElemT for T {}

#[derive(Clone)]
#[derive(Debug)]
pub struct Vec3<T: ElemT> {
    vec: [T; 3]
}

const X:usize = 0;
const Y:usize = 1;
const Z:usize = 2;

impl<T: ElemT> Vec3<T> {
    pub fn new(x: T, y: T, z: T) -> Vec3<T> {
        Vec3 {
            vec: [x, y, z]
        }
    }

    // TODO: this might be complicated...do it when you need it
    // fn from_buffer<U: BufRead>(r: &U) -> Option<Vec3<T>> {
    //     let (xb, yb, zb) = (it.next(), it.next(), it.next());
    //     let
    //     Some(Vec3::new(x, 0.0, 0.0))
    // }

    pub fn from_vector(v: &Vec<T>) -> Vec3<T> {
        Vec3::new(v[0], v[1], v[2])
    }

    pub fn x(&self) -> T { self.vec[X] }
    pub fn y(&self) -> T { self.vec[Y] }
    pub fn z(&self) -> T { self.vec[Z] }
    pub fn r(&self) -> T { self.vec[X] }
    pub fn g(&self) -> T { self.vec[Y] }
    pub fn b(&self) -> T { self.vec[Z] }

    pub fn squared_length(&self) -> T {
        self.x().powi(2) + self.y().powi(2) + self.z().powi(2)
    }

    pub fn length(&self) -> T {
        self.squared_length().sqrt()
    }

    pub fn dot(&self, rhs: &Vec3<T>) -> T {
        self.x() * rhs.x() + self.y() * rhs.y() + self.z() * rhs.z()
    }

    pub fn cross(&self, rhs: &Vec3<T>) -> Vec3<T> {
        Vec3::<T>::new(self.y() * rhs.z() - self.z() * rhs.y(),
                    (-(self.x() * rhs.z() - self.z() * rhs.x())),
                       self.x() * rhs.y() - self.y() * rhs.x())
    }

    pub fn unit_vector(&self) -> Vec3<T> {
        let vnew = Vec3::new(self.x(), self.y(), self.z());
        (vnew / self.length())
    }

    pub fn make_unit_vector(&mut self) {
        let len = self.length();
        self.vec[X] /= len;
        self.vec[Y] /= len;
        self.vec[Z] /= len;
    }
}

////////////////////
// I/O Operations //
////////////////////

impl<T: ElemT> fmt::Display for Vec3<T> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{} {} {}", self.x(), self.y(), self.z())
    }
}

////////////////////
// Free Operations//
////////////////////

pub fn dot<T: ElemT>(v1: &Vec3<T>, v2: &Vec3<T>) -> T {
    v1.dot(v2)
}

pub fn cross<T: ElemT>(v1: &Vec3<T>, v2: &Vec3<T>) -> Vec3<T> {
    v1.cross(v2)
}

pub fn unit_vector<T: ElemT>(v: &Vec3<T>) -> Vec3<T> {
    v.unit_vector()
}

//////////////////////
// Unary operations //
//////////////////////

impl<T: ElemT> Neg for Vec3<T> {
    type Output = Vec3<T>;
    fn neg(self) -> Vec3<T> {
        Vec3::<T>::new(-self.x(), -self.y(), -self.z())
    }
}

impl<T: ElemT> Index<usize> for Vec3<T> {
    type Output = T;
    fn index(&self, index: usize) -> &T {
        &self.vec[index]
    }
}

impl<T: ElemT> IndexMut<usize> for Vec3<T> {
    fn index_mut(&mut self, index: usize) -> &mut T {
        &mut self.vec[index]
    }
}

/////////////////////////
// Compound operations //
/////////////////////////

impl<T: ElemT> AddAssign for Vec3<T> {
    fn add_assign(&mut self, rhs: Vec3<T>) {
        self.vec[X] += rhs.vec[X];
        self.vec[Y] += rhs.vec[Y];
        self.vec[Z] += rhs.vec[Z];
    }
}

impl<T: ElemT> SubAssign for Vec3<T> {
    fn sub_assign(&mut self, rhs: Vec3<T>) {
        self.vec[X] -= rhs.vec[X];
        self.vec[Y] -= rhs.vec[Y];
        self.vec[Z] -= rhs.vec[Z];
    }
}

impl<T: ElemT> MulAssign for Vec3<T> {
    fn mul_assign(&mut self, rhs: Vec3<T>) {
        self.vec[X] *= rhs.vec[X];
        self.vec[Y] *= rhs.vec[Y];
        self.vec[Z] *= rhs.vec[Z];
    }
}

impl<T: ElemT> DivAssign for Vec3<T> {
    fn div_assign(&mut self, rhs: Vec3<T>) {
        self.vec[X] /= rhs.vec[X];
        self.vec[Y] /= rhs.vec[Y];
        self.vec[Z] /= rhs.vec[Z];
    }
}

impl<T: ElemT> MulAssign<T> for Vec3<T> {
    fn mul_assign(&mut self, rhs: T) {
        self.vec[X] *= rhs;
        self.vec[Y] *= rhs;
        self.vec[Z] *= rhs;
    }
}

impl<T: ElemT> DivAssign<T> for Vec3<T> {
    fn div_assign(&mut self, rhs: T) {
        self.vec[X] /= rhs;
        self.vec[Y] /= rhs;
        self.vec[Z] /= rhs;
    }
}

///////////////////////
// Binary operations //
///////////////////////

impl<T: ElemT> Add<Vec3<T>> for Vec3<T> {
    type Output = Vec3<T>;
    fn add(self, rhs: Vec3<T>) -> Vec3<T> {
        Vec3::new(self.x() + rhs.x(),
                  self.y() + rhs.y(),
                  self.z() + rhs.z())
    }
}

impl<'a, T: ElemT> Add<Vec3<T>> for &'a Vec3<T> {
    type Output = Vec3<T>;
    fn add(self, rhs: Vec3<T>) -> Vec3<T> {
        Vec3::new(self.x() + rhs.x(),
                  self.y() + rhs.y(),
                  self.z() + rhs.z())
    }
}

impl<'a, T: ElemT> Add<&'a Vec3<T>> for Vec3<T> {
    type Output = Vec3<T>;
    fn add(self, rhs: &'a Vec3<T>) -> Vec3<T> {
        Vec3::new(self.x() + rhs.x(),
                  self.y() + rhs.y(),
                  self.z() + rhs.z())
    }
}

impl<'a, 'b, T: ElemT> Add<&'b Vec3<T>> for &'a Vec3<T> {
    type Output = Vec3<T>;
    fn add(self, rhs: &'b Vec3<T>) -> Vec3<T> {
        Vec3::new(self.x() + rhs.x(),
                  self.y() + rhs.y(),
                  self.z() + rhs.z())
    }
}

impl<T: ElemT> Sub for Vec3<T> {
    type Output = Vec3<T>;
    fn sub(self, rhs: Vec3<T>) -> Vec3<T> {
        Vec3::new(self.x() - rhs.x(),
                  self.y() - rhs.y(),
                  self.z() - rhs.z())
    }
}

impl<'a, T: ElemT> Sub<Vec3<T>> for &'a Vec3<T> {
    type Output = Vec3<T>;
    fn sub(self, rhs: Vec3<T>) -> Vec3<T> {
        Vec3::new(self.x() - rhs.x(),
                  self.y() - rhs.y(),
                  self.z() - rhs.z())
    }
}

impl<'a, 'b, T: ElemT> Sub<&'b Vec3<T>> for &'a Vec3<T> {
    type Output = Vec3<T>;
    fn sub(self, rhs: &'b Vec3<T>) -> Vec3<T> {
        Vec3::new(self.x() - rhs.x(),
                  self.y() - rhs.y(),
                  self.z() - rhs.z())
    }
}

impl<'a, T: ElemT> Sub<&'a Vec3<T>> for Vec3<T> {
    type Output = Vec3<T>;
    fn sub(self, rhs: &'a Vec3<T>) -> Vec3<T> {
        Vec3::new(self.x() - rhs.x(),
                  self.y() - rhs.y(),
                  self.z() - rhs.z())
    }
}

impl<T: ElemT> Mul for Vec3<T> {
    type Output = Vec3<T>;
    fn mul(self, rhs: Vec3<T>) -> Vec3<T> {
        Vec3::new(self.x() * rhs.x(),
                  self.y() * rhs.y(),
                  self.z() * rhs.z())
    }
}

impl<T: ElemT> Div for Vec3<T> {
    type Output = Vec3<T>;
    fn div(self, rhs: Vec3<T>) -> Vec3<T> {
        Vec3::new(self.x() / rhs.x(),
                  self.y() / rhs.y(),
                  self.z() / rhs.z())
    }
}

impl<T: ElemT> Mul<T> for Vec3<T> {
    type Output = Vec3<T>;
    fn mul(self, rhs: T) -> Vec3<T> {
        Vec3::new(self.x() * rhs,
                  self.y() * rhs,
                  self.z() * rhs)
    }
}

impl<'a, T: ElemT> Mul<T> for &'a Vec3<T> {
    type Output = Vec3<T>;
    fn mul(self, rhs: T) -> Vec3<T> {
        Vec3::new(self.x() * rhs,
                  self.y() * rhs,
                  self.z() * rhs)
    }
}

impl<T: ElemT> Div<T> for Vec3<T> {
    type Output = Vec3<T>;
    fn div(self, rhs: T) -> Vec3<T> {
        Vec3::new(self.x() / rhs,
                  self.y() / rhs,
                  self.z() / rhs)
    }
}

impl<'a, T: ElemT> Div<T> for &'a Vec3<T> {
    type Output = Vec3<T>;
    fn div(self, rhs: T) -> Vec3<T> {
        Vec3::new(self.x() / rhs,
                  self.y() / rhs,
                  self.z() / rhs)
    }
}

/////////////////////////////////
// Special-case LHS scalar ops //
/////////////////////////////////

impl Mul<Vec3<f32>> for f32 {
    type Output = Vec3<f32>;
    fn mul(self, rhs: Vec3<f32>) -> Vec3<f32> {
        Vec3::new(self * rhs.x(),
                  self * rhs.y(),
                  self * rhs.z())
    }
}

impl Mul<Vec3<f64>> for f64 {
    type Output = Vec3<f64>;
    fn mul(self, rhs: Vec3<f64>) -> Vec3<f64> {
        Vec3::new(self * rhs.x(),
                  self * rhs.y(),
                  self * rhs.z())
    }
}

#[cfg(test)]
mod tests {
    use super::Vec3;
    use super::dot;
    use super::cross;

    #[test]
    fn test_squared_length() {
        let a = Vec3::<f64>::new(0.0, 4.0, 3.0);
        assert_approx_eq!(25.0, a.squared_length());
    }

    #[test]
    fn test_length() {
        let a = Vec3::<f64>::new(0.0, 4.0, 3.0);
        assert_approx_eq!(5.0, a.length());
    }

    #[test]
    fn test_dot() {
        let a = Vec3::<f64>::new(4.0, 5.0, 6.0);
        let b = Vec3::<f64>::new(2.0, 1.0, 3.0);

        assert_approx_eq!(31.0, dot(&a, &b));
    }

    #[test]
    fn test_cross() {
        let a = Vec3::<f64>::new(4.0, 5.0, 6.0);
        let b = Vec3::<f64>::new(2.0, 1.0, 3.0);
        
        let c = cross(&a, &b);

        assert_approx_eq!(9.0, c.x());
        assert_approx_eq!(0.0, c.y());
        assert_approx_eq!(-6.0, c.z());
    }

    #[test]
    fn test_unit_vector() {
        let a = Vec3::<f64>::new(0.0, 4.0, 3.0);

        let b = a.unit_vector();

        assert_approx_eq!(0.0, b.x());
        assert_approx_eq!(0.8, b.y());
        assert_approx_eq!(0.6, b.z());
    }

    #[test]
    fn test_make_unit_vector() {
        let mut a = Vec3::<f64>::new(0.0, 4.0, 3.0);

        a.make_unit_vector();

        assert_approx_eq!(0.0, a.x());
        assert_approx_eq!(0.8, a.y());
        assert_approx_eq!(0.6, a.z());
    }

    #[test]
    fn test_neg() {
        let a = Vec3::<f64>::new(4.0, 5.0, 6.0);

        let b = -a;

        assert_approx_eq!(-4.0, b.x());
        assert_approx_eq!(-5.0, b.y());
        assert_approx_eq!(-6.0, b.z());
    }

    #[test]
    fn test_index() {
        let a = Vec3::<f64>::new(4.0, 5.0, 6.0);

        assert_approx_eq!(4.0, a[0]);
        assert_approx_eq!(5.0, a[1]);
        assert_approx_eq!(6.0, a[2]);
    }

    #[test]
    fn test_mut_index() {
        let mut a = Vec3::<f64>::new(4.0, 5.0, 6.0);

        a[0] = 1.0;
        a[1] = 2.0;
        a[2] = 3.0;

        assert_approx_eq!(1.0, a[0]);
        assert_approx_eq!(2.0, a[1]);
        assert_approx_eq!(3.0, a[2]);
    }

    #[test]
    fn test_add_assign() {
        let mut a = Vec3::<f64>::new(4.0, 5.0, 6.0);
        let b = Vec3::<f64>::new(2.0, 1.0, 3.0);

        a += b;

        assert_approx_eq!(6.0, a.x());
        assert_approx_eq!(6.0, a.y());
        assert_approx_eq!(9.0, a.z());
    }

    #[test]
    fn test_sub_assign() {
        let mut a = Vec3::<f64>::new(4.0, 5.0, 6.0);
        let b = Vec3::<f64>::new(2.0, 1.0, 3.0);

        a -= b;

        assert_approx_eq!(2.0, a.x());
        assert_approx_eq!(4.0, a.y());
        assert_approx_eq!(3.0, a.z());
    }

    #[test]
    fn test_mul_assign() {
        let mut a = Vec3::<f64>::new(4.0, 5.0, 6.0);
        let b = Vec3::<f64>::new(2.0, 1.0, 3.0);

        a *= b;

        assert_approx_eq!(8.0, a.x());
        assert_approx_eq!(5.0, a.y());
        assert_approx_eq!(18.0, a.z());
    }

    #[test]
    fn test_div_assign() {
        let mut a = Vec3::<f64>::new(4.0, 5.0, 6.0);
        let b = Vec3::<f64>::new(2.0, 1.0, 3.0);

        a /= b;

        assert_approx_eq!(2.0, a.x());
        assert_approx_eq!(5.0, a.y());
        assert_approx_eq!(2.0, a.z());
    }

    #[test]
    fn test_mul_assign_scalar() {
        let mut a = Vec3::<f64>::new(4.0, 5.0, 6.0);
        let b = 2.0;

        a *= b;

        assert_approx_eq!(8.0, a.x());
        assert_approx_eq!(10.0, a.y());
        assert_approx_eq!(12.0, a.z());
    }

    #[test]
    fn test_div_assign_scalar() {
        let mut a = Vec3::<f64>::new(4.0, 10.0, 6.0);
        let b = 2.0;

        a /= b;

        assert_approx_eq!(2.0, a.x());
        assert_approx_eq!(5.0, a.y());
        assert_approx_eq!(3.0, a.z());
    }

    #[test]
    fn test_add() {
        let a = Vec3::<f64>::new(4.0, 5.0, 6.0);
        let b = Vec3::<f64>::new(2.0, 1.0, 3.0);

        let c = a + b;

        assert_approx_eq!(6.0, c.x());
        assert_approx_eq!(6.0, c.y());
        assert_approx_eq!(9.0, c.z());
    }

    #[test]
    fn test_sub() {
        let a = Vec3::<f64>::new(4.0, 5.0, 6.0);
        let b = Vec3::<f64>::new(2.0, 1.0, 3.0);

        let c = a - b;

        assert_approx_eq!(2.0, c.x());
        assert_approx_eq!(4.0, c.y());
        assert_approx_eq!(3.0, c.z());
    }

    #[test]
    fn test_mul() {
        let a = Vec3::<f64>::new(4.0, 5.0, 6.0);
        let b = Vec3::<f64>::new(2.0, 1.0, 3.0);

        let c = a * b;

        assert_approx_eq!(8.0, c.x());
        assert_approx_eq!(5.0, c.y());
        assert_approx_eq!(18.0, c.z());
    }

    #[test]
    fn test_div() {
        let a = Vec3::<f64>::new(4.0, 5.0, 6.0);
        let b = Vec3::<f64>::new(2.0, 1.0, 3.0);

        let c = a / b;

        assert_approx_eq!(2.0, c.x());
        assert_approx_eq!(5.0, c.y());
        assert_approx_eq!(2.0, c.z());
    }

    #[test]
    fn test_mul_scalarr() {
        let a = Vec3::<f64>::new(4.0, 5.0, 6.0);
        let b = 3.0;

        let c = a * b;

        assert_approx_eq!(12.0, c.x());
        assert_approx_eq!(15.0, c.y());
        assert_approx_eq!(18.0, c.z());
    }

    #[test]
    fn test_div_scalarr() {
        let a = Vec3::<f64>::new(4.0, 8.0, 6.0);
        let b = 2.0;

        let c = a / b;

        assert_approx_eq!(2.0, c.x());
        assert_approx_eq!(4.0, c.y());
        assert_approx_eq!(3.0, c.z());
    }

    #[test]
    fn test_mul_scalarl() {
        let a = Vec3::<f64>::new(4.0, 5.0, 6.0);
        let b = 3.0;

        let c = b * a;

        assert_approx_eq!(12.0, c.x());
        assert_approx_eq!(15.0, c.y());
        assert_approx_eq!(18.0, c.z());
    }
}
