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

#[derive(Debug, Clone, Copy, PartialEq)]
enum Operation {
    Add,
    Sub,
    Mul,
    Div,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Calculation<T: markers::Calculable> {
    pub lhs: T,
    pub rhs: T,
    op: Operation,
}

impl<T: markers::Calculable> Calculation<T> {
    pub fn add(lhs: T, rhs: T) -> Self {
        Calculation {
            lhs,
            rhs,
            op: Operation::Add,
        }
    }

    pub fn sub(lhs: T, rhs: T) -> Self {
        Calculation {
            lhs,
            rhs,
            op: Operation::Sub,
        }
    }

    pub fn mul(lhs: T, rhs: T) -> Self {
        Calculation {
            lhs,
            rhs,
            op: Operation::Mul,
        }
    }

    pub fn div(lhs: T, rhs: T) -> Self {
        Calculation {
            lhs,
            rhs,
            op: Operation::Div,
        }
    }
}

impl<T: markers::Calculable> std::fmt::Display for Calculation<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "calc({} {} {})",
            self.lhs,
            match self.op {
                Operation::Add => "+",
                Operation::Sub => "-",
                Operation::Mul => "*",
                Operation::Div => "/",
            },
            self.rhs
        )
    }
}

#[cfg(test)]
mod tests {
    use crate::units::*;

    #[test]
    fn test_display() {
        let l1 = Length::Vw(100.);
        let l2 = Length::Px(300.);
        assert_eq!(format!("{}", l1 + l2), "calc(100vw + 300px)");
        assert_eq!(format!("{}", l1 - l2), "calc(100vw - 300px)");
        assert_eq!(format!("{}", l1 * l2), "calc(100vw * 300px)");
        assert_eq!(format!("{}", l1 / l2), "calc(100vw / 300px)");
    }
}
