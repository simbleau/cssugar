use crate::functions::markers::Calculable;
use crate::functions::Calculation;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Angle {
    Deg(f64),
    Grad(f64),
    Rad(f64),
    Turn(f64),
    Percent(f64),
}

impl Calculable<Angle> for Angle {}

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

impl std::ops::Add for Angle {
    type Output = Calculation<Angle, Angle, Angle>;
    fn add(self, rhs: Self) -> Self::Output {
        Calculation::add(self, rhs)
    }
}

impl std::ops::Sub for Angle {
    type Output = Calculation<Angle, Angle, Angle>;
    fn sub(self, rhs: Self) -> Self::Output {
        Calculation::sub(self, rhs)
    }
}

impl std::ops::Mul for Angle {
    type Output = Calculation<Angle, Angle, Angle>;
    fn mul(self, rhs: Self) -> Self::Output {
        Calculation::mul(self, rhs)
    }
}

impl std::ops::Div for Angle {
    type Output = Calculation<Angle, Angle, Angle>;
    fn div(self, rhs: Self) -> Self::Output {
        Calculation::div(self, rhs)
    }
}
