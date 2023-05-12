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

pub fn normal_border() -> Border {
    Border {
        top: String::from("─"),
        bottom: String::from("─"),
        left: String::from("│"),
        right: String::from("│"),
        top_left: String::from("┌"),
        top_right: String::from("┐"),
        bottom_left: String::from("└"),
        bottom_right: String::from("┘"),
    }
}

pub fn rounded_border() -> Border {
    Border {
        top: String::from("─"),
        bottom: String::from("─"),
        left: String::from("│"),
        right: String::from("│"),
        top_left: String::from("╭"),
        top_right: String::from("╮"),
        bottom_left: String::from("╰"),
        bottom_right: String::from("╯"),
    }
}

pub fn block_border() -> Border {
    Border {
        top: String::from("█"),
        bottom: String::from("█"),
        left: String::from("█"),
        right: String::from("█"),
        top_left: String::from("█"),
        top_right: String::from("█"),
        bottom_left: String::from("█"),
        bottom_right: String::from("█"),
    }
}

pub fn outer_half_block_border() -> Border {
    Border {
        top: String::from("▀"),
        bottom: String::from("▄"),
        left: String::from("▌"),
        right: String::from("▐"),
        top_left: String::from("▛"),
        top_right: String::from("▜"),
        bottom_left: String::from("▙"),
        bottom_right: String::from("▟"),
    }
}

pub fn inner_half_block_border() -> Border {
    Border {
        top: "▄".into(),
        bottom: "▀".into(),
        left: "▐".into(),
        right: "▌".into(),
        top_left: "▗".into(),
        top_right: "▖".into(),
        bottom_left: "▝".into(),
        bottom_right: "▘".into(),
    }
}

pub fn thick_border() -> Border {
    Border {
        top: "━".into(),
        bottom: "━".into(),
        left: "┃".into(),
        right: "┃".into(),
        top_left: "┏".into(),
        top_right: "┓".into(),
        bottom_left: "┗".into(),
        bottom_right: "┛".into(),
    }
}

pub fn double_border() -> Border {
    Border {
        top: "═".to_string(),
        bottom: "═".to_string(),
        left: "║".to_string(),
        right: "║".to_string(),
        top_left: "╔".to_string(),
        top_right: "╗".to_string(),
        bottom_left: "╚".to_string(),
        bottom_right: "╝".to_string(),
    }
}

pub fn hidden_border() -> Border {
    Border {
        top: " ".into(),
        bottom: " ".into(),
        left: " ".into(),
        right: " ".into(),
        top_left: " ".into(),
        top_right: " ".into(),
        bottom_left: " ".into(),
        bottom_right: " ".into(),
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
    let chars: Vec<String> = middle.chars().map(|c| c.to_string()).collect();
    compiled_string.push_str(left);

    while i < width {
        compiled_string.push_str(&chars[j]);
        j += 1;
        if j >= chars.len() {
            j = 0
        }
        i += display_width(&chars[j]);
    }
    compiled_string.push_str(right);

    compiled_string
}
