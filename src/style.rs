use std::{cmp, collections::HashMap, usize};

use crossterm::style::{Attribute, Color, SetBackgroundColor, SetForegroundColor};
use textwrap::core::display_width;

use crate::{
    align::{align_text_horizontal, align_text_vertical, get_lines},
    border::{get_first_char_as_string, render_horizontal_edge, Border},
    color::{ColorValue, Colour},
    padding::{pad_bottom, pad_left, pad_right, pad_top},
    position::Position,
    renderer::Renderer,
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
    TextColorKey,
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
    Color(Colour),
    Pos(Position),
    Border(Border),
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

    fn get_border_style(&self) -> Border {
        if self.rules.contains_key(&Props::BorderStyleKey) {
            if let Value::Border(val) = self.rules.get(&Props::BorderStyleKey).unwrap() {
                return val.clone();
            }
        }
        Border::default()
    }

    pub fn get_as_color(&self, prop: Props) -> Colour {
        if self.rules.contains_key(&prop) {
            if let Value::Color(val) = self.rules.get(&prop).unwrap() {
                return val.clone();
            }
        }
        Colour::default()
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

    pub fn padding(mut self, values: &[i32]) -> Self {
        if values.len() > 4 {
            panic!("Cannot provide more than 4 values for padding");
        }
        let (top, right, bottom, left) = which_sides_int(&values);

        self.set(Props::PaddingTopKey, Value::Int(top as usize));
        self.set(Props::PaddingBottomKey, Value::Int(bottom as usize));
        self.set(Props::PaddingLeftKey, Value::Int(left as usize));
        self.set(Props::PaddingRightKey, Value::Int(right as usize));
        self
    }

    pub fn padding_top(mut self, value: i32) -> Self {
        self.set(Props::PaddingTopKey, Value::Int(value as usize));
        self
    }

    pub fn padding_bottom(mut self, value: i32) -> Self {
        self.set(Props::PaddingBottomKey, Value::Int(value as usize));
        self
    }

    pub fn padding_right(mut self, value: i32) -> Self {
        self.set(Props::PaddingRightKey, Value::Int(value as usize));
        self
    }

    pub fn padding_left(mut self, value: i32) -> Self {
        self.set(Props::PaddingLeftKey, Value::Int(value as usize));
        self
    }

    pub fn border(mut self, b: Border, sides: &[bool]) -> Self {
        self.set(Props::BorderStyleKey, Value::Border(b));
        let (top, right, bottom, left) = which_sides_bool(&sides);
        self.set(Props::BorderTopKey, Value::Bool(top));
        self.set(Props::BorderBottomKey, Value::Bool(bottom));
        self.set(Props::BorderLeftKey, Value::Bool(left));
        self.set(Props::BorderRightKey, Value::Bool(right));
        self
    }

    pub fn border_foreground(mut self, cols: &[Colour]) -> Self {
        if cols.len() > 4 {
            panic!("Cannot provide more than 4 values for border color");
        }
        let (top, right, bottom, left) = which_sides_color(cols);
        self.set(Props::BorderTopForegroundKey, Value::Color(top));
        self.set(Props::BorderBottomForegroundKey, Value::Color(bottom));
        self.set(Props::BorderRightForegroundKey, Value::Color(right));
        self.set(Props::BorderLeftForegroundKey, Value::Color(left));
        self
    }

    pub fn border_background(mut self, cols: &[Colour]) -> Self {
        if cols.len() > 4 {
            panic!("Cannot provide more than 4 values for border color");
        }
        let (top, right, bottom, left) = which_sides_color(cols);
        self.set(Props::BorderTopBackgroundKey, Value::Color(top));
        self.set(Props::BorderBottomBackgroundKey, Value::Color(bottom));
        self.set(Props::BorderRightBackgroundKey, Value::Color(right));
        self.set(Props::BorderLeftBackgroundKey, Value::Color(left));
        self
    }

    pub fn foreground(mut self, c: Colour) -> Self {
        self.set(Props::ForegroundKey, Value::Color(c));
        self
    }

    pub fn background(mut self, c: Colour) -> Self {
        self.set(Props::BackgroundKey, Value::Color(c));
        self
    }

    pub fn text_color(mut self, c: Colour) -> Self {
        self.set(Props::TextColorKey, Value::Color(c));
        self
    }

    fn apply_border(&self, strs: &str) -> String {
        let top_set = self.is_set(Props::BorderTopKey);
        let right_set = self.is_set(Props::BorderRightKey);
        let bottom_set = self.is_set(Props::BorderBottomKey);
        let left_set = self.is_set(Props::BorderLeftKey);

        let mut border = self.get_border_style();
        let mut has_top = self.get_as_bool(Props::BorderTopKey, false);
        let mut has_right = self.get_as_bool(Props::BorderRightKey, false);
        let mut has_bottom = self.get_as_bool(Props::BorderBottomKey, false);
        let mut has_left = self.get_as_bool(Props::BorderLeftKey, false);

        let top_fg = self.get_as_color(Props::BorderTopForegroundKey);
        let bottom_fg = self.get_as_color(Props::BorderBottomForegroundKey);
        let right_fg = self.get_as_color(Props::BorderRightForegroundKey);
        let left_fg = self.get_as_color(Props::BorderLeftForegroundKey);

        let top_bg = self.get_as_color(Props::BorderTopBackgroundKey);
        let right_bg = self.get_as_color(Props::BorderRightBackgroundKey);
        let left_bg = self.get_as_color(Props::BorderLeftBackgroundKey);
        let bottom_bg = self.get_as_color(Props::BorderBottomBackgroundKey);

        // if border is not set or all the sides have been disabled then return the str as it is without applying borders.
        if border == Border::default() || (!top_set && !right_set && !bottom_set && !left_set) {
            return strs.to_string();
        }

        // If the border is set and no side is specifically mentioned then apply borders to all sides.
        if border != Border::default() && !(top_set || right_set || bottom_set || left_set) {
            has_top = true;
            has_right = true;
            has_bottom = true;
            has_left = true;
        }

        let (lines, mut width) = get_lines(strs);

        if has_left {
            if border.left.is_empty() {
                border.left = String::from(" ");
            }
            width += display_width(&border.left);
        }

        if has_right {
            if border.right.is_empty() {
                border.right = String::from(" ");
            }
            width += display_width(&border.right);
        }

        if has_top && has_left && border.top_left.is_empty() {
            border.top_left = String::from(" ");
        }

        if has_top && has_right && border.top_right.is_empty() {
            border.top_right = String::from(" ");
        }

        if has_bottom && has_left && border.bottom_left.is_empty() {
            border.bottom_left = String::from(" ");
        }

        if has_bottom && has_right && border.bottom_right.is_empty() {
            border.bottom_right = String::from(" ");
        }

        if has_top {
            match (has_left, has_right) {
                (false, false) => {
                    border.top_left = String::from("");
                    border.top_right = String::from("");
                }
                (true, false) => {
                    border.top_right = String::from("");
                }
                (false, true) => {
                    border.top_left = String::from("");
                }
                _ => {}
            }
        }

        if has_bottom {
            match (has_left, has_right) {
                (false, false) => {
                    border.bottom_left = String::from("");
                    border.bottom_right = String::from("");
                }
                (true, false) => {
                    border.bottom_right = String::from("");
                }
                (false, true) => {
                    border.bottom_left = String::from("");
                }
                _ => {}
            }
        }

        border.top_left = get_first_char_as_string(&border.top_left);
        border.top_right = get_first_char_as_string(&border.top_right);
        border.bottom_left = get_first_char_as_string(&border.bottom_left);
        border.bottom_right = get_first_char_as_string(&border.bottom_right);

        let mut compiled_string = String::new();

        if has_top {
            let mut top =
                render_horizontal_edge(&border.top_left, &border.top, &border.top_right, width);
            top = style_border(&top, top_fg, top_bg);
            compiled_string.push_str(&top);
            compiled_string.push('\n');
        }
        let mut left_index = 0;
        let mut right_index = 0;
        for (i, line) in lines.clone().enumerate() {
            if has_left {
                let left_chars: Vec<String> = border.left.chars().map(|c| c.to_string()).collect();
                let r = &left_chars[left_index];
                left_index += 1;
                if left_index >= left_chars.len() {
                    left_index = 0;
                }
                compiled_string.push_str(&style_border(&r, left_fg, left_bg))
            }
            compiled_string.push_str(&line);
            if has_right {
                let right_chars: Vec<String> =
                    border.right.chars().map(|c| c.to_string()).collect();
                let r = &right_chars[right_index];
                right_index += 1;
                if right_index >= right_chars.len() {
                    right_index = 0;
                }
                compiled_string.push_str(&style_border(&r, right_fg, right_bg))
            }
            if i < lines.clone().count() - 1 {
                compiled_string.push('\n')
            }
        }

        if has_bottom {
            let mut bottom = render_horizontal_edge(
                &border.bottom_left,
                &border.bottom,
                &border.bottom_right,
                width,
            );
            bottom = style_border(&bottom, bottom_fg, bottom_bg);
            compiled_string.push('\n');
            compiled_string.push_str(&bottom);
        }

        return compiled_string;
    }

    fn is_set(&self, key: Props) -> bool {
        if self.rules.contains_key(&key) {
            return true;
        }
        false
    }

    fn set(&mut self, key: Props, value: Value) {
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
        let mut te_white_space = String::new();

        let bold = self.get_as_bool(Props::BoldKey, false);
        let italic = self.get_as_bool(Props::ItalicKey, false);
        let underline = self.get_as_bool(Props::UnderlineKey, false);
        let strikethrough = self.get_as_bool(Props::StrikethroughKey, false);
        let reverse = self.get_as_bool(Props::ReverseKey, false);
        let blink = self.get_as_bool(Props::BlinkKey, false);
        let faint = self.get_as_bool(Props::FaintKey, false);

        let fg = self.get_as_color(Props::ForegroundKey);
        let bg = self.get_as_color(Props::BackgroundKey);
        let text_color = self.get_as_color(Props::TextColorKey);

        // Text width height and alignment related components.
        let width = self.get_as_int(Props::WidthKey);
        let height = self.get_as_int(Props::HeightKey);
        let horizontal_align = self.get_as_position(Props::AlignHorizontalKey);
        let vertical_align = self.get_as_position(Props::AlignVerticalKey);

        // Padding related components
        let top_padding = self.get_as_int(Props::PaddingTopKey);
        let right_padding = self.get_as_int(Props::PaddingRightKey);
        let bottom_padding = self.get_as_int(Props::PaddingBottomKey);
        let left_padding = self.get_as_int(Props::PaddingLeftKey);

        let inline = self.get_as_bool(Props::InlineKey, false);

        let color_whitespaces = self.get_as_bool(Props::ColorWhitespaceKey, true);
        let underline_spaces = underline && self.get_as_bool(Props::UnderlineSpacesKey, true);
        let strikethrough_spaces =
            strikethrough && self.get_as_bool(Props::StrikethroughSpacesKey, true);

        let style_whitespace = reverse;
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

        if text_color != Colour::default() {
            if let ColorValue::Color(val) = text_color.color {
                te.push_str(&SetForegroundColor(val).to_string())
            }
        }

        if bg != Colour::default() {
            if let ColorValue::Color(val) = bg.color {
                te.push_str(&SetBackgroundColor(val).to_string());
                if color_whitespaces {
                    te_white_space =
                        format!("{}{te_white_space}", &SetBackgroundColor(val).to_string());
                }

                if use_space_styler {
                    te_space = format!("{}{te_space}", &SetBackgroundColor(val).to_string());
                }
            }
        }

        if fg != Colour::default() {
            if let ColorValue::Color(val) = fg.color {
                te.push_str(&SetForegroundColor(val).to_string());
                if color_whitespaces {
                    te_white_space =
                        format!("{}{te_white_space}", &SetForegroundColor(val).to_string());
                }

                if use_space_styler {
                    te_space = format!("{}{te_space}", &SetForegroundColor(val).to_string());
                }
            }
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
                            temp.push_str(&format!("{}{}{}", te_space, ch, Attribute::Reset));
                            continue;
                        }
                        temp.push_str(&format!("{}{}{}", te, ch, Attribute::Reset));
                    }
                } else {
                    temp.push_str(&format!(
                        "{}{}{}{:?}",
                        te,
                        line,
                        Attribute::Reset,
                        Color::Reset
                    ))
                }

                if i != l.clone().count() - 1 {
                    temp.push('\n');
                }
            }
            compiled_string = temp;
        }

        if !inline {
            if left_padding > 0 {
                let mut style: Option<&String> = None;
                if color_whitespaces || style_whitespace {
                    style = Some(&te_white_space);
                }
                compiled_string = pad_left(&compiled_string, left_padding, style);
            }
            if right_padding > 0 {
                let mut style: Option<&String> = None;
                if color_whitespaces || style_whitespace {
                    style = Some(&te_white_space);
                }
                compiled_string = pad_right(&compiled_string, left_padding, style);
            }
            if top_padding > 0 {
                compiled_string = pad_top(&compiled_string, top_padding)
            }
            if bottom_padding > 0 {
                compiled_string = pad_bottom(&compiled_string, bottom_padding);
            }
        }

        if height > 0 {
            // Aligns the text to top, bottom or center vertically.
            compiled_string = align_text_vertical(&mut compiled_string, vertical_align, height)
        }

        {
            let num_lines = compiled_string.split('\n').count();
            if !(num_lines == 0 && width == 0) {
                let mut style: Option<&String> = None;
                if color_whitespaces || style_whitespace {
                    style = Some(&te_white_space);
                }
                compiled_string =
                    align_text_horizontal(&compiled_string, horizontal_align, width, style)
            }
        }

        if !inline {
            compiled_string = self.apply_border(&compiled_string);
        }

        compiled_string
    }
}

