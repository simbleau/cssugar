use crate::math::markers::Scalable;

impl<Unit> Scalable<Unit> for f64 {}
impl<Unit> Scalable<Unit> for f32 {}

#[cfg(test)]
mod tests {
    use crate::dimensions::*;

    #[test]
    fn test_composition() {
        let f1 = 32.5;

        let lc1 = Length::Vw(100.) + Length::In(2.5);
        assert_eq!(format!("{}", lc1 * f1), "calc(calc(100vw + 2.5in) * 32.5)");

        let ac1 = Angle::Percent(50.) + Angle::Deg(180.);
        assert_eq!(format!("{}", ac1 * f1), "calc(calc(50% + 180deg) * 32.5)");

        let rc1 = Resolution::Dpi(50.) + Resolution::Dppx(100.);
        assert_eq!(
            format!("{}", rc1 * f1),
            "calc(calc(50dpi + 100dppx) * 32.5)"
        );

        let tc1 = Time::Seconds(5.) + Time::Milliseconds(500.);
        assert_eq!(format!("{}", tc1 * f1), "calc(calc(5s + 500ms) * 32.5)");
    }

    #[test]
    fn test_number() {
        let l1 = Length::Vw(100.);
        let a1 = Angle::Deg(180.);
        let r1 = Resolution::Dpi(1024.);
        let t1 = Time::Seconds(1.);

        let f1 = 32.5;
        assert_eq!(format!("{}", l1 * f1), "calc(100vw * 32.5)");
        assert_eq!(format!("{}", l1 / f1), "calc(100vw / 32.5)");

        assert_eq!(format!("{}", a1 * f1), "calc(180deg * 32.5)");
        assert_eq!(format!("{}", a1 / f1), "calc(180deg / 32.5)");

        assert_eq!(format!("{}", r1 * f1), "calc(1024dpi * 32.5)");
        assert_eq!(format!("{}", r1 / f1), "calc(1024dpi / 32.5)");

        assert_eq!(format!("{}", t1 * f1), "calc(1s * 32.5)");
        assert_eq!(format!("{}", t1 / f1), "calc(1s / 32.5)");
    }
}
