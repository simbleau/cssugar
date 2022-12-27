use crate::math::__markers::{Addable, Calculable, Comparable, Scalable};
use std::marker::PhantomData;

#[derive(Debug, Clone, Copy, PartialEq)]
pub(crate) enum Operation {
    Add,
    Sub,
    Mul,
    Div,
    Min,
    Max,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Function<Unit, L, R> {
    lhs: L,
    rhs: R,
    op: Operation,
    _pd: PhantomData<Unit>,
}

impl<Unit, L, R> Function<Unit, L, R> {
    pub(crate) fn new(lhs: L, rhs: R, op: Operation) -> Self {
        Self {
            lhs,
            rhs,
            op,
            _pd: PhantomData,
        }
    }
}

impl<Unit, L, R> std::fmt::Display for Function<Unit, L, R>
where
    L: std::fmt::Display,
    R: std::fmt::Display,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self.op {
                Operation::Add => format!("calc({}+{})", self.lhs, self.rhs),
                Operation::Sub => format!("calc({}-{})", self.lhs, self.rhs),
                Operation::Mul => format!("calc({}*{})", self.lhs, self.rhs),
                Operation::Div => format!("calc({}/{})", self.lhs, self.rhs),
                Operation::Min => format!("min({},{})", self.lhs, self.rhs),
                Operation::Max => format!("max({},{})", self.lhs, self.rhs),
            }
        )
    }
}

impl<Unit, L, R> Calculable<Unit> for Function<Unit, L, R>
where
    L: Calculable<Unit>,
    R: Calculable<Unit>,
{
}

impl<Unit, L, R> Addable<Unit> for Function<Unit, L, R>
where
    L: Calculable<Unit>,
    R: Calculable<Unit>,
{
}

impl<Unit, L, R> Scalable<Unit> for Function<Unit, L, R>
where
    L: Scalable<Unit>,
    R: Scalable<Unit>,
{
}

impl<Unit, L, R> Comparable<Unit> for Function<Unit, L, R>
where
    L: Comparable<Unit>,
    R: Comparable<Unit>,
{
    fn min<Rhs>(self, rhs: Rhs) -> Function<Unit, Self, Rhs> {
        Function::new(self, rhs, Operation::Min)
    }

    fn max<Rhs>(self, rhs: Rhs) -> Function<Unit, Self, Rhs> {
        Function::new(self, rhs, Operation::Max)
    }
}

impl<Unit, L, R, Rhs> std::ops::Add<Rhs> for Function<Unit, L, R>
where
    Rhs: Addable<Unit>,
{
    type Output = Function<Unit, Self, Rhs>;
    fn add(self, rhs: Rhs) -> Function<Unit, Self, Rhs> {
        Function::new(self, rhs, Operation::Add)
    }
}

impl<Unit, L, R, Rhs> std::ops::Sub<Rhs> for Function<Unit, L, R>
where
    Rhs: Addable<Unit>,
{
    type Output = Function<Unit, Self, Rhs>;
    fn sub(self, rhs: Rhs) -> Function<Unit, Self, Rhs> {
        Function::new(self, rhs, Operation::Sub)
    }
}

impl<Unit, L, R, Rhs> std::ops::Mul<Rhs> for Function<Unit, L, R>
where
    Rhs: Scalable<Unit>,
{
    type Output = Function<Unit, Self, Rhs>;
    fn mul(self, rhs: Rhs) -> Function<Unit, Self, Rhs> {
        Function::new(self, rhs, Operation::Mul)
    }
}

impl<Unit, L, R, Rhs> std::ops::Div<Rhs> for Function<Unit, L, R>
where
    Rhs: Scalable<Unit>,
{
    type Output = Function<Unit, Self, Rhs>;
    fn div(self, rhs: Rhs) -> Function<Unit, Self, Rhs> {
        Function::new(self, rhs, Operation::Div)
    }
}

#[cfg(test)]
mod tests {
    use crate::prelude::*;

    #[test]
    fn test_level1() {
        let l1 = Length::Vw(100.);
        let l2 = Length::Vw(100.);
        assert_eq!(format!("{}", l1 + l2), "calc(100vw+100vw)");
        assert_eq!(format!("{}", l1 - l2), "calc(100vw-100vw)");
        assert_eq!(format!("{}", l1 * l2), "calc(100vw*100vw)");
        assert_eq!(format!("{}", l1 / l2), "calc(100vw/100vw)");
        assert_eq!(format!("{}", l1.min(l2)), "min(100vw,100vw)");
        assert_eq!(format!("{}", l1.max(l2)), "max(100vw,100vw)");
    }

    #[test]
    fn test_level2() {
        let l1 = Length::Vw(100.);
        let l2 = Length::Vw(100.) + Length::Px(300.);
        assert_eq!(format!("{}", l1 + l2), "calc(100vw+calc(100vw+300px))");
        assert_eq!(format!("{}", l1 - l2), "calc(100vw-calc(100vw+300px))");
        assert_eq!(format!("{}", l1 * l2), "calc(100vw*calc(100vw+300px))");
        assert_eq!(format!("{}", l1 / l2), "calc(100vw/calc(100vw+300px))");
        assert_eq!(format!("{}", l1.min(l2)), "min(100vw,calc(100vw+300px))");
        assert_eq!(format!("{}", l1.max(l2)), "max(100vw,calc(100vw+300px))");
    }

    #[test]
    fn test_level3() {
        let l1 = Length::Px(50.0) + Length::Px(50.);
        let l2 = Length::Vw(100.) + Length::Px(300.);
        assert_eq!(
            format!("{}", l1 + l2),
            "calc(calc(50px+50px)+calc(100vw+300px))"
        );
        assert_eq!(
            format!("{}", l1 - l2),
            "calc(calc(50px+50px)-calc(100vw+300px))"
        );
        assert_eq!(
            format!("{}", l1 * l2),
            "calc(calc(50px+50px)*calc(100vw+300px))"
        );
        assert_eq!(
            format!("{}", l1 / l2),
            "calc(calc(50px+50px)/calc(100vw+300px))"
        );
        assert_eq!(
            format!("{}", l1.min(l2)),
            "min(calc(50px+50px),calc(100vw+300px))"
        );
        assert_eq!(
            format!("{}", l1.max(l2)),
            "max(calc(50px+50px),calc(100vw+300px))"
        );
    }
}
