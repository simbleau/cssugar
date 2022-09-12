use crate::units::Length;

#[derive(Debug, Clone, PartialEq)]
pub enum LengthCalculation {
    Add {
        left: Box<Length>,
        right: Box<Length>,
    },
    Sub {
        left: Box<Length>,
        right: Box<Length>,
    },
    Mul {
        left: Box<Length>,
        right: Box<Length>,
    },
    Div {
        left: Box<Length>,
        right: Box<Length>,
    },
}

impl std::fmt::Display for LengthCalculation {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            LengthCalculation::Add { left, right } => write!(f, "calc({} + {})", left, right),
            LengthCalculation::Sub { left, right } => write!(f, "calc({} - {})", left, right),
            LengthCalculation::Mul { left, right } => write!(f, "calc({} * {})", left, right),
            LengthCalculation::Div { left, right } => write!(f, "calc({} / {})", left, right),
        }
    }
}
