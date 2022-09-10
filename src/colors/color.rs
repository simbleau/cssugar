#[derive(Debug, PartialEq, Clone, Copy)]
pub struct Color {
    pub r: u8,
    pub g: u8,
    pub b: u8,
    pub a: f32,
}

impl Color {
    pub const fn new(r: u8, g: u8, b: u8, a: f32) -> Color {
        Color { r, g, b, a }
    }

    pub const fn opaque(r: u8, g: u8, b: u8) -> Color {
        Color { r, g, b, a: 1.0 }
    }

    /// Lighten the color by a percentage. 1.0 will always result in white, 0.0
    /// will result in no color change.
    pub fn lighten(&self, a: f32) -> Color {
        let adjustment = (255.0 * a) as u8;
        Color {
            r: { (self.r + adjustment).clamp(0, 255) },
            g: { (self.g + adjustment).clamp(0, 255) },
            b: { (self.b + adjustment).clamp(0, 255) },
            a: self.a,
        }
    }

    /// Darken the color by a percentage. 1.0 will always result in black, 0.0
    /// will result in no color change.
    pub fn darken(&self, a: f32) -> Color {
        self.lighten(-a)
    }

    pub const fn with_alpha(&self, a: f32) -> Color {
        Color {
            r: self.r,
            g: self.g,
            b: self.b,
            a,
        }
    }
}

impl std::fmt::Display for Color {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "rgba({}, {}, {}, {})", self.r, self.g, self.b, self.a)
    }
}

#[cfg(test)]
mod tests {
    use crate::colors::*;

    #[test]
    fn test_opaque() {
        let color = Color::opaque(1, 2, 3);
        assert_eq!(color.r, 1);
        assert_eq!(color.g, 2);
        assert_eq!(color.b, 3);
    }

    #[test]
    fn test_new() {
        let color = Color::new(1, 2, 3, 0.5);
        assert_eq!(color.r, 1);
        assert_eq!(color.g, 2);
        assert_eq!(color.b, 3);
        assert_eq!(color.a, 0.5);
    }

    #[test]
    fn test_display() {
        let color = Color::new(100, 200, 255, 0.5);
        assert_eq!(format!("{}", color), format!("rgba(100, 200, 255, 0.5)"));
    }
}
