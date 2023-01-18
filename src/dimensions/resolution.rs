use crate::math::function::Operation;
use crate::math::{Addable, Calculable, Comparable, Function, Scalable};

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Resolution {
    Dpi(f64),
    Dpcm(f64),
    Dppx(f64),
    Unchecked(&'static str),
}

impl std::fmt::Display for Resolution {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Resolution::Dpi(v) => write!(f, "{}dpi", v),
            Resolution::Dpcm(v) => write!(f, "{}dpcm", v),
            Resolution::Dppx(v) => write!(f, "{}dppx", v),
            Resolution::Unchecked(v) => write!(f, "{v}"),
        }
    }
}

impl Calculable<Resolution> for Resolution {}
impl Addable<Resolution> for Resolution {}
impl Scalable<Resolution> for Resolution {}

impl Comparable<Resolution> for Resolution {
    fn min<Rhs>(self, rhs: Rhs) -> Function<Resolution, Self, Rhs> {
        Function::new(self, rhs, Operation::Min)
    }

    fn max<Rhs>(self, rhs: Rhs) -> Function<Resolution, Self, Rhs> {
        Function::new(self, rhs, Operation::Max)
    }
}

impl<Rhs> std::ops::Add<Rhs> for Resolution
where
    Rhs: Addable<Resolution>,
{
    type Output = Function<Resolution, Self, Rhs>;
    fn add(self, rhs: Rhs) -> Function<Resolution, Self, Rhs> {
        Function::new(self, rhs, Operation::Add)
    }
}

impl<Rhs> std::ops::Sub<Rhs> for Resolution
where
    Rhs: Addable<Resolution>,
{
    type Output = Function<Resolution, Self, Rhs>;
    fn sub(self, rhs: Rhs) -> Function<Resolution, Self, Rhs> {
        Function::new(self, rhs, Operation::Sub)
    }
}

impl<Rhs> std::ops::Mul<Rhs> for Resolution
where
    Rhs: Scalable<Resolution>,
{
    type Output = Function<Resolution, Self, Rhs>;
    fn mul(self, rhs: Rhs) -> Function<Resolution, Self, Rhs> {
        Function::new(self, rhs, Operation::Mul)
    }
}

impl<Rhs> std::ops::Div<Rhs> for Resolution
where
    Rhs: Scalable<Resolution>,
{
    type Output = Function<Resolution, Self, Rhs>;
    fn div(self, rhs: Rhs) -> Function<Resolution, Self, Rhs> {
        Function::new(self, rhs, Operation::Div)
    }
}
#[cfg(test)]
mod tests {
    use crate::prelude::*;

    #[test]
    fn test_add() {
        let r1 = Resolution::Dpi(50.0);
        let r2 = Resolution::Dppx(25.0);
        assert_eq!(format!("{}", r1 + r2), "calc(50dpi + 25dppx)");
    }

    #[test]
    fn test_sub() {
        let r1 = Resolution::Dpi(50.0);
        let r2 = Resolution::Dppx(25.0);
        assert_eq!(format!("{}", r1 - r2), "calc(50dpi - 25dppx)");
    }

    #[test]
    fn test_mul() {
        let r1 = Resolution::Dpi(50.0);
        let r2 = Resolution::Dppx(25.0);
        assert_eq!(format!("{}", r1 * r2), "calc(50dpi * 25dppx)");
    }

    #[test]
    fn test_div() {
        let r1 = Resolution::Dpi(50.0);
        let r2 = Resolution::Dppx(25.0);
        assert_eq!(format!("{}", r1 / r2), "calc(50dpi / 25dppx)");
    }

    #[test]
    fn test_max() {
        let r1 = Resolution::Dpi(50.0);
        let r2 = Resolution::Dppx(25.0);
        assert_eq!(format!("{}", r1.max(r2)), "max(50dpi, 25dppx)");
    }

    #[test]
    fn test_min() {
        let r1 = Resolution::Dpi(50.0);
        let r2 = Resolution::Dppx(25.0);
        assert_eq!(format!("{}", r1.min(r2)), "min(50dpi, 25dppx)");
    }

    #[test]
    fn test_calc() {
        let a1 = Resolution::Dpi(100.0);
        let a2 = Resolution::Dppx(100.) + Resolution::Dpcm(30.);
        assert_eq!(
            format!("{}", a1 + a2),
            "calc(100dpi + calc(100dppx + 30dpcm))"
        );
        assert_eq!(
            format!("{}", a1 - a2),
            "calc(100dpi - calc(100dppx + 30dpcm))"
        );
        assert_eq!(
            format!("{}", a1 * a2),
            "calc(100dpi * calc(100dppx + 30dpcm))"
        );
        assert_eq!(
            format!("{}", a1 / a2),
            "calc(100dpi / calc(100dppx + 30dpcm))"
        );
    }

    #[test]
    fn test_unchecked() {
        let r1 = Resolution::Unchecked("50dpi");
        let r2 = Resolution::Unchecked("25dppx");
        assert_eq!(format!("{}", r1 + r2), "calc(50dpi + 25dppx)");
    }
}
