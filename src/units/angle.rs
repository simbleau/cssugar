use crate::functions::{Calculable, Calculation};

#[derive(Debug, Clone, PartialEq)]
pub enum Angle<'a> {
    Deg(f64),
    Grad(f64),
    Rad(f64),
    Turn(f64),
    Percent(f64),
    // Special
    Calc(Calculation<&'a Angle<'a>>),
}

impl<'a> Calculable for &'a Angle<'a> {}

impl std::fmt::Display for Angle<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Angle::Deg(v) => write!(f, "{}deg", v),
            Angle::Grad(v) => write!(f, "{}grad", v),
            Angle::Rad(v) => write!(f, "{}rad", v),
            Angle::Turn(v) => write!(f, "{}turn", v),
            Angle::Percent(v) => write!(f, "{}%", v),
            Angle::Calc(v) => v.fmt(f),
        }
    }
}

impl<'a> std::ops::Add for &'a Angle<'a> {
    type Output = Angle<'a>;
    fn add(self, rhs: Self) -> Self::Output {
        Angle::Calc(Calculation {
            lhs: &self,
            rhs: &rhs,
        })
    }
}

impl<'a> std::ops::Sub for &'a Angle<'a> {
    type Output = Angle<'a>;
    fn sub(self, rhs: Self) -> Self::Output {
        Angle::Calc(Calculation {
            lhs: &self,
            rhs: &rhs,
        })
    }
}

impl<'a> std::ops::Mul for &'a Angle<'a> {
    type Output = Angle<'a>;
    fn mul(self, rhs: Self) -> Self::Output {
        Angle::Calc(Calculation {
            lhs: &self,
            rhs: &rhs,
        })
    }
}

impl<'a> std::ops::Div for &'a Angle<'a> {
    type Output = Angle<'a>;
    fn div(self, rhs: Self) -> Self::Output {
        Angle::Calc(Calculation {
            lhs: &self,
            rhs: &rhs,
        })
    }
}
