#![allow(dead_code)]

use std::ops::{Add, Sub, Neg, Mul, Div};
use std::cmp::{PartialEq, PartialOrd, Ordering};
pub use num::{Zero, One};
use num;
use super::float::Float;
use std::fmt;
use clamp::Clamp;

/// Vec4 is a generic four-component (3D) vector type.
#[derive(Copy, Clone, Debug)]
pub struct Vec4<T>{
    x: T,
    y: T,
    z: T,
    w: T
}

impl<T> Vec4<T>{
    /// new returns a new vector with the given parameters.
    ///
    /// # Examples
    ///
    /// ```
    /// let x = fiz_math::Vec4::new(4.0f32, 8.0f32, 2.0f32, 3.0f32);
    /// ```
    ///
    /// ```
    /// let x = fiz_math::Vec4::new(1u8, 5u8, 2u8, 3u8);
    /// ```
    ///
    /// ```
    /// use fiz_math::Vec4;
    /// use fiz_math::unit::MM;
    ///
    /// let x = Vec4::new(MM(1.0), MM(5.0), MM(2.0), MM(1.2));
    /// let y = Vec4::new(MM(1.0), MM(5.1), MM(1.9), MM(1.1));
    /// assert!(x.almost_equal(y, 0.1));
    /// ```
    pub fn new(x: T, y: T, z: T, w: T) -> Self {
        Vec4{x: x, y: y, z: z, w: w}
    }
}

impl<T: fmt::Display> fmt::Display for Vec4<T> {
    /// fmt formats the vector.
    ///
    /// # Examples
    ///
    /// ```
    /// let x = fiz_math::Vec4::new(1u8, 5u8, 2u8, 3u8);
    /// assert_eq!(format!("{}", x), "Vec3(1, 5, 2, 3)");
    /// ```
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Vec3({}, {}, {}, {})", self.x, self.y, self.z, self.w)
    }
}

impl<T: One> One for Vec4<T>{
    /// one returns the one value for a vector whose component type implements the
    /// num::One trait.
    ///
    /// # Examples
    ///
    /// ```
    /// use fiz_math::One;
    ///
    /// let x = fiz_math::Vec4::<f32>::one();
    /// ```
    ///
    /// ```
    /// use fiz_math::One;
    ///
    /// let x = fiz_math::Vec4::<i64>::one();
    /// ```
    fn one() -> Self {
        Vec4{x: T::one(), y: T::one(), z: T::one(), w: T::one()}
    }
}

impl<T: Float> Vec4<T>{
    /// almost_equal tells if this vector is equal to the other given an absolute
    /// tolerence value (see the almost_equal function for more details).
    ///
    /// # Examples
    ///
    /// ```
    /// use fiz_math::Vec4;
    ///
    /// let a = Vec4::<f32>::new(1.0, 1.0, 1.0, 1.0);
    /// let b = Vec4::<f32>::new(0.9, 0.9, 0.9, 0.9);
    /// assert!(a.almost_equal(b, 0.1000001));
    /// assert!(!a.almost_equal(b, 0.1));
    /// ```
    pub fn almost_equal<N: num::Float>(self, other: Self, abs_tol: N) -> bool {
        self.x.almost_equal(other.x, abs_tol) &&
        self.y.almost_equal(other.y, abs_tol) &&
        self.z.almost_equal(other.z, abs_tol) &&
        self.w.almost_equal(other.w, abs_tol)
    }

    /// is_nan tells if all of this vectors components are NaN.
    ///
    /// # Examples
    ///
    /// ```
    /// # extern crate num;
    /// # extern crate fiz_math;
    /// use num::traits::Float;
    /// use fiz_math::Vec4;
    ///
    /// # fn main() {
    /// let n:f32 = Float::nan();
    /// assert!(Vec4::new(n, n, n, n).is_nan());
    /// assert!(!Vec4::new(n, 0.0, 0.0, 0.0).is_nan());
    /// # }
    /// ```
    pub fn is_nan(self) -> bool {
        self.x.is_nan() &&
        self.y.is_nan() &&
        self.z.is_nan() &&
        self.w.is_nan()
    }
}

