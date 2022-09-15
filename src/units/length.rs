use crate::functions::markers::Calculable;
use crate::functions::Calculation;

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

impl Calculable for Length {
    type Unit = Length;
}

impl<Rhs> std::ops::Add<Rhs> for Length {
    type Output = Calculation<Self, Rhs>;
    fn add(self, rhs: Rhs) -> Calculation<Self, Rhs> {
        Calculation::add(self, rhs)
    }
}

impl<Rhs> std::ops::Sub<Rhs> for Length {
    type Output = Calculation<Self, Rhs>;
    fn sub(self, rhs: Rhs) -> Calculation<Self, Rhs> {
        Calculation::sub(self, rhs)
    }
}

impl<Rhs> std::ops::Mul<Rhs> for Length {
    type Output = Calculation<Self, Rhs>;
    fn mul(self, rhs: Rhs) -> Calculation<Self, Rhs> {
        Calculation::mul(self, rhs)
    }
}

impl<Rhs> std::ops::Div<Rhs> for Length {
    type Output = Calculation<Self, Rhs>;
    fn div(self, rhs: Rhs) -> Calculation<Self, Rhs> {
        Calculation::div(self, rhs)
    }
}
