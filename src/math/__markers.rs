use crate::math::Function;
use std::{fmt::Display, ops::Add, ops::Div, ops::Mul, ops::Sub};

/// Able to be embedded in a calculation.
pub trait Calculable<Unit>: Display + Sized {}

/// Able to be added or subtracted from types with the same underlying unit.
pub trait Addable<Unit>: Display + Sized + Add + Sub {}

/// Able to be scaled from types with the same underlying unit.
pub trait Scalable<Unit>: Display + Sized + Mul + Div {}

/// Able to be compared to yield a maximum or minimum value.
pub trait Comparable<Unit>: Display + Sized {
    fn min<Rhs>(self, rhs: Rhs) -> Function<Unit, Self, Rhs>;
    fn max<Rhs>(self, rhs: Rhs) -> Function<Unit, Self, Rhs>;
}
