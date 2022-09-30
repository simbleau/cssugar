use super::ops::{Max, Min};
use std::{fmt::Display, ops::Add, ops::Div, ops::Mul, ops::Sub};

pub trait Addable: Display + Sized + Add + Sub {
    type Unit;
}

pub trait Scalable: Display + Sized + Mul + Div {
    type Unit;
}

pub trait Maxable: Display + Sized + Max {
    type Unit;
}

pub trait Minable: Display + Sized + Min {
    type Unit;
}
