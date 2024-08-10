use approx::AbsDiffEq;
use num::{Bounded, Num};

use na::Scalar;
use simba::scalar::{ClosedAddAssign, ClosedMulAssign, ClosedSubAssign, RealField};

/// A number that can either be an integer or a float.
pub trait Number:
    Scalar
    + Copy
    + PartialOrd
    + ClosedAddAssign
    + ClosedSubAssign
    + ClosedMulAssign
    + AbsDiffEq<Epsilon = Self>
    + Num
    + Bounded
{
}

impl<
        T: Scalar
            + Copy
            + PartialOrd
            + ClosedAddAssign
            + ClosedSubAssign
            + ClosedMulAssign
            + AbsDiffEq<Epsilon = Self>
            + Num
            + Bounded,
    > Number for T
{
}

/// A number that can be any float type.
pub trait RealNumber: Number + RealField {}

impl<T: Number + RealField> RealNumber for T {}
