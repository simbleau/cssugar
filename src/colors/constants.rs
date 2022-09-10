use crate::colors::Color;

pub const RED: Color = Color::opaque(255, 0, 0);
pub const GREEN: Color = Color::opaque(0, 255, 0);
pub const BLUE: Color = Color::opaque(0, 0, 255);
pub const WHITE: Color = Color::opaque(255, 255, 255);
pub const BLACK: Color = Color::opaque(0, 0, 0);
pub const TRANSPARENT: Color = Color::new(0, 0, 0, 0.0);

#[cfg(test)]
mod tests {
    use crate::colors::*;
    #[test]
    fn test_constants() {
        assert_eq!(RED, Color::opaque(255, 0, 0));
        assert_eq!(GREEN, Color::opaque(0, 255, 0));
        assert_eq!(BLUE, Color::opaque(0, 0, 255));
        assert_eq!(WHITE, Color::opaque(255, 255, 255));
        assert_eq!(BLACK, Color::opaque(0, 0, 0));
        assert_eq!(TRANSPARENT.a, 0.0);
    }
}
