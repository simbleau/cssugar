use std::{fmt::Display, ops::Add, ops::Div, ops::Mul, ops::Sub};

pub trait Calculable: Display + Sized + Add + Sub + Mul + Div {
    type Unit;
}
