use crate::math::{
    calculation::Operation,
    markers::{Calculable, Maxable, Minable},
    Calculation, Max, Min,
};

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Resolution {
    Dpi(f64),
    Dpcm(f64),
    Dppx(f64),
}

impl Calculable for Resolution {
    type Unit = Resolution;
}

impl Maxable for Resolution {
    type Unit = Resolution;
}

impl Minable for Resolution {
    type Unit = Resolution;
}

impl std::fmt::Display for Resolution {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Resolution::Dpi(v) => write!(f, "{}dpi", v),
            Resolution::Dpcm(v) => write!(f, "{}dpcm", v),
            Resolution::Dppx(v) => write!(f, "{}dppx", v),
        }
    }
}

impl<Rhs> crate::math::ops::Max<Rhs> for Resolution {
    type Output = Max<Self, Rhs>;
    fn max(self, rhs: Rhs) -> Max<Self, Rhs> {
        Max::new(self, rhs)
    }
}

impl<Rhs> crate::math::ops::Min<Rhs> for Resolution {
    type Output = Min<Self, Rhs>;
    fn min(self, rhs: Rhs) -> Min<Self, Rhs> {
        Min::new(self, rhs)
    }
}

impl<Rhs> std::ops::Add<Rhs> for Resolution {
    type Output = Calculation<Self, Rhs>;
    fn add(self, rhs: Rhs) -> Calculation<Self, Rhs> {
        Calculation::new(self, rhs, Operation::Add)
    }
}

impl<Rhs> std::ops::Sub<Rhs> for Resolution {
    type Output = Calculation<Self, Rhs>;
    fn sub(self, rhs: Rhs) -> Calculation<Self, Rhs> {
        Calculation::new(self, rhs, Operation::Sub)
    }
}

impl<Rhs> std::ops::Mul<Rhs> for Resolution {
    type Output = Calculation<Self, Rhs>;
    fn mul(self, rhs: Rhs) -> Calculation<Self, Rhs> {
        Calculation::new(self, rhs, Operation::Mul)
    }
}

impl<Rhs> std::ops::Div<Rhs> for Resolution {
    type Output = Calculation<Self, Rhs>;
    fn div(self, rhs: Rhs) -> Calculation<Self, Rhs> {
        Calculation::new(self, rhs, Operation::Div)
    }
}

#[cfg(test)]
mod tests {
    use crate::{dimensions::*, math::ops::Max, math::ops::Min};

    #[test]
    fn test_calc() {
        let r1 = Resolution::Dpi(100.);
        let r2 = Resolution::Dpcm(50.);
        assert_eq!(format!("{}", r1 + r2), "calc(100dpi + 50dpcm)");
        assert_eq!(format!("{}", r1 - r2), "calc(100dpi - 50dpcm)");
        assert_eq!(format!("{}", r1 * r2), "calc(100dpi * 50dpcm)");
        assert_eq!(format!("{}", r1 / r2), "calc(100dpi / 50dpcm)");
    }

    #[test]
    fn test_max() {
        let r1 = Resolution::Dpi(100.);
        let r2 = Resolution::Dpcm(50.);
        assert_eq!(format!("{}", r1.max(r2)), "max(100dpi, 50dpcm)");
    }

    #[test]
    fn test_min() {
        let r1 = Resolution::Dpi(100.);
        let r2 = Resolution::Dpcm(50.);
        assert_eq!(format!("{}", r1.min(r2)), "min(100dpi, 50dpcm)");
    }
}
