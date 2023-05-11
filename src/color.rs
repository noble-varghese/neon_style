use crossterm::style::Color;

#[derive(PartialEq, Clone, Copy)]
pub enum ColorValue {
    Color(Color),
    NoColor(),
}

#[derive(PartialEq, Clone, Copy)]
pub struct Colour {
    pub color: ColorValue,
}

fn to_hex_digit(c: char) -> u8 {
    c.to_digit(16).unwrap() as u8
}

impl Colour {
    fn from_hex(hex: &str) -> Option<Self> {
        let mut chars = hex
            .chars()
            .map(|c| c.to_ascii_lowercase())
            .collect::<Vec<_>>();
        if chars.len() != 7 {
            return None;
        }
        if chars.remove(0) != '#' {
            return None;
        }
        if chars.iter().any(|c| !c.is_ascii_hexdigit()) {
            return None;
        }
        let r = to_hex_digit(chars[0]) * 16 + to_hex_digit(chars[1]);
        let g = to_hex_digit(chars[2]) * 16 + to_hex_digit(chars[3]);
        let b = to_hex_digit(chars[4]) * 16 + to_hex_digit(chars[5]);
        Some(Self {
            color: ColorValue::Color(Color::Rgb { r, g, b }),
        })
    }
}

impl Default for Colour {
    fn default() -> Self {
        Self {
            color: ColorValue::NoColor(),
        }
    }
}

impl From<&str> for Colour {
    fn from(s: &str) -> Self {
        Self::from_hex(s).unwrap()
    }
}

impl From<Color> for Colour {
    fn from(value: Color) -> Self {
        Self {
            color: ColorValue::Color(value),
        }
    }
}
