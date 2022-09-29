use super::__markers::Minable;

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Min<L, R> {
    lhs: L,
    rhs: R,
}

impl<L, R> Min<L, R> {
    pub(crate) fn new(lhs: L, rhs: R) -> Self {
        Min { lhs, rhs }
    }
}

impl<L, R> Minable for Min<L, R>
where
    L: Minable,
    R: Minable<Unit = <L as Minable>::Unit>,
{
    type Unit = <L as Minable>::Unit;
}

impl<L, R> std::fmt::Display for Min<L, R>
where
    L: Minable,
    R: Minable,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "min({}, {})", self.lhs, self.rhs)
    }
}

impl<L, R, Rhs> crate::math::ops::Min<Rhs> for Min<L, R>
where
    L: Minable,
    R: Minable<Unit = <L as Minable>::Unit>,
    Rhs: Minable<Unit = <L as Minable>::Unit>,
{
    type Output = Min<Self, Rhs>;
    fn min(self, rhs: Rhs) -> Min<Self, Rhs> {
        Min::new(self, rhs)
    }
}

#[cfg(test)]
mod tests {
    use crate::{dimensions::*, math::ops::Min};

    #[test]
    fn test_display() {
        let l1 = Length::Vw(100.);
        let l2 = Length::Px(300.);
        assert_eq!(format!("{}", l1.min(l2)), "min(100vw, 300px)");
    }

    #[test]
    fn test_composition() {
        let l1 = Length::Vw(100.);
        let l2 = Length::Px(300.);
        let l3 = Length::In(3.);

        assert_eq!(
            format!("{}", l1.min(l2).min(l3)),
            "min(min(100vw, 300px), 3in)"
        );
    }
}
