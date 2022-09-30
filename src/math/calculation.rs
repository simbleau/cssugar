use super::__markers::{Addable, Scalable};

#[derive(Debug, Clone, Copy, PartialEq)]
pub(crate) enum Operation {
    Add,
    Sub,
    Mul,
    Div,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Calculation<L, R> {
    lhs: L,
    rhs: R,
    op: Operation,
}

impl<L, R> Calculation<L, R> {
    pub(crate) fn new(lhs: L, rhs: R, op: Operation) -> Self {
        Calculation { lhs, rhs, op }
    }
}

impl<L, R> Addable for Calculation<L, R>
where
    L: Addable,
    R: Addable<Unit = <L as Addable>::Unit>,
{
    type Unit = <L as Addable>::Unit;
}

impl<L, R> Scalable for Calculation<L, R>
where
    L: Scalable,
    R: Scalable<Unit = <L as Scalable>::Unit>,
{
    type Unit = <L as Scalable>::Unit;
}

impl<L, R> std::fmt::Display for Calculation<L, R>
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

impl<L, R, Rhs> std::ops::Add<Rhs> for Calculation<L, R>
where
    L: Addable,
    R: Addable<Unit = <L as Addable>::Unit>,
    Rhs: Addable<Unit = <L as Addable>::Unit>,
{
    type Output = Calculation<Self, Rhs>;
    fn add(self, rhs: Rhs) -> Calculation<Self, Rhs> {
        Calculation::new(self, rhs, Operation::Add)
    }
}

impl<L, R, Rhs> std::ops::Sub<Rhs> for Calculation<L, R>
where
    L: Addable,
    R: Addable<Unit = <L as Addable>::Unit>,
    Rhs: Addable<Unit = <L as Addable>::Unit>,
{
    type Output = Calculation<Self, Rhs>;
    fn sub(self, rhs: Rhs) -> Calculation<Self, Rhs> {
        Calculation::new(self, rhs, Operation::Sub)
    }
}

impl<L, R, Rhs> std::ops::Mul<Rhs> for Calculation<L, R>
where
    L: Scalable,
    R: Scalable<Unit = <L as Scalable>::Unit>,
    Rhs: Scalable<Unit = <L as Scalable>::Unit>,
{
    type Output = Calculation<Self, Rhs>;
    fn mul(self, rhs: Rhs) -> Calculation<Self, Rhs> {
        Calculation::new(self, rhs, Operation::Mul)
    }
}

impl<L, R, Rhs> std::ops::Div<Rhs> for Calculation<L, R>
where
    L: Scalable,
    R: Scalable<Unit = <L as Scalable>::Unit>,
    Rhs: Scalable<Unit = <L as Scalable>::Unit>,
{
    type Output = Calculation<Self, Rhs>;
    fn div(self, rhs: Rhs) -> Calculation<Self, Rhs> {
        Calculation::new(self, rhs, Operation::Div)
    }
}

#[cfg(test)]
mod tests {
    use crate::dimensions::*;

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
