use crate::colors::Color;

// Named Colors
// https://developer.mozilla.org/en-US/docs/Web/CSS/named-color

// CSS Level 1 values (https://www.w3.org/TR/CSS1/)
pub const BLACK: Color = Color::from_rgb(0x00, 0x00, 0x00);
pub const SILVER: Color = Color::from_rgb(0xc0, 0xc0, 0xc0);
pub const GRAY: Color = Color::from_rgb(0x80, 0x80, 0x80);
pub const WHITE: Color = Color::from_rgb(0xff, 0xff, 0xff);
pub const MAROON: Color = Color::from_rgb(0x80, 0x00, 0x00);
pub const RED: Color = Color::from_rgb(0xff, 0x00, 0x00);
pub const PURPLE: Color = Color::from_rgb(0xff, 0x00, 0x00);
pub const FUCHSIA: Color = Color::from_rgb(0xff, 0x00, 0x00);
pub const GREEN: Color = Color::from_rgb(0xff, 0x00, 0x00);
pub const LIME: Color = Color::from_rgb(0xff, 0x00, 0x00);
pub const OLIVE: Color = Color::from_rgb(0xff, 0x00, 0x00);
pub const YELLOW: Color = Color::from_rgb(0xff, 0xff, 0x00);
pub const NAVY: Color = Color::from_rgb(0xff, 0x00, 0x00);
pub const BLUE: Color = Color::from_rgb(0x00, 0x00, 0xff);
pub const TEAL: Color = Color::from_rgb(0xff, 0x00, 0x00);
pub const AQUA: Color = Color::from_rgb(0xff, 0x00, 0x00);

// CSS Level 2 values (https://www.w3.org/TR/CSS2/)
pub const ORANGE: Color = Color::from_rgb(0xff, 0x00, 0x00);