impl<T: Float> Vec4<T> {
    /// round returns the nearest integer to a number. Round half-way cases away
    /// from 0.0.
    ///
    /// # Examples
    ///
    /// ```
    /// use fiz_math::Vec4;
    ///
    /// assert_eq!(Vec4::new(0.3, 1.3, 2.0, 2.7).round(), Vec4::new(0.0, 1.0, 2.0, 3.0))
    /// ```
    pub fn round(&self) -> Self {
        Vec4::new(self.x.round(), self.y.round(), self.z.round(), self.w.round())
    }

    /// length returns the magnitude of this vector. Use length_sq for comparing
    /// distances instead, because it avoids the sqrt operation.
    pub fn length(self) -> T { self.length_sq().sqrt() }
}

impl<T: num::traits::Num + Copy> Vec4<T> {
    /// dot returns the dot product of self and b.
    pub fn dot(self, b: Self) -> T {
        self.x*b.x + self.y+b.y + self.z+b.z + self.w+b.w
    }

    /// length_sq returns the magnitude squared of this vector, useful primarily
    /// for comparing distances.
    pub fn length_sq(self) -> T {
        self.x*self.x + self.y*self.y + self.z*self.z + self.w*self.w
    }
}

impl<T: Add<Output = T>> Add for Vec4<T>{
    type Output = Self;

    /// add performs component-wise addition of two vectors.
    ///
    /// # Examples
    ///
    /// ```
    /// use fiz_math::Vec4;
    ///
    /// let a = Vec4::new(1, 2, 3, 3);
    /// let b = Vec4::new(4, 5, 6, 6);
    /// assert_eq!(a + b, Vec4::new(5, 7, 9, 9));
    /// ```
    fn add(self, _rhs: Self) -> Self {
        Vec4{
            x: self.x + _rhs.x,
            y: self.y + _rhs.y,
            z: self.z + _rhs.z,
            w: self.w + _rhs.w,
        }
    }
}

impl<T: Add<Output = T> + Copy> Vec4<T> {
    /// add_scalar performs scalar addition on a vector.
    ///
    /// # Examples
    ///
    /// ```
    /// use fiz_math::Vec4;
    ///
    /// let a = Vec4::new(1, 2, 3, 4);
    /// assert_eq!(a.add_scalar(1), Vec4::new(2, 3, 4, 5));
    /// ```
    pub fn add_scalar(self, _rhs: T) -> Self {
        Vec4{
            x: self.x + _rhs,
            y: self.y + _rhs,
            z: self.z + _rhs,
            w: self.w + _rhs,
        }
    }
}

impl<T: Neg<Output = T>> Neg for Vec4<T>{
    type Output = Self;

    /// neg returns the negated (i.e. inversed) vector self.
    ///
    /// # Examples
    ///
    /// ```
    /// use fiz_math::Vec4;
    ///
    /// assert_eq!(-Vec4::new(1, 2, 3, 4), Vec4::new(-1, -2, -3, -4));
    /// ```
    fn neg(self) -> Self {
        Vec4{x: -self.x, y: -self.y, z: -self.z, w: -self.w}
    }
}

impl<T: Sub<Output = T>> Sub for Vec4<T>{
    type Output = Self;

    /// sub performs component-wise subtraction of two vectors.
    ///
    /// # Examples
    ///
    /// ```
    /// use fiz_math::Vec4;
    ///
    /// let a = Vec4::new(1, 2, 3, 3);
    /// let b = Vec4::new(4, 5, 6, 6);
    /// assert_eq!(a - b, Vec4::new(-3, -3, -3, -3));
    /// ```
    fn sub(self, _rhs: Self) -> Self {
        Vec4{
            x: self.x - _rhs.x,
            y: self.y - _rhs.y,
            z: self.z - _rhs.z,
            w: self.w - _rhs.w,
        }
    }
}

