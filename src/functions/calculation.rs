pub(crate) mod markers {
    pub trait Calculable:
        std::fmt::Display
        + std::ops::Add
        + std::ops::Sub
        + std::ops::Mul
        + std::ops::Div
        + Sized
    {
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct Calculation<T: markers::Calculable> {
    pub lhs: T,
    pub rhs: T,
}

impl<T: markers::Calculable> std::fmt::Display for Calculation<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "calc({})", self.lhs)
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
}
