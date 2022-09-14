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

impl Calculable<Length> for Length {}

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

impl<T: Calculable<Length>> std::ops::Add<T> for Length {
    type Output = Calculation<Length, Length, T>;
    fn add(self, rhs: T) -> Self::Output {
        Calculation::add(self, rhs)
    }
}

impl<T: Calculable<Length>> std::ops::Sub<T> for Length {
    type Output = Calculation<Length, Length, T>;
    fn sub(self, rhs: T) -> Self::Output {
        Calculation::sub(self, rhs)
    }
}

impl<T: Calculable<Length>> std::ops::Mul<T> for Length {
    type Output = Calculation<Length, Length, T>;
    fn mul(self, rhs: T) -> Self::Output {
        Calculation::mul(self, rhs)
    }
}

impl<T: Calculable<Length>> std::ops::Div<T> for Length {
    type Output = Calculation<Length, Length, T>;
    fn div(self, rhs: T) -> Self::Output {
        Calculation::div(self, rhs)
    }
}
