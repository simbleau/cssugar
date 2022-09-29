use crate::math::calculation::Operation;
use crate::math::{markers::Calculable, Calculation};

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Time {
    Seconds(f64),
    Milliseconds(f64),
    Percent(f64),
    Duration(std::time::Duration),
}

impl Calculable for Time {
    type Unit = Time;
}

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

impl<Rhs> std::ops::Add<Rhs> for Time {
    type Output = Calculation<Self, Rhs>;
    fn add(self, rhs: Rhs) -> Calculation<Self, Rhs> {
        Calculation::new(self, rhs, Operation::Add)
    }
}

impl<Rhs> std::ops::Sub<Rhs> for Time {
    type Output = Calculation<Self, Rhs>;
    fn sub(self, rhs: Rhs) -> Calculation<Self, Rhs> {
        Calculation::new(self, rhs, Operation::Sub)
    }
}

impl<Rhs> std::ops::Mul<Rhs> for Time {
    type Output = Calculation<Self, Rhs>;
    fn mul(self, rhs: Rhs) -> Calculation<Self, Rhs> {
        Calculation::new(self, rhs, Operation::Mul)
    }
}

impl<Rhs> std::ops::Div<Rhs> for Time {
    type Output = Calculation<Self, Rhs>;
    fn div(self, rhs: Rhs) -> Calculation<Self, Rhs> {
        Calculation::new(self, rhs, Operation::Div)
    }
}
