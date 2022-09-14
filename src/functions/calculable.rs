pub trait Calculable<T>:
    std::fmt::Display
    + std::ops::Add<T>
    + std::ops::Sub<T>
    + std::ops::Mul<T>
    + std::ops::Div<T>
    + Sized
{
}
