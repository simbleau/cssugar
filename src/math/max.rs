use super::__markers::Maxable;

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Max<L, R> {
    lhs: L,
    rhs: R,
}

impl<L, R> Max<L, R> {
    pub(crate) fn new(lhs: L, rhs: R) -> Self {
        Max { lhs, rhs }
    }
}

impl<L, R> Maxable for Max<L, R>
where
    L: Maxable,
    R: Maxable<Unit = <L as Maxable>::Unit>,
{
    type Unit = <L as Maxable>::Unit;
}

impl<L, R> std::fmt::Display for Max<L, R>
where
    L: Maxable,
    R: Maxable,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "max({}, {})", self.lhs, self.rhs)
    }
}

impl<L, R, Rhs> crate::math::ops::Max<Rhs> for Max<L, R>
where
    L: Maxable,
    R: Maxable<Unit = <L as Maxable>::Unit>,
    Rhs: Maxable<Unit = <L as Maxable>::Unit>,
{
    type Output = Max<Self, Rhs>;
    fn max(self, rhs: Rhs) -> Max<Self, Rhs> {
        Max::new(self, rhs)
    }
}

#[cfg(test)]
mod tests {
    use crate::{dimensions::*, math::ops::Max};

    #[test]
    fn test_display() {
        let l1 = Length::Vw(100.);
        let l2 = Length::Px(300.);
        assert_eq!(format!("{}", l1.max(l2)), "max(100vw, 300px)");
    }

    #[test]
    fn test_composition() {
        let l1 = Length::Vw(100.);
        let l2 = Length::Px(300.);
        let l3 = Length::In(3.);

        assert_eq!(
            format!("{}", l1.max(l2).max(l3)),
            "max(max(100vw, 300px), 3in)"
        );
    }
}