impl<T: Sub<Output = T> + Copy> Vec4<T> {
    /// sub_scalar performs scalar subtraction on a vector.
    ///
    /// # Examples
    ///
    /// ```
    /// use fiz_math::Vec4;
    ///
    /// let a = Vec4::new(2, 3, 4, 5);
    /// assert_eq!(a.sub_scalar(1), Vec4::new(1, 2, 3, 4));
    /// ```
    pub fn sub_scalar(self, _rhs: T) -> Self {
        Vec4{
            x: self.x - _rhs,
            y: self.y - _rhs,
            z: self.z - _rhs,
            w: self.w - _rhs,
        }
    }
}

impl<T: Mul<Output = T>> Mul for Vec4<T>{
    type Output = Self;

    /// mul performs component-wise multiplication of two vectors.
    ///
    /// # Examples
    ///
    /// ```
    /// use fiz_math::Vec4;
    ///
    /// let a = Vec4::new(1, 2, 3, 3);
    /// let b = Vec4::new(4, 5, 6, 6);
    /// assert_eq!(a * b, Vec4::new(4, 10, 18, 18));
    /// ```
    fn mul(self, _rhs: Self) -> Self {
        Vec4{
            x: self.x * _rhs.x,
            y: self.y * _rhs.y,
            z: self.z * _rhs.z,
            w: self.w * _rhs.w,
        }
    }
}

impl<T: Mul<Output = T> + Copy> Vec4<T> {
    /// mul_scalar performs scalar multiplication on a vector.
    ///
    /// # Examples
    ///
    /// ```
    /// use fiz_math::Vec4;
    ///
    /// let a = Vec4::new(1, 2, 3, 4);
    /// assert_eq!(a.mul_scalar(2), Vec4::new(2, 4, 6, 8));
    /// ```
    pub fn mul_scalar(self, _rhs: T) -> Self {
        Vec4{
            x: self.x * _rhs,
            y: self.y * _rhs,
            z: self.z * _rhs,
            w: self.w * _rhs,
        }
    }
}

impl<T: Div<Output = T>> Div for Vec4<T>{
    type Output = Self;

    /// div performs component-wise division of two vectors.
    ///
    /// # Examples
    ///
    /// ```
    /// use fiz_math::Vec4;
    ///
    /// let a = Vec4::new(4, 5, 9, 9);
    /// let b = Vec4::new(1, 2, 3, 3);
    /// assert_eq!(a / b, Vec4::new(4, 2, 3, 3));
    /// ```
    fn div(self, _rhs: Self) -> Self {
        Vec4{
            x: self.x / _rhs.x,
            y: self.y / _rhs.y,
            z: self.z / _rhs.z,
            w: self.w / _rhs.w,
        }
    }
}

impl<T: Div<Output = T> + Copy> Vec4<T> {
    /// div_scalar performs scalar division on a vector.
    ///
    /// # Examples
    ///
    /// ```
    /// use fiz_math::Vec4;
    ///
    /// let a = Vec4::new(2, 4, 6, 8);
    /// assert_eq!(a.div_scalar(2), Vec4::new(1, 2, 3, 4));
    /// ```
    pub fn div_scalar(self, _rhs: T) -> Self {
        Vec4{
            x: self.x / _rhs,
            y: self.y / _rhs,
            z: self.z / _rhs,
            w: self.w / _rhs,
        }
    }
}

impl<T: Clamp<Elem = T> + Copy> Clamp for Vec4<T>{
    type Elem = T;

    /// clamp returns the vector with each element clamped to the range of
    /// [min, max].
    ///
    /// # Examples
    ///
    /// ```
    /// use fiz_math::{Vec4, Clamp};
    ///
    /// let a = Vec4::new(-2, 4, -6, 8);
    /// assert_eq!(a.clamp(-1, 2), Vec4::new(-1, 2, -1, 2));
    /// ```
    fn clamp(self, min: T, max: T) -> Self {
        Vec4{
            x: self.x.clamp(min, max),
            y: self.y.clamp(min, max),
            z: self.z.clamp(min, max),
            w: self.w.clamp(min, max),
        }
    }
}

impl<T> AsRef<Vec4<T>> for Vec4<T> {
    fn as_ref(&self) -> &Self {
        self
    }
}

