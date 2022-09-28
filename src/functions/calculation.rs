use super::markers::Calculable;

#[derive(Debug, Clone, Copy, PartialEq)]
enum Operation {
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
    pub(crate) fn add(lhs: L, rhs: R) -> Self {
        Calculation {
            lhs,
            rhs,
            op: Operation::Add,
        }
    }
    pub(crate) fn sub(lhs: L, rhs: R) -> Self {
        Calculation {
            lhs,
            rhs,
            op: Operation::Sub,
        }
    }
    pub(crate) fn mul(lhs: L, rhs: R) -> Self {
        Calculation {
            lhs,
            rhs,
            op: Operation::Mul,
        }
    }
    pub(crate) fn div(lhs: L, rhs: R) -> Self {
        Calculation {
            lhs,
            rhs,
            op: Operation::Div,
        }
    }
}

impl<L, R> Calculable for Calculation<L, R>
where
    L: Calculable,
    R: Calculable<Unit = <L as Calculable>::Unit>,
{
    type Unit = <L as Calculable>::Unit;
}

impl<L, R> std::fmt::Display for Calculation<L, R>
where
    L: Calculable,
    R: Calculable,
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
    L: Calculable,
    R: Calculable<Unit = <L as Calculable>::Unit>,
    Rhs: Calculable<Unit = <L as Calculable>::Unit>,
{
    type Output = Calculation<Self, Rhs>;
    fn add(self, rhs: Rhs) -> Calculation<Self, Rhs> {
        Calculation::add(self, rhs)
    }
}

impl<L, R, Rhs> std::ops::Sub<Rhs> for Calculation<L, R>
where
    L: Calculable,
    R: Calculable<Unit = <L as Calculable>::Unit>,
    Rhs: Calculable<Unit = <L as Calculable>::Unit>,
{
    type Output = Calculation<Self, Rhs>;
    fn sub(self, rhs: Rhs) -> Calculation<Self, Rhs> {
        Calculation::sub(self, rhs)
    }
}

impl<L, R, Rhs> std::ops::Mul<Rhs> for Calculation<L, R>
where
    L: Calculable,
    R: Calculable<Unit = <L as Calculable>::Unit>,
    Rhs: Calculable<Unit = <L as Calculable>::Unit>,
{
    type Output = Calculation<Self, Rhs>;
    fn mul(self, rhs: Rhs) -> Calculation<Self, Rhs> {
        Calculation::mul(self, rhs)
    }
}

impl<L, R, Rhs> std::ops::Div<Rhs> for Calculation<L, R>
where
    L: Calculable,
    R: Calculable<Unit = <L as Calculable>::Unit>,
    Rhs: Calculable<Unit = <L as Calculable>::Unit>,
{
    type Output = Calculation<Self, Rhs>;
    fn div(self, rhs: Rhs) -> Calculation<Self, Rhs> {
        Calculation::div(self, rhs)
    }
}

#[cfg(test)]
mod tests {
    use crate::dimensions::*;

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
