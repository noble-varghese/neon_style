use std::{cmp, collections::HashMap};

use crossterm::style::Attribute;

use crate::{
    align::align_text_vertical, color::TerminalColor, position::Position, renderer::Renderer,
};

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
    Int(usize),
    Color(Box<dyn TerminalColor>),
    Pos(Position),
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

    pub fn get_as_int(&self, prop: Props) -> usize {
        if self.rules.contains_key(&prop) {
            if let Value::Int(val) = self.rules.get(&prop).unwrap() {
                return *val;
            }
        }
        0
    }

    pub fn get_as_position(&self, prop: Props) -> Position {
        if self.rules.contains_key(&prop) {
            if let Value::Pos(val) = self.rules.get(&prop).unwrap() {
                return *val;
            }
        }
        Position::Top
    }

    // pub fn get_as_color(&self, prop: Props) -> Box<dyn TerminalColor> {
    //     if self.rules.contains_key(&prop) {
    //         if let Value::Color(val) = self.rules.get(&prop).unwrap() {
    //             return *val;
    //         }
    //     }
    //     Box::new(NoColor)
    // }

    pub fn style(style: &str, strs: &str) -> String {
        let mut compiled_string = String::new();
        compiled_string.push_str(&style);
        compiled_string.push_str(strs);
        compiled_string.push_str(&Attribute::Reset.to_string());
        compiled_string
    }

    pub fn style_char(style: &str, ch: char) -> String {
        let mut compiled_string = String::new();
        compiled_string.push_str(&style);
        compiled_string.push(ch);
        compiled_string.push_str(&Attribute::Reset.to_string());
        compiled_string
    }

    pub fn bold(mut self, value: bool) -> Self {
        self.set(Props::BoldKey, Value::Bool(value));
        self
    }

    pub fn italic(mut self, value: bool) -> Self {
        self.set(Props::ItalicKey, Value::Bool(value));
        self
    }

    pub fn underline(mut self, value: bool) -> Self {
        self.set(Props::UnderlineKey, Value::Bool(value));
        self
    }

    pub fn strikethrough(mut self, value: bool) -> Self {
        self.set(Props::StrikethroughKey, Value::Bool(value));
        self
    }

    pub fn reverse(mut self, value: bool) -> Self {
        self.set(Props::ReverseKey, Value::Bool(value));
        self
    }

    pub fn blink(mut self, value: bool) -> Self {
        self.set(Props::BlinkKey, Value::Bool(value));
        self
    }

    pub fn faint(mut self, value: bool) -> Self {
        self.set(Props::FaintKey, Value::Bool(value));
        self
    }

    pub fn underline_spaces(mut self, value: bool) -> Self {
        self.set(Props::UnderlineSpacesKey, Value::Bool(value));
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

    pub fn render(&mut self, strs: &str) -> String {
        // The final compiled string to be returned after all the operations.
        let mut compiled_string = String::new();
        compiled_string.push_str(strs);

        if self.r.is_none() {
            self.r = Some(Renderer::new());
        }
        if self.value == "" {
            compiled_string.push_str(&self.value);
        }

        if self.rules.len() == 0 {
            return compiled_string.to_string();
        }
        let mut te = String::new();
        let mut te_space = String::new();
        // let mut te_white_space = String::new();

        let bold = self.get_as_bool(Props::BoldKey, false);
        let italic = self.get_as_bool(Props::ItalicKey, false);
        let underline = self.get_as_bool(Props::UnderlineKey, false);
        let strikethrough = self.get_as_bool(Props::StrikethroughKey, false);
        let reverse = self.get_as_bool(Props::ReverseKey, false);
        let blink = self.get_as_bool(Props::BlinkKey, false);
        let faint = self.get_as_bool(Props::FaintKey, false);

        let width = self.get_as_int(Props::WidthKey);
        let height = self.get_as_int(Props::HeightKey);
        let horizontal_align = self.get_as_position(Props::AlignHorizontalKey);
        let vertical_align = self.get_as_position(Props::AlignVerticalKey);

        let top_padding = self.get_as_int(Props::PaddingTopKey);
        let right_padding = self.get_as_int(Props::PaddingRightKey);
        let bottom_padding = self.get_as_int(Props::PaddingBottomKey);
        let left_padding = self.get_as_int(Props::PaddingLeftKey);

        let inline = self.get_as_bool(Props::InlineKey, false);

        // let color_whitespaces = self.get_as_bool(Props::ColorWhitespaceKey, true);
        let underline_spaces = underline && self.get_as_bool(Props::UnderlineSpacesKey, true);
        let strikethrough_spaces =
            strikethrough && self.get_as_bool(Props::StrikethroughSpacesKey, true);

        // let style_whitespace = reverse;
        let use_space_styler = !underline_spaces || !strikethrough_spaces;

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

        if inline {
            compiled_string = compiled_string.replace('\n', "");
        }

        // Word wrap feature.
        // TODO: Handle the case of text wrapping with hyphenation.
        if !inline && width > 0 {
            let wrap_at = (width - left_padding - right_padding) as usize;
            compiled_string = textwrap::fill(&compiled_string, wrap_at);
        }

        // Rendering the core text here. Inside a code block to delete the temp values
        // once it goes out of scope.
        {
            let mut temp = String::new();
            let l = compiled_string.split('\n');
            for (i, line) in l.clone().enumerate() {
                // Identify the spaces and applying the styling separately to the spaces.
                // This only works for underscores and strikethroughs
                if use_space_styler {
                    for ch in line.chars() {
                        if ch.is_whitespace() {
                            temp.push_str(&Style::style_char(&te_space, ch));
                            continue;
                        }
                        temp.push_str(&Style::style_char(&te, ch));
                    }
                } else {
                    temp.push_str(&Style::style(&te, line))
                }

                if i != l.clone().count() - 1 {
                    temp.push('\n');
                }
            }
            compiled_string = temp;
        }

        if !inline {
            if left_padding > 0 {
                compiled_string = pad_left(&mut compiled_string, left_padding);
            }
            if right_padding > 0 {
                compiled_string = pad_right(&mut compiled_string, left_padding);
            }
            if top_padding > 0 {
                let mut sp = "\n".repeat(top_padding as usize);

                sp.push_str(&compiled_string);
                compiled_string = sp;
            }
            if bottom_padding > 0 {
                let sp = "\n".repeat(top_padding as usize);
                compiled_string.push_str(&sp);
            }
        }

        if height > 0 {
            // Aligns the text to top, bottom or center vertically.
            compiled_string = align_text_vertical(&mut compiled_string, vertical_align, height)
        }

        compiled_string
    }
}

fn pad_left(strs: &mut str, n: usize) -> String {
    let sp = "\n".repeat(n);
    if n == 0 {
        return strs.to_string();
    }
    let mut temp = String::new();
    let lines = strs.split('\n');
    for (i, line) in lines.clone().enumerate() {
        temp.push_str(&sp);
        temp.push_str(line);
        if i < lines.clone().count() - 1 {
            temp.push('\n');
        }
    }
    return temp;
}

fn pad_right(strs: &mut str, n: usize) -> String {
    let sp = "\n".repeat(n);
    if n == 0 {
        return strs.to_string();
    }
    let mut temp = String::new();
    let lines = strs.split('\n');
    for (i, line) in lines.clone().enumerate() {
        temp.push_str(line);
        temp.push_str(&sp);
        if i < lines.clone().count() - 1 {
            temp.push('\n');
        }
    }
    return temp;
}
