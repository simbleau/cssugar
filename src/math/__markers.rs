use crate::math::ops::{Max, Min};
use std::{fmt::Display, ops::Add, ops::Div, ops::Mul, ops::Sub};

/// Able to be embedded in a calculation.
pub trait Calculable<T>: Display + Sized {}

/// Able to be added or subtracted from types with the same underlying unit.
pub trait Addable<T>: Display + Sized + Add + Sub {}

/// Able to be scaled from types with the same underlying unit.
pub trait Scalable<T>: Display + Sized + Mul + Div {}

/// Able to be compared to yield a maximum value.
pub trait Maxable: Display + Sized + Max {
    type Unit;
}

/// Able to be compared to yield a minimum value.
pub trait Minable: Display + Sized + Min {
    type Unit;
}
