use crate::functions::{Calculable, Calculation};

#[derive(Debug, Clone, PartialEq)]
pub enum Time {
    Seconds(f64),
    Milliseconds(f64),
    Percent(f64),
    Duration(std::time::Duration),
    // Special
    Calc(Box<Calculation<Time>>),
}

impl Calculable for Time {}

impl std::fmt::Display for Time {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Time::Seconds(v) => write!(f, "{}s", v),
            Time::Milliseconds(v) => write!(f, "{}ms", v),
            Time::Percent(v) => write!(f, "{}%", v),
            Time::Duration(v) => write!(f, "{}ms", v.as_millis()),
            Time::Calc(v) => v.fmt(f),
        }
    }
}

impl std::ops::Add for Time {
    type Output = Time;
    fn add(self, rhs: Self) -> Self::Output {
        Time::Calc(Box::new(Calculation::Add { lhs: self, rhs }))
    }
}

impl std::ops::Sub for Time {
    type Output = Time;
    fn sub(self, rhs: Self) -> Self::Output {
        Time::Calc(Box::new(Calculation::Sub { lhs: self, rhs }))
    }
}

impl std::ops::Mul for Time {
    type Output = Time;
    fn mul(self, rhs: Self) -> Self::Output {
        Time::Calc(Box::new(Calculation::Mul { lhs: self, rhs }))
    }
}

impl std::ops::Div for Time {
    type Output = Time;
    fn div(self, rhs: Self) -> Self::Output {
        Time::Calc(Box::new(Calculation::Div { lhs: self, rhs }))
    }
}
