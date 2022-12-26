use crate::math::__markers::{Addable, Calculable, Scalable};
use std::marker::PhantomData;

// TODO: Rename to Operator
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

    #[test]
    fn test_composition_integer() {
        let int = 2;

        let lc1 = Length::Vw(100.) + Length::In(2.5);
        let lc2 = Length::Percent(25.) * int;
        assert_eq!(
            format!("{}", lc1 + lc2),
            "calc(calc(100vw + 2.5in) + calc(25% * 2))"
        );
        assert_eq!(
            format!("{}", lc1 - lc2),
            "calc(calc(100vw + 2.5in) - calc(25% * 2))"
        );
        assert_eq!(
            format!("{}", lc1 * lc2),
            "calc(calc(100vw + 2.5in) * calc(25% * 2))"
        );
        assert_eq!(
            format!("{}", lc1 / lc2),
            "calc(calc(100vw + 2.5in) / calc(25% * 2))"
        );

        let ac1 = Angle::Percent(50.) + Angle::Deg(180.);
        let ac2 = Angle::Percent(25.) * int;
        assert_eq!(
            format!("{}", ac1 + ac2),
            "calc(calc(50% + 180deg) + calc(25% * 2))"
        );
        assert_eq!(
            format!("{}", ac1 - ac2),
            "calc(calc(50% + 180deg) - calc(25% * 2))"
        );
        assert_eq!(
            format!("{}", ac1 * ac2),
            "calc(calc(50% + 180deg) * calc(25% * 2))"
        );
        assert_eq!(
            format!("{}", ac1 / ac2),
            "calc(calc(50% + 180deg) / calc(25% * 2))"
        );

        let rc1 = Resolution::Dpi(50.) + Resolution::Dppx(100.);
        let rc2 = Resolution::Dpi(25.) * int;
        assert_eq!(
            format!("{}", rc1 + rc2),
            "calc(calc(50dpi + 100dppx) + calc(25dpi * 2))"
        );
        assert_eq!(
            format!("{}", rc1 - rc2),
            "calc(calc(50dpi + 100dppx) - calc(25dpi * 2))"
        );
        assert_eq!(
            format!("{}", rc1 * rc2),
            "calc(calc(50dpi + 100dppx) * calc(25dpi * 2))"
        );
        assert_eq!(
            format!("{}", rc1 / rc2),
            "calc(calc(50dpi + 100dppx) / calc(25dpi * 2))"
        );

        let tc1 = Time::Seconds(5.) + Time::Milliseconds(500.);
        let tc2 = Time::Percent(25.) * int;
        assert_eq!(
            format!("{}", tc1 + tc2),
            "calc(calc(5s + 500ms) + calc(25% * 2))"
        );
        assert_eq!(
            format!("{}", tc1 - tc2),
            "calc(calc(5s + 500ms) - calc(25% * 2))"
        );
        assert_eq!(
            format!("{}", tc1 * tc2),
            "calc(calc(5s + 500ms) * calc(25% * 2))"
        );
        assert_eq!(
            format!("{}", tc1 / tc2),
            "calc(calc(5s + 500ms) / calc(25% * 2))"
        );
    }

    #[test]
    fn test_composition_number() {
        let num = 2.0;

        let lc1 = Length::Vw(100.) + Length::In(2.5);
        let lc2 = Length::Percent(25.) * num;
        assert_eq!(
            format!("{}", lc1 + lc2),
            "calc(calc(100vw + 2.5in) + calc(25% * 2))"
        );
        assert_eq!(
            format!("{}", lc1 - lc2),
            "calc(calc(100vw + 2.5in) - calc(25% * 2))"
        );
        assert_eq!(
            format!("{}", lc1 * lc2),
            "calc(calc(100vw + 2.5in) * calc(25% * 2))"
        );
        assert_eq!(
            format!("{}", lc1 / lc2),
            "calc(calc(100vw + 2.5in) / calc(25% * 2))"
        );

        let ac1 = Angle::Percent(50.) + Angle::Deg(180.);
        let ac2 = Angle::Percent(25.) * num;
        assert_eq!(
            format!("{}", ac1 + ac2),
            "calc(calc(50% + 180deg) + calc(25% * 2))"
        );
        assert_eq!(
            format!("{}", ac1 - ac2),
            "calc(calc(50% + 180deg) - calc(25% * 2))"
        );
        assert_eq!(
            format!("{}", ac1 * ac2),
            "calc(calc(50% + 180deg) * calc(25% * 2))"
        );
        assert_eq!(
            format!("{}", ac1 / ac2),
            "calc(calc(50% + 180deg) / calc(25% * 2))"
        );

        let rc1 = Resolution::Dpi(50.) + Resolution::Dppx(100.);
        let rc2 = Resolution::Dpi(25.) * num;
        assert_eq!(
            format!("{}", rc1 + rc2),
            "calc(calc(50dpi + 100dppx) + calc(25dpi * 2))"
        );
        assert_eq!(
            format!("{}", rc1 - rc2),
            "calc(calc(50dpi + 100dppx) - calc(25dpi * 2))"
        );
        assert_eq!(
            format!("{}", rc1 * rc2),
            "calc(calc(50dpi + 100dppx) * calc(25dpi * 2))"
        );
        assert_eq!(
            format!("{}", rc1 / rc2),
            "calc(calc(50dpi + 100dppx) / calc(25dpi * 2))"
        );

        let tc1 = Time::Seconds(5.) + Time::Milliseconds(500.);
        let tc2 = Time::Percent(25.) * num;
        assert_eq!(
            format!("{}", tc1 + tc2),
            "calc(calc(5s + 500ms) + calc(25% * 2))"
        );
        assert_eq!(
            format!("{}", tc1 - tc2),
            "calc(calc(5s + 500ms) - calc(25% * 2))"
        );
        assert_eq!(
            format!("{}", tc1 * tc2),
            "calc(calc(5s + 500ms) * calc(25% * 2))"
        );
        assert_eq!(
            format!("{}", tc1 / tc2),
            "calc(calc(5s + 500ms) / calc(25% * 2))"
        );
    }
}
