use std::marker::PhantomData;

use super::__markers::{Addable, Scalable};

#[derive(Debug, Clone, Copy, PartialEq)]
pub(crate) enum Operation {
    Add,
    Sub,
    Mul,
    Div,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Calculation<Unit, L, R> {
    lhs: L,
    rhs: R,
    op: Operation,
    _pd: PhantomData<Unit>,
}

impl<Unit, L, R> Calculation<Unit, L, R> {
    pub(crate) fn new(lhs: L, rhs: R, op: Operation) -> Self {
        Calculation {
            lhs,
            rhs,
            op,
            _pd: PhantomData,
        }
    }
}

impl<Unit, L, R> Addable<Unit> for Calculation<Unit, L, R>
where
    L: Addable<Unit>,
    R: Addable<Unit>,
{
}

impl<Unit, L, R> Scalable<Unit> for Calculation<Unit, L, R>
where
    L: Scalable<Unit>,
    R: Scalable<Unit>,
{
}

impl<Unit, L, R> std::fmt::Display for Calculation<Unit, L, R>
where
    L: std::fmt::Display,
    R: std::fmt::Display,
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

impl<Unit, L, R, Rhs> std::ops::Add<Rhs> for Calculation<Unit, L, R>
where
    L: Addable<Unit>,
    R: Addable<Unit>,
    Rhs: Addable<Unit>,
{
    type Output = Calculation<Unit, Self, Rhs>;
    fn add(self, rhs: Rhs) -> Calculation<Unit, Self, Rhs> {
        Calculation::new(self, rhs, Operation::Add)
    }
}

impl<Unit, L, R, Rhs> std::ops::Sub<Rhs> for Calculation<Unit, L, R>
where
    L: Addable<Unit>,
    R: Addable<Unit>,
    Rhs: Addable<Unit>,
{
    type Output = Calculation<Unit, Self, Rhs>;
    fn sub(self, rhs: Rhs) -> Calculation<Unit, Self, Rhs> {
        Calculation::new(self, rhs, Operation::Sub)
    }
}

impl<Unit, L, R, Rhs> std::ops::Mul<Rhs> for Calculation<Unit, L, R>
where
    L: Scalable<Unit>,
    R: Scalable<Unit>,
    Rhs: Scalable<Unit>,
{
    type Output = Calculation<Unit, Self, Rhs>;
    fn mul(self, rhs: Rhs) -> Calculation<Unit, Self, Rhs> {
        Calculation::new(self, rhs, Operation::Mul)
    }
}

impl<Unit, L, R, Rhs> std::ops::Div<Rhs> for Calculation<Unit, L, R>
where
    L: Scalable<Unit>,
    R: Scalable<Unit>,
    Rhs: Scalable<Unit>,
{
    type Output = Calculation<Unit, Self, Rhs>;
    fn div(self, rhs: Rhs) -> Calculation<Unit, Self, Rhs> {
        Calculation::new(self, rhs, Operation::Div)
    }
}

#[cfg(test)]
mod tests {
    use crate::prelude::*;

    #[test]
    fn test_calc() {
        let c1 = Length::Vw(100.);
        let c2 = Length::Vw(100.) + Length::Px(300.);
        assert_eq!(format!("{}", c1 + c2), "calc(100vw + calc(100vw + 300px))");
        assert_eq!(format!("{}", c1 - c2), "calc(100vw - calc(100vw + 300px))");
        assert_eq!(format!("{}", c1 * c2), "calc(100vw * calc(100vw + 300px))");
        assert_eq!(format!("{}", c1 / c2), "calc(100vw / calc(100vw + 300px))");
    }

    #[test]
    fn test_calc2() {
        let c1 = Length::Vw(100.) + Length::Px(300.);
        let c2 = Length::Vw(100.) + Length::Px(300.);
        assert_eq!(
            format!("{}", c1 + c2),
            "calc(calc(100vw + 300px) + calc(100vw + 300px))"
        );
        assert_eq!(
            format!("{}", c1 - c2),
            "calc(calc(100vw + 300px) - calc(100vw + 300px))"
        );
        assert_eq!(
            format!("{}", c1 * c2),
            "calc(calc(100vw + 300px) * calc(100vw + 300px))"
        );
        assert_eq!(
            format!("{}", c1 / c2),
            "calc(calc(100vw + 300px) / calc(100vw + 300px))"
        );
    }
}
