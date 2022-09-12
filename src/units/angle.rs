use crate::functions::{Calculable, Calculation};

#[derive(Debug, Clone, PartialEq)]
pub enum Angle {
    Deg(f64),
    Grad(f64),
    Rad(f64),
    Turn(f64),
    Percent(f64),
    // Special
    Calc(Box<Calculation<Angle>>),
}

impl Calculable for Angle {}

impl std::fmt::Display for Angle {
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

impl std::ops::Add for Angle {
    type Output = Angle;
    fn add(self, rhs: Self) -> Self::Output {
        Angle::Calc(Box::new(Calculation::Add { lhs: self, rhs }))
    }
}

impl std::ops::Sub for Angle {
    type Output = Angle;
    fn sub(self, rhs: Self) -> Self::Output {
        Angle::Calc(Box::new(Calculation::Sub { lhs: self, rhs }))
    }
}

impl std::ops::Mul for Angle {
    type Output = Angle;
    fn mul(self, rhs: Self) -> Self::Output {
        Angle::Calc(Box::new(Calculation::Mul { lhs: self, rhs }))
    }
}

impl std::ops::Div for Angle {
    type Output = Angle;
    fn div(self, rhs: Self) -> Self::Output {
        Angle::Calc(Box::new(Calculation::Div { lhs: self, rhs }))
    }
}
