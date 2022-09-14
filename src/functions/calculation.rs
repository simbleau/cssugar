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
pub struct Calculation<A, L, R>
where
    L: Calculable<A>,
    R: Calculable<A>,
{
    pub lhs: L,
    pub rhs: R,
    op: Operation,
    _pd: PhantomData<A>,
}

impl<A, L: Calculable<A>, R: Calculable<A>> Calculation<A, L, R> {
    pub fn add(lhs: L, rhs: R) -> Self {
        Calculation {
            lhs,
            rhs,
            op: Operation::Add,
            _pd: PhantomData,
        }
    }

    pub fn sub(lhs: L, rhs: R) -> Self {
        Calculation {
            lhs,
            rhs,
            op: Operation::Sub,
            _pd: PhantomData,
        }
    }

    pub fn mul(lhs: L, rhs: R) -> Self {
        Calculation {
            lhs,
            rhs,
            op: Operation::Mul,
            _pd: PhantomData,
        }
    }

    pub fn div(lhs: L, rhs: R) -> Self {
        Calculation {
            lhs,
            rhs,
            op: Operation::Div,
            _pd: PhantomData,
        }
    }
}

impl<A, L: markers::Calculable<A>, R: markers::Calculable<A>> std::fmt::Display
    for Calculation<A, L, R>
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

impl<A, L, R> Calculable<A> for Calculation<A, L, R>
where
    L: Calculable<A>,
    R: Calculable<A>,
{
}

impl<A, L, R, Other> std::ops::Add<Other> for Calculation<A, L, R>
where
    L: Calculable<A>,
    R: Calculable<A>,
    Other: Calculable<A>,
{
    type Output = Calculation<A, Self, Other>;
    fn add(self, rhs: Other) -> Calculation<A, Self, Other> {
        Calculation {
            lhs: self,
            rhs,
            op: Operation::Add,
            _pd: std::marker::PhantomData,
        }
    }
}

impl<A, L, R, Other> std::ops::Sub<Other> for Calculation<A, L, R>
where
    L: Calculable<A>,
    R: Calculable<A>,
    Other: Calculable<A>,
{
    type Output = Calculation<A, Self, Other>;
    fn sub(self, rhs: Other) -> Calculation<A, Self, Other> {
        Calculation {
            lhs: self,
            rhs,
            op: Operation::Sub,
            _pd: std::marker::PhantomData,
        }
    }
}

impl<A, L, R, Other> std::ops::Mul<Other> for Calculation<A, L, R>
where
    L: Calculable<A>,
    R: Calculable<A>,
    Other: Calculable<A>,
{
    type Output = Calculation<A, Self, Other>;
    fn mul(self, rhs: Other) -> Calculation<A, Self, Other> {
        Calculation {
            lhs: self,
            rhs,
            op: Operation::Mul,
            _pd: std::marker::PhantomData,
        }
    }
}

impl<A, L, R, Other> std::ops::Div<Other> for Calculation<A, L, R>
where
    L: Calculable<A>,
    R: Calculable<A>,
    Other: Calculable<A>,
{
    type Output = Calculation<A, Self, Other>;
    fn div(self, rhs: Other) -> Calculation<A, Self, Other> {
        Calculation {
            lhs: self,
            rhs,
            op: Operation::Div,
            _pd: std::marker::PhantomData,
        }
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
            "calc(calc(100vw + 300px) + 3in)"
        );
    }
}
