use crate::math::ops::{Max, Min};
use std::{fmt::Display, ops::Add, ops::Div, ops::Mul, ops::Sub};

pub trait Addable<T>: Display + Sized + Add + Sub {}

pub trait Scalable<T>: Display + Sized + Mul + Div {}

pub trait Maxable: Display + Sized + Max {
    type Unit;
}

pub trait Minable: Display + Sized + Min {
    type Unit;
}
