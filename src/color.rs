pub struct Color {
    pub r: f32,
    pub g: f32,
    pub b: f32,
    pub a: f32,
}

// TODO: Replace this with a const fn when it's stable
macro_rules! define_color {
    ($hex:expr) => {
        Color {
            r: (($hex >> 16) & 0xFF) as f32 / 255.0,
            g: (($hex >> 8) & 0xFF) as f32 / 255.0,
            b: (($hex >> 0) & 0xFF) as f32 / 255.0,
            a: 1.0,
        }
    };
}
impl Color {
    pub const AliceBlue: Color = define_color!(0xF0F8FF);
    pub const AntiqueWhite: Color = define_color!(0xFAEBD7);
    pub const Aqua: Color = define_color!(0x00FFFF);
    pub const Aquamarine: Color = define_color!(0x7FFFD4);
    pub const Azure: Color = define_color!(0xF0FFFF);
    pub const Beige: Color = define_color!(0xF5F5DC);
    pub const Bisque: Color = define_color!(0xFFE4C4);
    pub const Black: Color = define_color!(0x000000);
    pub const BlanchedAlmond: Color = define_color!(0xFFEBCD);
    pub const Blue: Color = define_color!(0x0000FF);
    pub const BlueViolet: Color = define_color!(0x8A2BE2);
    pub const Brown: Color = define_color!(0xA52A2A);
    pub const BurlyWood: Color = define_color!(0xDEB887);
    pub const CadetBlue: Color = define_color!(0x5F9EA0);
    pub const Chartreuse: Color = define_color!(0x7FFF00);
    pub const Chocolate: Color = define_color!(0xD2691E);
    pub const Coral: Color = define_color!(0xFF7F50);
    pub const CornflowerBlue: Color = define_color!(0x6495ED);
    pub const Cornsilk: Color = define_color!(0xFFF8DC);
    pub const Crimson: Color = define_color!(0xDC143C);
    pub const Cyan: Color = define_color!(0x00FFFF);
    pub const DarkBlue: Color = define_color!(0x00008B);
    pub const DarkCyan: Color = define_color!(0x008B8B);
    pub const DarkGoldenrod: Color = define_color!(0xB8860B);
    pub const DarkGray: Color = define_color!(0xA9A9A9);
    pub const DarkGreen: Color = define_color!(0x006400);
    pub const DarkKhaki: Color = define_color!(0xBDB76B);
    pub const DarkMagenta: Color = define_color!(0x8B008B);
    pub const DarkOliveGreen: Color = define_color!(0x556B2F);
    pub const DarkOrange: Color = define_color!(0xFF8C00);
    pub const DarkOrchid: Color = define_color!(0x9932CC);
    pub const DarkRed: Color = define_color!(0x8B0000);
    pub const DarkSalmon: Color = define_color!(0xE9967A);
    pub const DarkSeaGreen: Color = define_color!(0x8FBC8F);
    pub const DarkSlateBlue: Color = define_color!(0x483D8B);
    pub const DarkSlateGray: Color = define_color!(0x2F4F4F);
    pub const DarkTurquoise: Color = define_color!(0x00CED1);
    pub const DarkViolet: Color = define_color!(0x9400D3);
    pub const DeepPink: Color = define_color!(0xFF1493);
    pub const DeepSkyBlue: Color = define_color!(0x00BFFF);
    pub const DimGray: Color = define_color!(0x696969);
    pub const DodgerBlue: Color = define_color!(0x1E90FF);
    pub const Firebrick: Color = define_color!(0xB22222);
    pub const FloralWhite: Color = define_color!(0xFFFAF0);
    pub const ForestGreen: Color = define_color!(0x228B22);
    pub const Fuchsia: Color = define_color!(0xFF00FF);
    pub const Gainsboro: Color = define_color!(0xDCDCDC);
    pub const GhostWhite: Color = define_color!(0xF8F8FF);
    pub const Gold: Color = define_color!(0xFFD700);
    pub const Goldenrod: Color = define_color!(0xDAA520);
    pub const Gray: Color = define_color!(0x808080);
    pub const Green: Color = define_color!(0x008000);
    pub const GreenYellow: Color = define_color!(0xADFF2F);
    pub const Honeydew: Color = define_color!(0xF0FFF0);
    pub const HotPink: Color = define_color!(0xFF69B4);
    pub const IndianRed: Color = define_color!(0xCD5C5C);
    pub const Indigo: Color = define_color!(0x4B0082);
    pub const Ivory: Color = define_color!(0xFFFFF0);
    pub const Khaki: Color = define_color!(0xF0E68C);
    pub const Lavender: Color = define_color!(0xE6E6FA);
    pub const LavenderBlush: Color = define_color!(0xFFF0F5);
    pub const LawnGreen: Color = define_color!(0x7CFC00);
    pub const LemonChiffon: Color = define_color!(0xFFFACD);
    pub const LightBlue: Color = define_color!(0xADD8E6);
    pub const LightCoral: Color = define_color!(0xF08080);
    pub const LightCyan: Color = define_color!(0xE0FFFF);
    pub const LightGoldenrodYellow: Color = define_color!(0xFAFAD2);
    pub const LightGreen: Color = define_color!(0x90EE90);
    pub const LightGray: Color = define_color!(0xD3D3D3);
    pub const LightPink: Color = define_color!(0xFFB6C1);
    pub const LightSalmon: Color = define_color!(0xFFA07A);
    pub const LightSeaGreen: Color = define_color!(0x20B2AA);
    pub const LightSkyBlue: Color = define_color!(0x87CEFA);
    pub const LightSlateGray: Color = define_color!(0x778899);
    pub const LightSteelBlue: Color = define_color!(0xB0C4DE);
    pub const LightYellow: Color = define_color!(0xFFFFE0);
    pub const Lime: Color = define_color!(0x00FF00);
    pub const LimeGreen: Color = define_color!(0x32CD32);
    pub const Linen: Color = define_color!(0xFAF0E6);
    pub const Magenta: Color = define_color!(0xFF00FF);
    pub const Maroon: Color = define_color!(0x800000);
    pub const MediumAquamarine: Color = define_color!(0x66CDAA);
    pub const MediumBlue: Color = define_color!(0x0000CD);
    pub const MediumOrchid: Color = define_color!(0xBA55D3);
    pub const MediumPurple: Color = define_color!(0x9370DB);
    pub const MediumSeaGreen: Color = define_color!(0x3CB371);
    pub const MediumSlateBlue: Color = define_color!(0x7B68EE);
    pub const MediumSpringGreen: Color = define_color!(0x00FA9A);
    pub const MediumTurquoise: Color = define_color!(0x48D1CC);
    pub const MediumVioletRed: Color = define_color!(0xC71585);
    pub const MidnightBlue: Color = define_color!(0x191970);
    pub const MintCream: Color = define_color!(0xF5FFFA);
    pub const MistyRose: Color = define_color!(0xFFE4E1);
    pub const Moccasin: Color = define_color!(0xFFE4B5);
    pub const NavajoWhite: Color = define_color!(0xFFDEAD);
    pub const Navy: Color = define_color!(0x000080);
    pub const OldLace: Color = define_color!(0xFDF5E6);
    pub const Olive: Color = define_color!(0x808000);
    pub const OliveDrab: Color = define_color!(0x6B8E23);
    pub const Orange: Color = define_color!(0xFFA500);
    pub const OrangeRed: Color = define_color!(0xFF4500);
    pub const Orchid: Color = define_color!(0xDA70D6);
    pub const PaleGoldenrod: Color = define_color!(0xEEE8AA);
    pub const PaleGreen: Color = define_color!(0x98FB98);
    pub const PaleTurquoise: Color = define_color!(0xAFEEEE);
    pub const PaleVioletRed: Color = define_color!(0xDB7093);
    pub const PapayaWhip: Color = define_color!(0xFFEFD5);
    pub const PeachPuff: Color = define_color!(0xFFDAB9);
    pub const Peru: Color = define_color!(0xCD853F);
    pub const Pink: Color = define_color!(0xFFC0CB);
    pub const Plum: Color = define_color!(0xDDA0DD);
    pub const PowderBlue: Color = define_color!(0xB0E0E6);
    pub const Purple: Color = define_color!(0x800080);
    pub const Red: Color = define_color!(0xFF0000);
    pub const RosyBrown: Color = define_color!(0xBC8F8F);
    pub const RoyalBlue: Color = define_color!(0x4169E1);
    pub const SaddleBrown: Color = define_color!(0x8B4513);
    pub const Salmon: Color = define_color!(0xFA8072);
    pub const SandyBrown: Color = define_color!(0xF4A460);
    pub const SeaGreen: Color = define_color!(0x2E8B57);
    pub const SeaShell: Color = define_color!(0xFFF5EE);
    pub const Sienna: Color = define_color!(0xA0522D);
    pub const Silver: Color = define_color!(0xC0C0C0);
    pub const SkyBlue: Color = define_color!(0x87CEEB);
    pub const SlateBlue: Color = define_color!(0x6A5ACD);
    pub const SlateGray: Color = define_color!(0x708090);
    pub const Snow: Color = define_color!(0xFFFAFA);
    pub const SpringGreen: Color = define_color!(0x00FF7F);
    pub const SteelBlue: Color = define_color!(0x4682B4);
    pub const Tan: Color = define_color!(0xD2B48C);
    pub const Teal: Color = define_color!(0x008080);
    pub const Thistle: Color = define_color!(0xD8BFD8);
    pub const Tomato: Color = define_color!(0xFF6347);
    pub const Turquoise: Color = define_color!(0x40E0D0);
    pub const Violet: Color = define_color!(0xEE82EE);
    pub const Wheat: Color = define_color!(0xF5DEB3);
    pub const White: Color = define_color!(0xFFFFFF);
    pub const WhiteSmoke: Color = define_color!(0xF5F5F5);
    pub const Yellow: Color = define_color!(0xFFFF00);
    pub const YellowGreen: Color = define_color!(0x9ACD32);
}
