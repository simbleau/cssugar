use crate::math::{
    calculation::Operation,
    markers::{Addable, Maxable, Minable, Scalable},
    Calculation, Max, Min,
};

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Angle {
    Deg(f64),
    Grad(f64),
    Rad(f64),
    Turn(f64),
    Percent(f64),
}

impl Addable for Angle {
    type Unit = Angle;
}

impl Scalable for Angle {
    type Unit = Angle;
}

impl Maxable for Angle {
    type Unit = Angle;
}

impl Minable for Angle {
    type Unit = Angle;
}

impl std::fmt::Display for Angle {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Angle::Deg(v) => write!(f, "{}deg", v),
            Angle::Grad(v) => write!(f, "{}grad", v),
            Angle::Rad(v) => write!(f, "{}rad", v),
            Angle::Turn(v) => write!(f, "{}turn", v),
            Angle::Percent(v) => write!(f, "{}%", v),
        }
    }
}

impl<Rhs> crate::math::ops::Max<Rhs> for Angle {
    type Output = Max<Self, Rhs>;
    fn max(self, rhs: Rhs) -> Max<Self, Rhs> {
        Max::new(self, rhs)
    }
}

impl<Rhs> crate::math::ops::Min<Rhs> for Angle {
    type Output = Min<Self, Rhs>;
    fn min(self, rhs: Rhs) -> Min<Self, Rhs> {
        Min::new(self, rhs)
    }
}

impl<Rhs> std::ops::Add<Rhs> for Angle
where
    Rhs: Addable,
{
    type Output = Calculation<Self, Rhs>;
    fn add(self, rhs: Rhs) -> Calculation<Self, Rhs> {
        Calculation::new(self, rhs, Operation::Add)
    }
}

impl<Rhs> std::ops::Sub<Rhs> for Angle
where
    Rhs: Addable,
{
    type Output = Calculation<Self, Rhs>;
    fn sub(self, rhs: Rhs) -> Calculation<Self, Rhs> {
        Calculation::new(self, rhs, Operation::Sub)
    }
}

impl<Rhs> std::ops::Mul<Rhs> for Angle
where
    Rhs: Scalable,
{
    type Output = Calculation<Self, Rhs>;
    fn mul(self, rhs: Rhs) -> Calculation<Self, Rhs> {
        Calculation::new(self, rhs, Operation::Mul)
    }
}

impl<Rhs> std::ops::Div<Rhs> for Angle
where
    Rhs: Scalable,
{
    type Output = Calculation<Self, Rhs>;
    fn div(self, rhs: Rhs) -> Calculation<Self, Rhs> {
        Calculation::new(self, rhs, Operation::Div)
    }
}
#[cfg(test)]
mod tests {
    use crate::{dimensions::*, math::ops::Max, math::ops::Min};

    #[test]
    fn test_calc() {
        let a1 = Angle::Deg(100.);
        let a2 = Angle::Percent(50.);
        assert_eq!(format!("{}", a1 + a2), "calc(100deg + 50%)");
        assert_eq!(format!("{}", a1 - a2), "calc(100deg - 50%)");
        assert_eq!(format!("{}", a1 * a2), "calc(100deg * 50%)");
        assert_eq!(format!("{}", a1 / a2), "calc(100deg / 50%)");
    }

    #[test]
    fn test_max() {
        let a1 = Angle::Deg(100.);
        let a2 = Angle::Percent(50.);
        assert_eq!(format!("{}", a1.max(a2)), "max(100deg, 50%)");
    }

    #[test]
    fn test_min() {
        let a1 = Angle::Deg(100.);
        let a2 = Angle::Percent(50.);
        assert_eq!(format!("{}", a1.min(a2)), "min(100deg, 50%)");
    }
}
