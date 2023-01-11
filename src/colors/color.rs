#[derive(Debug, PartialEq, Clone, Copy)]
pub struct Color {
    pub r: u8,
    pub g: u8,
    pub b: u8,
    pub a: f32,
}

impl Color {
    pub const fn from_rgb(r: u8, g: u8, b: u8) -> Self {
        Self::from_rgba(r, g, b, 1.0)
    }

    pub const fn from_rgba(r: u8, g: u8, b: u8, a: f32) -> Self {
        Self { r, g, b, a }
    }

    pub const fn from_hex24(h: u32) -> Self {
        let r = (h >> 16 & 0xff) as u8;
        let g = (h >> 8 & 0xff) as u8;
        let b = (h & 0xff) as u8;
        Self::from_rgba(r, g, b, 1.0)
    }

    pub fn from_hex32(h: u32) -> Self {
        Self::from_hex24(h >> 8).set_alpha((h & 0xff) as f32 / 255.0)
    }

    pub fn from_hsl(h: i32, s: f32, l: f32) -> Self {
        Self::from_hsla(h, s, l, 1.0)
    }

    pub fn from_hsla(h: i32, s: f32, l: f32, a: f32) -> Self {
        let c = (1.0 - (2.0 * l - 1.0).abs()) * s;
        let x = c * (1 - ((h / 60) % 2 - 1).abs()) as f32;
        let m = l - c / 2.0;
        let (r_prime, g_prime, b_prime) = if h < 60 {
            (c, x, 0.0)
        } else if h < 120 {
            (x, c, 0.0)
        } else if h < 180 {
            (0.0, c, x)
        } else if h < 240 {
            (0.0, x, c)
        } else if h < 300 {
            (x, 0.0, c)
        } else {
            (c, 0.0, x)
        };
        let (r, g, b) = (
            (r_prime + m) * 255.0,
            (g_prime + m) * 255.0,
            (b_prime + m) * 255.0,
        );
        Self {
            r: r.max(0.0) as u8,
            g: g.max(0.0) as u8,
            b: b as u8,
            a,
        }
    }

    /// Lighten the color by a percentage. 1.0 will always result in white, 0.0
    /// will result in no color change.
    pub fn lighten(&self, a: f32) -> Self {
        Self {
            r: { (self.r as f32 + 255. * a).clamp(0., 255.) as u8 },
            g: { (self.g as f32 + 255. * a).clamp(0., 255.) as u8 },
            b: { (self.b as f32 + 255. * a).clamp(0., 255.) as u8 },
            a: self.a,
        }
    }

    pub fn set_lighter(&mut self, a: f32) -> Self {
        self.r = (self.r as f32 + 255. * a).clamp(0., 255.) as u8;
        self.g = (self.g as f32 + 255. * a).clamp(0., 255.) as u8;
        self.b = (self.b as f32 + 255. * a).clamp(0., 255.) as u8;
        self.a = a;
        *self
    }

    /// Darken the color by a percentage. 1.0 will always result in black, 0.0
    /// will result in no color change.
    pub fn darken(&self, a: f32) -> Self {
        self.lighten(-a)
    }

    pub fn set_darker(&mut self, a: f32) -> Self {
        self.set_lighter(-a)
    }

    pub fn set_alpha(&mut self, a: f32) -> Self {
        self.a = a;
        *self
    }

    pub fn alpha(&self, a: f32) -> Self {
        Self {
            r: self.r,
            g: self.g,
            b: self.b,
            a,
        }
    }

    pub fn set_red(&mut self, r: u8) -> Self {
        self.r = r;
        *self
    }

    pub fn red(&self, r: u8) -> Self {
        Self {
            r,
            g: self.g,
            b: self.b,
            a: self.a,
        }
    }

    pub fn set_green(&mut self, g: u8) -> Self {
        self.g = g;
        *self
    }