fn which_sides_int(values: &[i32]) -> (i32, i32, i32, i32) {
    let [mut top, mut bottom, mut left, mut right] = [0, 0, 0, 0];
    match values.len() {
        1 => {
            top = values[0];
            bottom = values[0];
            left = values[0];
            right = values[0];
        }
        2 => {
            top = values[0];
            bottom = values[0];
            left = values[1];
            right = values[1];
        }
        3 => {
            top = values[0];
            left = values[1];
            right = values[1];
            bottom = values[2];
        }
        4 => {
            top = values[0];
            right = values[1];
            bottom = values[2];
            left = values[3];
        }
        _ => {}
    }
    return (top, right, bottom, left);
}

fn style_border(border: &str, fg: Colour, bg: Colour) -> String {
    let mut compiled_string = String::new();
    if fg == Colour::default() && bg == Colour::default() {
        return border.to_string();
    }

    if fg != Colour::default() {
        if let ColorValue::Color(val) = fg.color {
            compiled_string.push_str(&SetForegroundColor(val).to_string());
        }
    }

    if bg != Colour::default() {
        if let ColorValue::Color(val) = bg.color {
            compiled_string.push_str(&SetBackgroundColor(val).to_string());
        }
    }
    compiled_string.push_str(border);
    compiled_string.push_str(&Attribute::Reset.to_string());
    compiled_string
}

