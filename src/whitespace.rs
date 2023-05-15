use crossterm::style::{Attribute, SetBackgroundColor, SetForegroundColor};
use textwrap::core::display_width;

use crate::{color::ColorValue, Color};

pub struct WhiteSpace {
    pub style: String,
    pub chars: String,
}

pub enum WhiteSpaceType {
    Style(String),
    Chars(String),
}

impl WhiteSpace {
    pub fn new(rules: &[WhiteSpaceType]) -> Self {
        let mut style = String::new();
        let mut chars = String::new();
        for r in rules {
            match r {
                WhiteSpaceType::Style(val) => style.push_str(val),
                WhiteSpaceType::Chars(val) => chars.push_str(val),
            }
        }
        Self { style, chars }
    }

    pub fn render(&mut self, width: usize) -> String {
        if self.chars.is_empty() {
            self.chars = String::from(" ");
        }

        let r: Vec<String> = self.chars.chars().map(|c| c.to_string()).collect();
        let mut j = 0;
        let mut i = 0;

        let mut b = format!("{}", &self.style);

        while i < width {
            b.push_str(&r[j]);
            j += 1;
            if j >= r.len() {
                j = 0;
            }
            i += display_width(&r[j]);
        }

        let short = width - display_width(&b);
        if short > 0 {
            let s = " ".repeat(short);
            b.push_str(&s);
        }
        b.push_str(&Attribute::Reset.to_string());
        b.to_string()
    }
}

pub fn with_whitespace_chars(strs: String) -> WhiteSpaceType {
    return WhiteSpaceType::Chars(strs);
}

pub fn with_whitespace_bg(c: Color) -> WhiteSpaceType {
    let mut b = String::new();
    if let ColorValue::Color(val) = c.color {
        b.push_str(&SetBackgroundColor(val).to_string());
    }
    WhiteSpaceType::Style(b)
}

pub fn with_whitespace_fg(c: Color) -> WhiteSpaceType {
    let mut b = String::new();
    if let ColorValue::Color(val) = c.color {
        b.push_str(&SetForegroundColor(val).to_string());
    }
    WhiteSpaceType::Style(b)
}
