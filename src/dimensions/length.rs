use crate::math::{
    calculation::Operation,
    markers::{Addable, Maxable, Minable, Scalable},
    Calculation, Max, Min,
};

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Length {
    // Relative
    Em(f64),
    Ex(f64),
    Rem(f64),
    Vh(f64),
    Vw(f64),
    Vmax(f64),
    Vmin(f64),
    Percent(f64),
    // Absolute
    Px(f64),
    Cm(f64),
    Mm(f64),
    In(f64),
    Pt(f64),
}

impl Addable<Length> for Length {}
impl Scalable<Length> for Length {}

impl Maxable for Length {
    type Unit = Length;
}

impl Minable for Length {
    type Unit = Length;
}

impl std::fmt::Display for Length {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Length::Em(v) => write!(f, "{}em", v),
            Length::Ex(v) => write!(f, "{}ex", v),
            Length::Rem(v) => write!(f, "{}rem", v),
            Length::Vh(v) => write!(f, "{}vh", v),
            Length::Vw(v) => write!(f, "{}vw", v),
            Length::Vmax(v) => write!(f, "{}vmax", v),
            Length::Vmin(v) => write!(f, "{}vmin", v),
            Length::Percent(v) => write!(f, "{}%", v),
            Length::Px(v) => write!(f, "{}px", v),
            Length::Cm(v) => write!(f, "{}cm", v),
            Length::Mm(v) => write!(f, "{}mm", v),
            Length::In(v) => write!(f, "{}in", v),
            Length::Pt(v) => write!(f, "{}pt", v),
        }
    }
}

impl<Rhs> crate::math::ops::Max<Rhs> for Length {
    type Output = Max<Self, Rhs>;
    fn max(self, rhs: Rhs) -> Max<Self, Rhs> {
        Max::new(self, rhs)
    }
}

impl<Rhs> crate::math::ops::Min<Rhs> for Length {
    type Output = Min<Self, Rhs>;
    fn min(self, rhs: Rhs) -> Min<Self, Rhs> {
        Min::new(self, rhs)
    }
}

impl<Rhs> std::ops::Add<Rhs> for Length
where
    Rhs: Addable<Length>,
{
    type Output = Calculation<Length, Self, Rhs>;
    fn add(self, rhs: Rhs) -> Calculation<Length, Self, Rhs> {
        Calculation::new(self, rhs, Operation::Add)
    }
}

impl<Rhs> std::ops::Sub<Rhs> for Length
where
    Rhs: Addable<Length>,
{
    type Output = Calculation<Length, Self, Rhs>;
    fn sub(self, rhs: Rhs) -> Calculation<Length, Self, Rhs> {
        Calculation::new(self, rhs, Operation::Sub)
    }
}

impl<Rhs> std::ops::Mul<Rhs> for Length
where
    Rhs: Scalable<Length>,
{
    type Output = Calculation<Length, Self, Rhs>;
    fn mul(self, rhs: Rhs) -> Calculation<Length, Self, Rhs> {
        Calculation::new(self, rhs, Operation::Mul)
    }
}

impl<Rhs> std::ops::Div<Rhs> for Length
where
    Rhs: Scalable<Length>,
{
    type Output = Calculation<Length, Self, Rhs>;
    fn div(self, rhs: Rhs) -> Calculation<Length, Self, Rhs> {
        Calculation::new(self, rhs, Operation::Div)
    }
}

#[cfg(test)]
mod tests {
    use crate::prelude::*;

    #[test]
    fn test_calc() {
        let l1 = Length::Vw(100.);
        let l2 = Length::Px(300.);
        assert_eq!(format!("{}", l1 + l2), "calc(100vw + 300px)");
        assert_eq!(format!("{}", l1 - l2), "calc(100vw - 300px)");
        assert_eq!(format!("{}", l1 * l2), "calc(100vw * 300px)");
        assert_eq!(format!("{}", l1 / l2), "calc(100vw / 300px)");
    }

    #[test]
    fn test_max() {
        let l1 = Length::Vw(100.);
        let l2 = Length::Px(300.);
        assert_eq!(format!("{}", l1.max(l2)), "max(100vw, 300px)");
    }

    #[test]
    fn test_min() {
        let l1 = Length::Vw(100.);
        let l2 = Length::Px(300.);
        assert_eq!(format!("{}", l1.min(l2)), "min(100vw, 300px)");
    }
}
