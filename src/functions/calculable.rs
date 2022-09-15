use std::fmt::Display;

pub trait Calculable:
    Display + Sized + std::ops::Add + std::ops::Sub + std::ops::Mul + std::ops::Div
{
    type Unit;
}
