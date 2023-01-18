use crate::math::function::Operation;
use crate::math::{Addable, Calculable, Comparable, Function, Scalable};

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Angle {
    Deg(f64),
    Grad(f64),
    Rad(f64),
    Turn(f64),
    Percent(f64),
    Unchecked(&'static str),
}

impl std::fmt::Display for Angle {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Angle::Deg(v) => write!(f, "{}deg", v),
            Angle::Grad(v) => write!(f, "{}grad", v),
            Angle::Rad(v) => write!(f, "{}rad", v),
            Angle::Turn(v) => write!(f, "{}turn", v),
            Angle::Percent(v) => write!(f, "{}%", v),
            Angle::Unchecked(v) => write!(f, "{v}"),
        }
    }
}

impl Calculable<Angle> for Angle {}
impl Addable<Angle> for Angle {}
impl Scalable<Angle> for Angle {}

impl Comparable<Angle> for Angle {
    fn min<Rhs>(self, rhs: Rhs) -> Function<Angle, Self, Rhs> {
        Function::new(self, rhs, Operation::Min)
    }

    fn max<Rhs>(self, rhs: Rhs) -> Function<Angle, Self, Rhs> {
        Function::new(self, rhs, Operation::Max)
    }
}

impl<Rhs> std::ops::Add<Rhs> for Angle
where
    Rhs: Addable<Angle>,
{
    type Output = Function<Angle, Self, Rhs>;
    fn add(self, rhs: Rhs) -> Function<Angle, Self, Rhs> {
        Function::new(self, rhs, Operation::Add)
    }
}

impl<Rhs> std::ops::Sub<Rhs> for Angle
where
    Rhs: Addable<Angle>,
{
    type Output = Function<Angle, Self, Rhs>;
    fn sub(self, rhs: Rhs) -> Function<Angle, Self, Rhs> {
        Function::new(self, rhs, Operation::Sub)
    }
}

impl<Rhs> std::ops::Mul<Rhs> for Angle
where
    Rhs: Scalable<Angle>,
{
    type Output = Function<Angle, Self, Rhs>;
    fn mul(self, rhs: Rhs) -> Function<Angle, Self, Rhs> {
        Function::new(self, rhs, Operation::Mul)
    }
}

impl<Rhs> std::ops::Div<Rhs> for Angle
where
    Rhs: Scalable<Angle>,
{
    type Output = Function<Angle, Self, Rhs>;
    fn div(self, rhs: Rhs) -> Function<Angle, Self, Rhs> {
        Function::new(self, rhs, Operation::Div)
    }
}

#[cfg(test)]
mod tests {
    use crate::prelude::*;

    #[test]
    fn test_add() {
        let a1 = Angle::Rad(100.);
        let a2 = Angle::Percent(500.);
        assert_eq!(format!("{}", a1 + a2), "calc(100rad + 500%)");
    }

    #[test]
    fn test_sub() {
        let a1 = Angle::Rad(100.);
        let a2 = Angle::Percent(500.);
        assert_eq!(format!("{}", a1 - a2), "calc(100rad - 500%)");
    }

    #[test]
    fn test_mul() {
        let a1 = Angle::Rad(100.);
        let a2 = Angle::Percent(500.);
        assert_eq!(format!("{}", a1 * a2), "calc(100rad * 500%)");
    }

    #[test]
    fn test_div() {
        let a1 = Angle::Rad(100.);
        let a2 = Angle::Percent(500.);
        assert_eq!(format!("{}", a1 / a2), "calc(100rad / 500%)");
    }

    #[test]
    fn test_max() {
        let a1 = Angle::Rad(100.);
        let a2 = Angle::Percent(500.);
        assert_eq!(format!("{}", a1.max(a2)), "max(100rad, 500%)");
    }

    #[test]
    fn test_min() {
        let a1 = Angle::Rad(100.);
        let a2 = Angle::Percent(500.);
        assert_eq!(format!("{}", a1.min(a2)), "min(100rad, 500%)");
    }

    #[test]
    fn test_calc() {
        let a1 = Angle::Rad(100.);
        let a2 = Angle::Rad(100.) + Angle::Percent(30.);
        assert_eq!(format!("{}", a1 + a2), "calc(100rad + calc(100rad + 30%))");
        assert_eq!(format!("{}", a1 - a2), "calc(100rad - calc(100rad + 30%))");
        assert_eq!(format!("{}", a1 * a2), "calc(100rad * calc(100rad + 30%))");
        assert_eq!(format!("{}", a1 / a2), "calc(100rad / calc(100rad + 30%))");
    }

    #[test]
    fn test_unchecked() {
        let a1 = Angle::Unchecked("100rad");
        let a2 = Angle::Unchecked("500%");
        assert_eq!(format!("{}", a1 + a2), "calc(100rad + 500%)");
    }
}
