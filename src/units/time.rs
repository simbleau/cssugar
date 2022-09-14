use crate::functions::markers::{self, Calculable};
use crate::functions::Calculation;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Time {
    Seconds(f64),
    Milliseconds(f64),
    Percent(f64),
    Duration(std::time::Duration),
}

impl Calculable<Time> for Time {}

impl std::fmt::Display for Time {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Time::Seconds(v) => write!(f, "{}s", v),
            Time::Milliseconds(v) => write!(f, "{}ms", v),
            Time::Percent(v) => write!(f, "{}%", v),
            Time::Duration(v) => write!(f, "{}ms", v.as_millis()),
        }
    }
}

impl std::ops::Add for Time {
    type Output = Calculation<Time, Time, Time>;
    fn add(self, rhs: Self) -> Self::Output {
        Calculation::add(self, rhs)
    }
}

impl std::ops::Sub for Time {
    type Output = Calculation<Time, Time, Time>;
    fn sub(self, rhs: Self) -> Self::Output {
        Calculation::sub(self, rhs)
    }
}

impl std::ops::Mul for Time {
    type Output = Calculation<Time, Time, Time>;
    fn mul(self, rhs: Self) -> Self::Output {
        Calculation::mul(self, rhs)
    }
}

impl std::ops::Div for Time {
    type Output = Calculation<Time, Time, Time>;
    fn div(self, rhs: Self) -> Self::Output {
        Calculation::div(self, rhs)
    }
}