    pub fn green(&self, g: u8) -> Self {
        Self {
            r: self.r,
            g,
            b: self.b,
            a: self.a,
        }
    }

    pub fn set_blue(&mut self, b: u8) -> Self {
        self.b = b;
        *self
    }

    pub fn blue(&self, b: u8) -> Self {
        Self {
            r: self.r,
            g: self.g,
            b,
            a: self.a,
        }
    }

    pub fn set_hue(&mut self, _h: f32) -> Self {
        todo!()
    }

    pub fn hue(&self, _h: f32) -> Self {
        todo!()
    }

    pub fn set_saturation(&mut self, _s: f32) -> Self {
        todo!()
    }

    pub fn saturation(&self, _s: f32) -> Self {
        todo!()
    }

    pub fn set_lightness(&mut self, _l: f32) -> Self {
        todo!()
    }

    pub fn lightness(&self, _l: f32) -> Self {
        todo!()
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
    fn test_display() {
        let color = Color::from_rgba(100, 200, 255, 0.5);
        assert_eq!(format!("{}", color), "rgba(100, 200, 255, 0.5)");
    }

    #[test]
    fn test_rgb() {
        assert_eq!(Color::from_rgb(0, 0, 0,), BLACK);
        assert_eq!(Color::from_rgb(255, 255, 255), WHITE);

        assert_eq!(Color::from_rgb(255, 0, 0), RED);
        assert_eq!(Color::from_rgb(0, 255, 0), GREEN);
        assert_eq!(Color::from_rgb(0, 0, 255), BLUE);

        assert_eq!(Color::from_rgb(255, 255, 0), YELLOW);
        assert_eq!(Color::from_rgb(0, 255, 255), CYAN);
        assert_eq!(Color::from_rgb(255, 0, 255), MAGENTA);
    }

    #[test]
    fn test_hex24() {
        assert_eq!(Color::from_hex24(0x000000), BLACK);
        assert_eq!(Color::from_hex24(0xffffff), WHITE);

        assert_eq!(Color::from_hex24(0xff0000), RED);
        assert_eq!(Color::from_hex24(0x00ff00), GREEN);
        assert_eq!(Color::from_hex24(0x0000ff), BLUE);

        assert_eq!(Color::from_hex24(0xffff00), YELLOW);
        assert_eq!(Color::from_hex24(0x00ffff), CYAN);
        assert_eq!(Color::from_hex24(0xff00ff), MAGENTA);
    }

    #[test]
    fn test_hex32() {
        assert_eq!(Color::from_hex32(0x000000ff), BLACK);
        assert_eq!(Color::from_hex32(0xffffffff), WHITE);
        assert_eq!(Color::from_hex32(0x00000000), TRANSPARENT);

        assert_eq!(Color::from_hex32(0xff0000ff), RED);
        assert_eq!(Color::from_hex32(0x00ff00ff), GREEN);
        assert_eq!(Color::from_hex32(0x0000ffff), BLUE);

        assert_eq!(Color::from_hex32(0xffff00ff), YELLOW);
        assert_eq!(Color::from_hex32(0x00ffffff), CYAN);
        assert_eq!(Color::from_hex32(0xff00ffff), MAGENTA);
    }

    #[test]
    fn test_hsl() {
        assert_eq!(Color::from_hsl(0, 0.0, 0.0), BLACK);
        assert_eq!(Color::from_hsl(0, 0.0, 1.0), WHITE);

        assert_eq!(Color::from_hsl(0, 1.0, 0.5), RED);
        assert_eq!(Color::from_hsl(120, 1.0, 0.5), GREEN);
        assert_eq!(Color::from_hsl(240, 1.0, 0.5), BLUE);

        assert_eq!(Color::from_hsl(60, 1.0, 0.5), YELLOW);
        assert_eq!(Color::from_hsl(180, 1.0, 0.5), CYAN);
        assert_eq!(Color::from_hsl(300, 1.0, 0.5), MAGENTA);
    }
}
