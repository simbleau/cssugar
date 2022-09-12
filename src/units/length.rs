use crate::functions::{Calculable, Calculation};

#[derive(Debug, Clone, PartialEq)]
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
    // Special
    Calc(Box<Calculation<Length>>),
}

impl Calculable for Length {}

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
            Length::Calc(v) => v.fmt(f),
        }
    }
}

impl std::ops::Add for Length {
    type Output = Length;
    fn add(self, rhs: Self) -> Self::Output {
        Length::Calc(Box::new(Calculation::Add { lhs: self, rhs }))
    }
}

impl std::ops::Sub for Length {
    type Output = Length;
    fn sub(self, rhs: Self) -> Self::Output {
        Length::Calc(Box::new(Calculation::Sub { lhs: self, rhs }))
    }
}

impl std::ops::Mul for Length {
    type Output = Length;
    fn mul(self, rhs: Self) -> Self::Output {
        Length::Calc(Box::new(Calculation::Mul { lhs: self, rhs }))
    }
}

impl std::ops::Div for Length {
    type Output = Length;
    fn div(self, rhs: Self) -> Self::Output {
        Length::Calc(Box::new(Calculation::Div { lhs: self, rhs }))
    }
}
