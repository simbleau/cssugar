pub trait Calculable<T>:
    std::fmt::Display
    + Sized
    + std::ops::Add
    + std::ops::Sub
    + std::ops::Mul
    + std::ops::Div
{
}
