use approx::AbsDiffEq;
use num::{Bounded, Num};

use core::cmp::PartialOrd;
use na::Scalar;
use simba::scalar::{ClosedAdd, ClosedMul, ClosedSub, RealField};

/// A number that can either be an integer or a float.
pub trait Number:
    Scalar
    + Copy
    + PartialOrd
    + ClosedAdd
    + ClosedSub
    + ClosedMul
    + AbsDiffEq<Epsilon = Self>
    + Num
    + Bounded
{
}

impl<
        T: Scalar
            + Copy
            + PartialOrd
            + ClosedAdd
            + ClosedSub
            + ClosedMul
            + AbsDiffEq<Epsilon = Self>
            + Num
            + Bounded,
    > Number for T
{
}

/// A number that can be any float type.
pub trait RealNumber: Number + RealField {}

impl<T: Number + RealField> RealNumber for T {}
