use crate::colors::Color;

pub const WHITE: Color = Color::from_rgb(255, 255, 255);
pub const BLACK: Color = Color::from_rgb(0, 0, 0);
pub const TRANSPARENT: Color = Color::from_rgba(0, 0, 0, 0.0);

pub const RED: Color = Color::from_rgb(255, 0, 0);
pub const GREEN: Color = Color::from_rgb(0, 255, 0);
pub const BLUE: Color = Color::from_rgb(0, 0, 255);

pub const YELLOW: Color = Color::from_rgb(255, 255, 0);
pub const CYAN: Color = Color::from_rgb(0, 255, 255);
pub const MAGENTA: Color = Color::from_rgb(255, 0, 255);

#[cfg(test)]
mod tests {
    use crate::colors::*;
    #[test]
    fn test_constants() {
        assert_eq!(RED.r, 255);
        assert_eq!(RED.g, 0);
        assert_eq!(RED.b, 0);
        assert_eq!(RED.a, 1.0);

        assert_eq!(GREEN.r, 0);
        assert_eq!(GREEN.g, 255);
        assert_eq!(GREEN.b, 0);
        assert_eq!(GREEN.a, 1.0);

        assert_eq!(BLUE.r, 0);
        assert_eq!(BLUE.g, 0);
        assert_eq!(BLUE.b, 255);
        assert_eq!(BLUE.a, 1.0);

        assert_eq!(WHITE.r, 255);
        assert_eq!(WHITE.g, 255);
        assert_eq!(WHITE.b, 255);
        assert_eq!(WHITE.a, 1.0);

        assert_eq!(BLACK.r, 0);
        assert_eq!(BLACK.g, 0);
        assert_eq!(BLACK.b, 0);
        assert_eq!(BLACK.a, 1.0);

        assert_eq!(TRANSPARENT.a, 0.0);
    }
}
