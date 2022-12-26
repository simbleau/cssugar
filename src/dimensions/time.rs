use crate::math::function::Operation;
use crate::math::{Addable, Calculable, Comparable, Function, Scalable};

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Time {
    Seconds(f64),
    Milliseconds(f64),
    Percent(f64),
    Duration(std::time::Duration),
}

impl std::fmt::Display for Time {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Time::Seconds(v) => write!(f, "{}s", v),
            Time::Milliseconds(v) => write!(f, "{}ms", v),
            Time::Percent(v) => write!(f, "{}%", v),
            Time::Duration(v) => write!(f, "{}ms", v.as_millis()),
        }
    }
}

impl Calculable<Time> for Time {}
impl Addable<Time> for Time {}
impl Scalable<Time> for Time {}

impl Comparable<Time> for Time {
    fn min<Rhs>(self, rhs: Rhs) -> Function<Time, Self, Rhs> {
        Function::new(self, rhs, Operation::Min)
    }

    fn max<Rhs>(self, rhs: Rhs) -> Function<Time, Self, Rhs> {
        Function::new(self, rhs, Operation::Max)
    }
}

impl<Rhs> std::ops::Add<Rhs> for Time
where
    Rhs: Addable<Time>,
{
    type Output = Function<Time, Self, Rhs>;
    fn add(self, rhs: Rhs) -> Function<Time, Self, Rhs> {
        Function::new(self, rhs, Operation::Add)
    }
}

impl<Rhs> std::ops::Sub<Rhs> for Time
where
    Rhs: Addable<Time>,
{
    type Output = Function<Time, Self, Rhs>;
    fn sub(self, rhs: Rhs) -> Function<Time, Self, Rhs> {
        Function::new(self, rhs, Operation::Sub)
    }
}

impl<Rhs> std::ops::Mul<Rhs> for Time
where
    Rhs: Scalable<Time>,
{
    type Output = Function<Time, Self, Rhs>;
    fn mul(self, rhs: Rhs) -> Function<Time, Self, Rhs> {
        Function::new(self, rhs, Operation::Mul)
    }
}

impl<Rhs> std::ops::Div<Rhs> for Time
where
    Rhs: Scalable<Time>,
{
    type Output = Function<Time, Self, Rhs>;
    fn div(self, rhs: Rhs) -> Function<Time, Self, Rhs> {
        Function::new(self, rhs, Operation::Div)
    }
}

#[cfg(test)]
mod tests {
    use crate::prelude::*;

    #[test]
    fn test_add() {
        let t1 = Time::Seconds(100.);
        let t2 = Time::Milliseconds(500.);
        assert_eq!(format!("{}", t1 + t2), "calc(100s+500ms)");
    }

    #[test]
    fn test_sub() {
        let t1 = Time::Seconds(100.);
        let t2 = Time::Milliseconds(500.);
        assert_eq!(format!("{}", t1 - t2), "calc(100s-500ms)");
    }

    #[test]
    fn test_mul() {
        let t1 = Time::Seconds(100.);
        let t2 = Time::Milliseconds(500.);
        assert_eq!(format!("{}", t1 * t2), "calc(100s*500ms)");
    }

    #[test]
    fn test_div() {
        let t1 = Time::Seconds(100.);
        let t2 = Time::Milliseconds(500.);
        assert_eq!(format!("{}", t1 / t2), "calc(100s/500ms)");
    }

    #[test]
    fn test_max() {
        let t1 = Time::Seconds(100.);
        let t2 = Time::Milliseconds(500.);
        assert_eq!(format!("{}", t1.max(t2)), "max(100s,500ms)");
    }

    #[test]
    fn test_min() {
        let t1 = Time::Seconds(100.);
        let t2 = Time::Milliseconds(500.);
        assert_eq!(format!("{}", t1.min(t2)), "min(100s,500ms)");
    }
}
