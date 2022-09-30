use crate::math::markers::Scalable;

impl<T> Scalable<T> for i8 {}
impl<T> Scalable<T> for i16 {}
impl<T> Scalable<T> for i32 {}
impl<T> Scalable<T> for i128 {}

#[cfg(test)]
mod tests {
    use crate::dimensions::*;

    #[test]
    fn test_composition() {
        let c1 = Length::Vw(100.) + Length::In(2.5);
        let i1 = 32;
        assert_eq!(format!("{}", c1 * i1), "calc(calc(100vw + 2.5in) * 32)");
    }

    #[test]
    fn test_integer() {
        let l1 = Length::Vw(100.);
        let a1 = Angle::Deg(180.);
        let r1 = Resolution::Dpi(1024.);
        let t1 = Time::Seconds(1.);

        let i1 = 32;
        assert_eq!(format!("{}", l1 * i1), "calc(100vw * 32)");
        assert_eq!(format!("{}", l1 / i1), "calc(100vw / 32)");

        assert_eq!(format!("{}", a1 * i1), "calc(180deg * 32)");
        assert_eq!(format!("{}", a1 / i1), "calc(180deg / 32)");

        assert_eq!(format!("{}", r1 * i1), "calc(1024dpi * 32)");
        assert_eq!(format!("{}", r1 / i1), "calc(1024dpi / 32)");

        assert_eq!(format!("{}", t1 * i1), "calc(1s * 32)");
        assert_eq!(format!("{}", t1 / i1), "calc(1s / 32)");
    }
}
