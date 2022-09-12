pub(crate) mod markers {
    pub trait Calculable:
        std::fmt::Display
        + std::ops::Add
        + std::ops::Sub
        + std::ops::Mul
        + std::ops::Div
        + Sized
    {
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum Calculation<T: markers::Calculable> {
    Add { lhs: T, rhs: T },
    Sub { lhs: T, rhs: T },
    Mul { lhs: T, rhs: T },
    Div { lhs: T, rhs: T },
}

impl<T: markers::Calculable> std::fmt::Display for Calculation<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "calc({})",
            match self {
                Calculation::Add { lhs, rhs } => format!("{} + {}", lhs, rhs),
                Calculation::Sub { lhs, rhs } => format!("{} - {}", lhs, rhs),
                Calculation::Mul { lhs, rhs } => format!("{} * {}", lhs, rhs),
                Calculation::Div { lhs, rhs } => format!("{} / {}", lhs, rhs),
            }
        )
    }
}
