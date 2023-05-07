use std::{cmp, collections::HashMap, fmt::Write};

use crossterm::style::Attribute;

use crate::{color::TerminalColor, renderer::Renderer};

#[derive(Eq, Hash, PartialEq)]
pub enum Props {
    BoldKey,
    ItalicKey,
    UnderlineKey,
    StrikethroughKey,
    ReverseKey,
    BlinkKey,
    FaintKey,
    ForegroundKey,
    BackgroundKey,
    WidthKey,
    HeightKey,
    AlignHorizontalKey,
    AlignVerticalKey,

    // Padding.,
    PaddingTopKey,
    PaddingRightKey,
    PaddingBottomKey,
    PaddingLeftKey,

    ColorWhitespaceKey,

    // Margins.,
    MarginTopKey,
    MarginRightKey,
    MarginBottomKey,
    MarginLeftKey,
    MarginBackgroundKey,

    // Border runes.,
    BorderStyleKey,

    // Border edges.,
    BorderTopKey,
    BorderRightKey,
    BorderBottomKey,
    BorderLeftKey,

    // Border foreground colors.,
    BorderTopForegroundKey,
    BorderRightForegroundKey,
    BorderBottomForegroundKey,
    BorderLeftForegroundKey,

    // Border background colors.,
    BorderTopBackgroundKey,
    BorderRightBackgroundKey,
    BorderBottomBackgroundKey,
    BorderLeftBackgroundKey,

    InlineKey,
    MaxWidthKey,
    MaxHeightKey,
    UnderlineSpacesKey,
    StrikethroughSpacesKey,
}

pub enum Value {
    Str(String),
    Bool(bool),
    Int(i32),
    Color(Box<dyn TerminalColor>),
}

pub struct Style {
    pub value: String,
    pub rules: HashMap<Props, Value>,
    pub r: Option<Renderer>,
}

impl Style {
    pub fn new_style() -> Self {
        Self {
            value: String::new(),
            rules: HashMap::new(),
            r: None,
        }
    }

    pub fn get_as_bool(&self, prop: Props, default_val: bool) -> bool {
        if self.rules.contains_key(&prop) {
            if let Value::Bool(val) = self.rules.get(&prop).unwrap() {
                return *val;
            }
        }
        default_val
    }

    pub fn get_as_int(&self, prop: Props) -> i32 {
        if self.rules.contains_key(&prop) {
            if let Value::Int(val) = self.rules.get(&prop).unwrap() {
                return *val;
            }
        }
        0
    }

    // pub fn get_as_color(&self, prop: Props) -> Box<dyn TerminalColor> {
    //     if self.rules.contains_key(&prop) {
    //         if let Value::Color(val) = self.rules.get(&prop).unwrap() {
    //             return *val;
    //         }
    //     }
    //     Box::new(NoColor)
    // }

    pub fn style(style: &str, input_strs: &str) -> String {
        let mut compiled_string = String::new();
        compiled_string.push_str(&style);
        compiled_string.push_str(&input_strs);
        compiled_string.push_str(&Attribute::Reset.to_string());
        compiled_string
    }

    pub fn bold(mut self, value: bool) -> Self {
        self.set(Props::BoldKey, Value::Bool(value));
        self
    }

    pub fn set(&mut self, key: Props, value: Value) {
        match value {
            Value::Int(val) => {
                // Inorder to eliminate the negative values.
                self.rules.insert(key, Value::Int(cmp::max(0, val)));
            }
            _ => {
                self.rules.insert(key, value);
            }
        }
    }

    pub fn render(&mut self, strs: &mut str) -> String {
        if self.r.is_none() {
            self.r = Some(Renderer::new());
        }
        if self.value == "" {
            strs.to_owned().push_str(&self.value);
        }

        // if self.rules.len() == 0 {
        //     return self.value.to_string();
        // }
        let mut te = String::new();
        let mut te_space = String::new();
        let mut te_white_space = String::new();

        let bold = self.get_as_bool(Props::BoldKey, false);
        let italic = self.get_as_bool(Props::ItalicKey, false);
        let underline = self.get_as_bool(Props::UnderlineKey, false);
        let strikethrough = self.get_as_bool(Props::StrikethroughKey, false);
        let reverse = self.get_as_bool(Props::ReverseKey, false);
        let blink = self.get_as_bool(Props::BlinkKey, false);
        let faint = self.get_as_bool(Props::FaintKey, false);

        let color_whitespaces = self.get_as_bool(Props::ColorWhitespaceKey, true);
        let underline_spaces = underline && self.get_as_bool(Props::UnderlineSpacesKey, true);
        let strikethrough_spaces =
            strikethrough && self.get_as_bool(Props::StrikethroughSpacesKey, true);

        let style_whitespace = reverse;
        let use_space_styler = underline_spaces || strikethrough_spaces;

        if bold {
            te.push_str(&Attribute::Bold.to_string());
        }

        if italic {
            te.push_str(&Attribute::Italic.to_string());
        }

        if underline {
            te.push_str(&Attribute::Underlined.to_string());
        }
        if strikethrough {
            te.push_str(&Attribute::CrossedOut.to_string());
        }
        if reverse {
            te.push_str(&Attribute::Reverse.to_string());
        }
        if blink {
            te.push_str(&Attribute::RapidBlink.to_string());
        }
        if faint {
            te.push_str(&Attribute::Dim.to_string());
        }

        if underline_spaces {
            te_space.push_str(&Attribute::Underlined.to_string());
        }

        if strikethrough_spaces {
            te_space.push_str(&Attribute::CrossedOut.to_string());
        }
        let l = strs.split("\n");
        let mut compiled_str = String::new();
        for i in l {
            compiled_str.push_str(&Style::style(&te, i).to_string());
        }
        compiled_str
    }
}
