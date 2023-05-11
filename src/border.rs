use textwrap::core::display_width;

#[derive(Clone, Default, PartialEq)]
pub struct Border {
    pub top: String,
    pub bottom: String,
    pub left: String,
    pub right: String,
    pub top_left: String,
    pub top_right: String,
    pub bottom_right: String,
    pub bottom_left: String,
}

impl Border {
    pub fn new() -> Self {
        Self {
            top: String::new(),
            bottom: String::new(),
            left: String::new(),
            right: String::new(),
            top_left: String::new(),
            top_right: String::new(),
            bottom_right: String::new(),
            bottom_left: String::new(),
        }
    }
}

pub fn get_first_char_as_string(strs: &str) -> String {
    if let Some(ch) = strs.chars().next() {
        return ch.to_string();
    }
    return String::from("");
}

pub fn render_horizontal_edge(left: &str, mut middle: &str, right: &str, width: usize) -> String {
    let mut compiled_string = String::new();
    if width < 1 {
        return String::from("");
    }

    if middle.is_empty() {
        middle = " ";
    }

    let left_width = display_width(left);
    let right_width = display_width(right);

    let mut j = 0;
    let mut i = left_width + right_width;
    let chars: Vec<char> = middle.chars().collect();
    while i < left_width + right_width + width {
        compiled_string.push_str(&chars[j].to_string());
        j += 1;
        if j >= chars.len() {
            j = 0
        }
        i += display_width(&chars[j].to_string());
    }
    compiled_string.push_str(right);

    compiled_string
}
