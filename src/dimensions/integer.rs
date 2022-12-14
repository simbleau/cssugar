use crate::math::{Calculable, Scalable};

impl<Unit> Scalable<Unit> for isize {}
impl<Unit> Scalable<Unit> for i8 {}
impl<Unit> Scalable<Unit> for i16 {}
impl<Unit> Scalable<Unit> for i32 {}
impl<Unit> Scalable<Unit> for i128 {}
impl<Unit> Scalable<Unit> for usize {}
impl<Unit> Scalable<Unit> for u8 {}
impl<Unit> Scalable<Unit> for u16 {}
impl<Unit> Scalable<Unit> for u32 {}
impl<Unit> Scalable<Unit> for u128 {}

impl<Unit> Calculable<Unit> for isize {}
impl<Unit> Calculable<Unit> for i8 {}
impl<Unit> Calculable<Unit> for i16 {}
impl<Unit> Calculable<Unit> for i32 {}
impl<Unit> Calculable<Unit> for i128 {}
impl<Unit> Calculable<Unit> for usize {}
impl<Unit> Calculable<Unit> for u8 {}
impl<Unit> Calculable<Unit> for u16 {}
impl<Unit> Calculable<Unit> for u32 {}
impl<Unit> Calculable<Unit> for u128 {}

#[cfg(test)]
mod tests {
    use crate::prelude::*;

    #[test]
    fn test_mul() {
        let l1 = Length::Vw(100.);
        let a1 = Angle::Deg(180.);
        let r1 = Resolution::Dpi(1024.);
        let t1 = Time::Seconds(1.);

        let i1 = 32;
        assert_eq!(format!("{}", l1 * i1), "calc(100vw * 32)");
        assert_eq!(format!("{}", a1 * i1), "calc(180deg * 32)");
        assert_eq!(format!("{}", r1 * i1), "calc(1024dpi * 32)");
        assert_eq!(format!("{}", t1 * i1), "calc(1s * 32)");
    }

    #[test]
    fn test_div() {
        let l1 = Length::Vw(100.);
        let a1 = Angle::Deg(180.);
        let r1 = Resolution::Dpi(1024.);
        let t1 = Time::Seconds(1.);

        let i1 = 32;
        assert_eq!(format!("{}", l1 / i1), "calc(100vw / 32)");
        assert_eq!(format!("{}", a1 / i1), "calc(180deg / 32)");
        assert_eq!(format!("{}", r1 / i1), "calc(1024dpi / 32)");
        assert_eq!(format!("{}", t1 / i1), "calc(1s / 32)");
    }

    #[test]
    fn test_calc() {
        let lc1 = Length::Vw(100.) + Length::In(2.5);
        let ac1 = Angle::Percent(50.) + Angle::Deg(180.);
        let rc1 = Resolution::Dpi(50.) + Resolution::Dppx(100.);
        let tc1 = Time::Seconds(5.) + Time::Milliseconds(500.);

        let i1 = 10;
        assert_eq!(format!("{}", lc1 * i1), "calc(calc(100vw + 2.5in) * 10)");
        assert_eq!(format!("{}", ac1 * i1), "calc(calc(50% + 180deg) * 10)");
        assert_eq!(format!("{}", rc1 * i1), "calc(calc(50dpi + 100dppx) * 10)");
        assert_eq!(format!("{}", tc1 * i1), "calc(calc(5s + 500ms) * 10)");
    }
}