fn which_sides_bool(values: &[bool]) -> (bool, bool, bool, bool) {
    let [mut top, mut bottom, mut left, mut right] = [false, false, false, false];
    match values.len() {
        1 => {
            top = values[0];
            bottom = values[0];
            left = values[0];
            right = values[0];
        }
        2 => {
            top = values[0];
            bottom = values[0];
            left = values[1];
            right = values[1];
        }
        3 => {
            top = values[0];
            left = values[1];
            right = values[1];
            bottom = values[2];
        }
        4 => {
            top = values[0];
            right = values[1];
            bottom = values[2];
            left = values[3];
        }
        _ => {}
    }
    return (top, right, bottom, left);
}

fn which_sides_color(values: &[Colour]) -> (Colour, Colour, Colour, Colour) {
    let [mut top, mut bottom, mut left, mut right] = [
        Colour::default(),
        Colour::default(),
        Colour::default(),
        Colour::default(),
    ];
    match values.len() {
        1 => {
            top = values[0];
            bottom = values[0];
            left = values[0];
            right = values[0];
        }
        2 => {
            top = values[0];
            bottom = values[0];
            left = values[1];
            right = values[1];
        }
        3 => {
            top = values[0];
            left = values[1];
            right = values[1];
            bottom = values[2];
        }
        4 => {
            top = values[0];
            right = values[1];
            bottom = values[2];
            left = values[3];
        }
        _ => {}
    }
    return (top, right, bottom, left);
}