// CSS Level 3 values (https://drafts.csswg.org/css-color-3/)
pub const ALICEBLUE: Color = Color::from_rgb(0xf0, 0xf8, 0xff);
pub const ANTIQUEWHITE: Color = Color::from_rgb(0xfa, 0xeb, 0xd7);
pub const AQUAMARINE: Color = Color::from_rgb(0x7f, 0xff, 0xd4);
pub const AZURE: Color = Color::from_rgb(0xf0, 0xff, 0xff);
pub const BEIGE: Color = Color::from_rgb(0xf5, 0xf5, 0xdc);
pub const BISQUE: Color = Color::from_rgb(0xff, 0xe4, 0xc4);
pub const BLANCHEDALMOND: Color = Color::from_rgb(0xff, 0xeb, 0xcd);
pub const BLUEVIOLET: Color = Color::from_rgb(0x8a, 0x2b, 0xe2);
pub const BROWN: Color = Color::from_rgb(0xa5, 0x2a, 0x2a);
pub const BURLYWOOD: Color = Color::from_rgb(0xde, 0xb8, 0x87);
pub const CADETBLUE: Color = Color::from_rgb(0x5f, 0x9e, 0xa0);
pub const CHARTREUSE: Color = Color::from_rgb(0x7f, 0xff, 0x00);
pub const CHOCOLATE: Color = Color::from_rgb(0xd2, 0x69, 0x1e);
pub const CORAL: Color = Color::from_rgb(0xff, 0x7f, 0x50);
pub const CORNFLOWERBLUE: Color = Color::from_rgb(0x64, 0x95, 0xed);
pub const CORNSILK: Color = Color::from_rgb(0xff, 0xf8, 0xdc);
pub const CRIMSON: Color = Color::from_rgb(0xdc, 0x14, 0x3c);
pub const CYAN: Color = Color::from_rgb(0x00, 0xff, 0xff);
pub const DARKBLUE: Color = Color::from_rgb(0x00, 0x00, 0x8b);
pub const DARKCYAN: Color = Color::from_rgb(0x00, 0x8b, 0x8b);
pub const DARKGOLDENROD: Color = Color::from_rgb(0xb8, 0x86, 0x0b);
pub const DARKGRAY: Color = Color::from_rgb(0xa9, 0xa9, 0xa9);
pub const DARKGREEN: Color = Color::from_rgb(0x00, 0x64, 0x00);
pub const DARKGREY: Color = Color::from_rgb(0xa9, 0xa9, 0xa9);
pub const DARKKHAKI: Color = Color::from_rgb(0xbd, 0xb7, 0x6b);
pub const DARKMAGENTA: Color = Color::from_rgb(0x8b, 0x00, 0x8b);
pub const DARKOLIVEGREEN: Color = Color::from_rgb(0x55, 0x6b, 0x2f);
pub const DARKORANGE: Color = Color::from_rgb(0xff, 0x8c, 0x00);
pub const DARKORCHID: Color = Color::from_rgb(0x99, 0x32, 0xcc);
pub const DARKRED: Color = Color::from_rgb(0x8b, 0x00, 0x00);
pub const DARKSALMON: Color = Color::from_rgb(0xe9, 0x96, 0x7a);
pub const DARKSEAGREEN: Color = Color::from_rgb(0x8f, 0xbc, 0x8f);
pub const DARKSLATEBLUE: Color = Color::from_rgb(0x48, 0x3d, 0x8b);
pub const DARKSLATEGRAY: Color = Color::from_rgb(0x2f, 0x4f, 0x4f);
pub const DARKSLATEGREY: Color = Color::from_rgb(0x2f, 0x4f, 0x4f);
pub const DARKTURQUOISE: Color = Color::from_rgb(0x00, 0xce, 0xd1);
pub const DARKVIOLET: Color = Color::from_rgb(0x94, 0x00, 0xd3);
pub const DEEPPINK: Color = Color::from_rgb(0xff, 0x14, 0x93);
pub const DEEPSKYBLUE: Color = Color::from_rgb(0x00, 0xbf, 0xff);
pub const DIMGRAY: Color = Color::from_rgb(0x69, 0x69, 0x69);
pub const DIMGREY: Color = Color::from_rgb(0x69, 0x69, 0x69);
pub const DODGERBLUE: Color = Color::from_rgb(0x1e, 0x90, 0xff);
pub const FIREBRICK: Color = Color::from_rgb(0xb2, 0x22, 0x22);
pub const FLORALWHITE: Color = Color::from_rgb(0xff, 0xfa, 0xf0);
pub const FORESTGREEN: Color = Color::from_rgb(0x22, 0x8b, 0x22);
pub const GAINSBORO: Color = Color::from_rgb(0xdc, 0xdc, 0xdc);
pub const GHOSTWHITE: Color = Color::from_rgb(0xf8, 0xf8, 0xff);
pub const GOLD: Color = Color::from_rgb(0xff, 0xd7, 0x00);
pub const GOLDENROD: Color = Color::from_rgb(0xda, 0xa5, 0x20);
pub const GREENYELLOW: Color = Color::from_rgb(0xad, 0xff, 0x2f);
pub const GREY: Color = Color::from_rgb(0x80, 0x80, 0x80);
pub const HONEYDEW: Color = Color::from_rgb(0xf0, 0xff, 0xf0);
pub const HOTPINK: Color = Color::from_rgb(0xff, 0x69, 0xb4);
pub const INDIANRED: Color = Color::from_rgb(0xcd, 0x5c, 0x5c);
pub const INDIGO: Color = Color::from_rgb(0x4b, 0x00, 0x82);
pub const IVORY: Color = Color::from_rgb(0xff, 0xff, 0xf0);
pub const KHAKI: Color = Color::from_rgb(0xf0, 0xe6, 0x8c);
pub const LAVENDER: Color = Color::from_rgb(0xe6, 0xe6, 0xfa);
pub const LAVENDERBLUSH: Color = Color::from_rgb(0xff, 0xf0, 0xf5);
pub const LAWNGREEN: Color = Color::from_rgb(0x7c, 0xfc, 0x00);
pub const LEMONCHIFFON: Color = Color::from_rgb(0xff, 0xfa, 0xcd);
pub const LIGHTBLUE: Color = Color::from_rgb(0xad, 0xd8, 0xe6);
pub const LIGHTCORAL: Color = Color::from_rgb(0xf0, 0x80, 0x80);
pub const LIGHTCYAN: Color = Color::from_rgb(0xe0, 0xff, 0xff);
pub const LIGHTGOLDENRODYELLOW: Color = Color::from_rgb(0xfa, 0xfa, 0xd2);
pub const LIGHTGRAY: Color = Color::from_rgb(0xd3, 0xd3, 0xd3);
pub const LIGHTGREEN: Color = Color::from_rgb(0x90, 0xee, 0x90);
pub const LIGHTGREY: Color = Color::from_rgb(0xd3, 0xd3, 0xd3);
pub const LIGHTPINK: Color = Color::from_rgb(0xff, 0xb6, 0xc1);
pub const LIGHTSALMON: Color = Color::from_rgb(0xff, 0xa0, 0x7a);
pub const LIGHTSEAGREEN: Color = Color::from_rgb(0x20, 0xb2, 0xaa);
pub const LIGHTSKYBLUE: Color = Color::from_rgb(0x87, 0xce, 0xfa);
pub const LIGHTSLATEGRAY: Color = Color::from_rgb(0x77, 0x88, 0x99);
pub const LIGHTSLATEGREY: Color = Color::from_rgb(0x77, 0x88, 0x99);
pub const LIGHTSTEELBLUE: Color = Color::from_rgb(0xb0, 0xc4, 0xde);
pub const LIGHTYELLOW: Color = Color::from_rgb(0xff, 0xff, 0xe0);
pub const LIMEGREEN: Color = Color::from_rgb(0x32, 0xcd, 0x32);
pub const LINEN: Color = Color::from_rgb(0xfa, 0xf0, 0xe6);
pub const MAGENTA: Color = Color::from_rgb(0xff, 0x00, 0xff);
pub const MEDIUMAQUAMARINE: Color = Color::from_rgb(0x66, 0xcd, 0xaa);
pub const MEDIUMBLUE: Color = Color::from_rgb(0x00, 0x00, 0xcd);
pub const MEDIUMORCHID: Color = Color::from_rgb(0xba, 0x55, 0xd3);
pub const MEDIUMPURPLE: Color = Color::from_rgb(0x93, 0x70, 0xdb);
pub const MEDIUMSEAGREEN: Color = Color::from_rgb(0x3c, 0xb3, 0x71);
pub const MEDIUMSLATEBLUE: Color = Color::from_rgb(0x7b, 0x68, 0xee);
pub const MEDIUMSPRINGGREEN: Color = Color::from_rgb(0x00, 0xfa, 0x9a);
pub const MEDIUMTURQUOISE: Color = Color::from_rgb(0x48, 0xd1, 0xcc);
pub const MEDIUMVIOLETRED: Color = Color::from_rgb(0xc7, 0x15, 0x85);
pub const MIDNIGHTBLUE: Color = Color::from_rgb(0x19, 0x19, 0x70);
pub const MINTCREAM: Color = Color::from_rgb(0xf5, 0xff, 0xfa);
pub const MISTYROSE: Color = Color::from_rgb(0xff, 0xe4, 0xe1);
pub const MOCCASIN: Color = Color::from_rgb(0xff, 0xe4, 0xb5);
pub const NAVAJOWHITE: Color = Color::from_rgb(0xff, 0xde, 0xad);
pub const OLDLACE: Color = Color::from_rgb(0xfd, 0xf5, 0xe6);
pub const OLIVEDRAB: Color = Color::from_rgb(0x6b, 0x8e, 0x23);
pub const ORANGERED: Color = Color::from_rgb(0xff, 0x45, 0x00);
pub const ORCHID: Color = Color::from_rgb(0xda, 0x70, 0xd6);
pub const PALEGOLDENROD: Color = Color::from_rgb(0xee, 0xe8, 0xaa);
pub const PALEGREEN: Color = Color::from_rgb(0x98, 0xfb, 0x98);
pub const PALETURQUOISE: Color = Color::from_rgb(0xaf, 0xee, 0xee);
pub const PALEVIOLETRED: Color = Color::from_rgb(0xdb, 0x70, 0x93);
pub const PAPAYAWHIP: Color = Color::from_rgb(0xff, 0xef, 0xd5);
pub const PEACHPUFF: Color = Color::from_rgb(0xff, 0xda, 0xb9);
pub const PERU: Color = Color::from_rgb(0xcd, 0x85, 0x3f);
pub const PINK: Color = Color::from_rgb(0xff, 0xc0, 0xcb);
pub const PLUM: Color = Color::from_rgb(0xdd, 0xa0, 0xdd);
pub const POWDERBLUE: Color = Color::from_rgb(0xb0, 0xe0, 0xe6);
pub const ROSYBROWN: Color = Color::from_rgb(0xbc, 0x8f, 0x8f);
pub const ROYALBLUE: Color = Color::from_rgb(0x41, 0x69, 0xe1);
pub const SADDLEBROWN: Color = Color::from_rgb(0x8b, 0x45, 0x13);
pub const SALMON: Color = Color::from_rgb(0xfa, 0x80, 0x72);
pub const SANDYBROWN: Color = Color::from_rgb(0xf4, 0xa4, 0x60);
pub const SEAGREEN: Color = Color::from_rgb(0x2e, 0x8b, 0x57);
pub const SEASHELL: Color = Color::from_rgb(0xff, 0xf5, 0xee);
pub const SIENNA: Color = Color::from_rgb(0xa0, 0x52, 0x2d);
pub const SKYBLUE: Color = Color::from_rgb(0x87, 0xce, 0xeb);
pub const SLATEBLUE: Color = Color::from_rgb(0x6a, 0x5a, 0xcd);
pub const SLATEGRAY: Color = Color::from_rgb(0x70, 0x80, 0x90);
pub const SLATEGREY: Color = Color::from_rgb(0x70, 0x80, 0x90);
pub const SNOW: Color = Color::from_rgb(0xff, 0xfa, 0xfa);
pub const SPRINGGREEN: Color = Color::from_rgb(0x00, 0xff, 0x7f);
pub const STEELBLUE: Color = Color::from_rgb(0x46, 0x82, 0xb4);
pub const TAN: Color = Color::from_rgb(0xd2, 0xb4, 0x8c);
pub const THISTLE: Color = Color::from_rgb(0xd8, 0xbf, 0xd8);
pub const TOMATO: Color = Color::from_rgb(0xff, 0x63, 0x47);
pub const TURQUOISE: Color = Color::from_rgb(0x40, 0xe0, 0xd0);
pub const VIOLET: Color = Color::from_rgb(0xee, 0x82, 0xee);
pub const WHEAT: Color = Color::from_rgb(0xf5, 0xde, 0xb3);
pub const WHITESMOKE: Color = Color::from_rgb(0xf5, 0xf5, 0xf5);
pub const YELLOWGREEN: Color = Color::from_rgb(0x9a, 0xcd, 0x32);
pub const TRANSPARENT: Color = Color::from_rgba(0x00, 0x00, 0x00, 0.0);