impl<T:PartialOrd> Vec4<T> {
    /// any_less tells if any component of the other vector is less than any
    /// component of this vector.
    ///
    /// # Examples
    ///
    /// ```
    /// use fiz_math::Vec4;
    ///
    /// let a = Vec4::new(0, 0, 0, 1);
    /// assert!(a.any_less(Vec4::new(0, 0, 0, 2)));
    /// ```
    pub fn any_less<O:AsRef<Self>>(&self, other: O) -> bool {
        let o = other.as_ref();
        self.x < o.x || self.y < o.y || self.z < o.z || self.w < o.w
    }

    /// any_greater tells if any component of the other vector is greater than
    /// any component of this vector.
    ///
    /// # Examples
    ///
    /// ```
    /// use fiz_math::Vec4;
    ///
    /// let a = Vec4::new(0, 0, 0, 2);
    /// assert!(a.any_greater(Vec4::new(0, 0, 0, 1)));
    /// ```
    pub fn any_greater<O:AsRef<Self>>(&self, other: O) -> bool {
        let o = other.as_ref();
        self.x > o.x || self.y > o.y || self.z > o.z || self.w > o.w
    }
}

impl<T: PartialEq> PartialEq for Vec4<T> {
    /// eq tests for component-wise binary equality of two vectors.
    ///
    /// # Examples
    ///
    /// ```
    /// use fiz_math::Vec4;
    ///
    /// let a = Vec4::new(4.0, 5.0, 5.0, 9.0);
    /// let b = Vec4::new(4.0, 5.0, 5.0, 9.00000000000000000000001);
    /// assert_eq!(a, b);
    /// ```
    ///
    /// ```
    /// use fiz_math::Vec4;
    ///
    /// let a = Vec4::new(4, 5, 9, 9);
    /// let b = Vec4::new(4, 5, 9, 9);
    /// assert_eq!(a, b);
    /// ```
    fn eq(&self, _rhs: &Self) -> bool {
        self.x == _rhs.x &&
        self.y == _rhs.y &&
        self.z == _rhs.z &&
        self.w == _rhs.w
    }
}

impl<T: PartialOrd> PartialOrd for Vec4<T>{
    /// partial_cmp compares the two vectors component-wise.
    ///
    /// # Examples
    ///
    /// ```
    /// use fiz_math::Vec4;
    ///
    /// let a = Vec4::new(1.0, 2.0, 3.0, 4.0);
    /// assert!(a < Vec4::new(1.1, 2.1, 3.1, 4.1));
    /// ```
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        if self.x < other.x && self.y < other.y && self.z < other.z && self.w < other.w {
            Some(Ordering::Less)
        } else if self.x > other.x && self.y > other.y && self.z > other.z && self.w > other.w {
            Some(Ordering::Greater)
        } else if self == other {
            Some(Ordering::Equal)
        } else {
            None
        }
    }
}

impl<T: Zero> Zero for Vec4<T>{
    /// zero returns the zero-value for the vector.
    ///
    /// # Examples
    ///
    /// ```
    /// use fiz_math::{Zero, Vec4};
    ///
    /// let x = Vec4::<u8>::zero();
    /// let y = Vec4::<i64>::zero();
    /// let z = Vec4::<f32>::zero();
    /// let w = Vec4::<f64>::zero();
    /// ```
    fn zero() -> Self {
        Vec4{x: Zero::zero(), y: Zero::zero(), z: Zero::zero(), w: Zero::zero()}
    }

    /// is_zero tests if the vector is equal to zero.
    ///
    /// # Examples
    ///
    /// ```
    /// use fiz_math::{Zero, Vec4};
    ///
    /// assert!(!Vec4::new(1i32, 0, 0, 0).is_zero());
    /// assert!(Vec4::new(0u8, 0, 0, 0).is_zero());
    /// assert!(!Vec4::new(1.0f32, 0.0, 0.0, 0.0).is_zero());
    /// assert!(Vec4::new(0.0f64, 0.0, 0.0, 0.0).is_zero());
    /// ```
    fn is_zero(&self) -> bool {
        self.x.is_zero() &&
        self.y.is_zero() &&
        self.z.is_zero() &&
        self.w.is_zero()
    }
}
