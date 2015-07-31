use num::traits::{Num, NumCast};

use super::cm::{CM, ToCM};
use super::mm::{MM, ToMM};
use super::km::{KM, ToKM};

/// ToM is the canonical trait to use for taking input in meters.
///
/// For example the millimeters type (MM) implements the ToM trait and thus
/// millimeters can be given as a parameter to any input that seeks meters.
pub trait ToM<T>{
    /// to_m returns these units in meters, performing conversion if needed.
    ///
    /// # Examples
    ///
    /// ```
    /// use fiz_math::dist::{MM, ToM};
    /// use fiz_math::Num;
    /// use std::fmt::Debug;
    ///
    /// fn walk<T: ToM<U>, U: Num+Debug>(dist: T) {
    ///     println!("{:?}", dist.to_m().0)
    /// }
    /// walk(MM(2.0));
    /// walk(MM::<i16>(2));
    /// ```
    fn to_m(self) -> M<T>;
}

/// M represents meters (the SI base unit representing distance).
///
/// # Examples
///
/// ```
/// use fiz_math::dist::M;
///
/// let x = M(1.0);
/// println!("{:?}", x);
/// ```
unit!(M);

impl<T: Num+NumCast> ToMM<T> for M<T> {
    /// to_mm returns these meters converted to millimeters.
    ///
    /// # Examples
    ///
    /// ```
    /// use fiz_math::dist::{M, MM, ToMM};
    ///
    /// assert_eq!(M(1.0).to_mm(), MM(1000.0));
    /// ```
    fn to_mm(self) -> MM<T> {
        MM(self.0 * T::from(1000).unwrap())
    }
}

impl<T: Num+NumCast> ToCM<T> for M<T> {
    /// to_cm returns these meters converted to centimeters.
    ///
    /// # Examples
    ///
    /// ```
    /// use fiz_math::dist::{M, CM, ToCM};
    ///
    /// assert_eq!(M(1.0).to_cm(), CM(100.0));
    /// ```
    fn to_cm(self) -> CM<T> {
        CM(self.0 * T::from(100).unwrap())
    }
}

impl<T: Num+NumCast> ToM<T> for M<T> {
    /// to_m simply returns self.
    ///
    /// # Examples
    ///
    /// ```
    /// use fiz_math::dist::{M, ToM};
    ///
    /// assert_eq!(M(1.0).to_m(), M(1.0));
    /// ```
    fn to_m(self) -> M<T> {
        self
    }
}

impl<T: Num+NumCast> ToKM<T> for M<T> {
    /// to_km returns these meters converted to kilometers.
    ///
    /// # Examples
    ///
    /// ```
    /// use fiz_math::dist::{M, KM, ToKM};
    ///
    /// assert_eq!(M(1000.0).to_km(), KM(1.0));
    /// ```
    fn to_km(self) -> KM<T> {
        KM(self.0 / T::from(1000).unwrap())
    }
}