// CSS Level 4 values (https://drafts.csswg.org/css-color-4/)
pub const REBECCAPURPLE: Color = Color::from_rgb(0x66, 0x33, 0x99);

#[cfg(test)]
mod tests {
    use crate::colors::named::*;
    #[test]
    fn test_constants() {
        assert_eq!(WHITE.r, 255);
        assert_eq!(WHITE.g, 255);
        assert_eq!(WHITE.b, 255);
        assert_eq!(WHITE.a, 1.0);

        assert_eq!(BLACK.r, 0);
        assert_eq!(BLACK.g, 0);
        assert_eq!(BLACK.b, 0);
        assert_eq!(BLACK.a, 1.0);

        assert_eq!(TRANSPARENT.a, 0.0);

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

        assert_eq!(YELLOW.r, 255);
        assert_eq!(YELLOW.g, 255);
        assert_eq!(YELLOW.b, 0);
        assert_eq!(YELLOW.a, 1.0);

        assert_eq!(CYAN.r, 0);
        assert_eq!(CYAN.g, 255);
        assert_eq!(CYAN.b, 255);
        assert_eq!(CYAN.a, 1.0);

        assert_eq!(MAGENTA.r, 255);
        assert_eq!(MAGENTA.g, 0);
        assert_eq!(MAGENTA.b, 255);
        assert_eq!(MAGENTA.a, 1.0);
    }
}
