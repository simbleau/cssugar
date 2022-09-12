pub(crate) mod markers {
    pub trait Calculable: std::fmt::Display {}
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
        match self {
            Calculation::Add { lhs, rhs } => write!(f, "calc({} + {})", lhs, rhs),
            Calculation::Sub { lhs, rhs } => write!(f, "calc({} - {})", lhs, rhs),
            Calculation::Mul { lhs, rhs } => write!(f, "calc({} * {})", lhs, rhs),
            Calculation::Div { lhs, rhs } => write!(f, "calc({} / {})", lhs, rhs),
        }
    }
}
