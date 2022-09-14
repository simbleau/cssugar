pub trait Calculable<T>:
    std::fmt::Display
    + std::ops::Add
    + std::ops::Sub
    + std::ops::Mul
    + std::ops::Div
    + Sized
{
}
