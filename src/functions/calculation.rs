use std::marker::PhantomData;

use super::markers::{self, Calculable};

#[derive(Debug, Clone, Copy, PartialEq)]
enum Operation {
    Add,
    Sub,
    Mul,
    Div,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Calculation<A, T, K>
where
    T: super::markers::Calculable<A>,
    K: super::markers::Calculable<A>,
{
    pub lhs: T,
    pub rhs: K,
    op: Operation,
    _phantom: PhantomData<A>,
}

impl<A, T: markers::Calculable<A>, K: markers::Calculable<A>>
    Calculation<A, T, K>
{
    pub fn add(lhs: T, rhs: K) -> Self {
        Calculation {
            lhs,
            rhs,
            op: Operation::Add,
            _phantom: PhantomData,
        }
    }

    pub fn sub(lhs: T, rhs: K) -> Self {
        Calculation {
            lhs,
            rhs,
            op: Operation::Sub,
            _phantom: PhantomData,
        }
    }

    pub fn mul(lhs: T, rhs: K) -> Self {
        Calculation {
            lhs,
            rhs,
            op: Operation::Mul,
            _phantom: PhantomData,
        }
    }

    pub fn div(lhs: T, rhs: K) -> Self {
        Calculation {
            lhs,
            rhs,
            op: Operation::Div,
            _phantom: PhantomData,
        }
    }
}

impl<A, T: markers::Calculable<A>, K: markers::Calculable<A>> std::fmt::Display
    for Calculation<A, T, K>
{
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

    #[test]
    fn test_composition() {
        let l1 = Length::Vw(100.);
        let l2 = Length::Px(300.);
        let l3 = Length::In(3.);
        assert_eq!(
            format!("{}", l1 + l2 + l3),
            "calc(calc(100vw + 300px), 3in)"
        );
    }
}
