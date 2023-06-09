use crossterm::style::Attribute;

pub fn pad_left(strs: &str, n: usize, style: Option<&String>) -> String {
    let mut sp = " ".repeat(n);
    if n == 0 {
        return strs.to_string();
    }
    if let Some(st) = style {
        sp = format!("{}{}", st, sp);
    }
    let mut temp = String::new();
    let lines = strs.split("\n");
    for (i, line) in lines.clone().enumerate() {
        temp.push_str(&format!("{}{}{}", &sp, line, &Attribute::Reset.to_string()));
        if i != lines.clone().count() - 1 {
            temp.push_str("\n");
        }
    }
    return temp;
}

pub fn pad_right(strs: &str, n: usize, style: Option<&String>) -> String {
    let mut sp = " ".repeat(n);
    if n == 0 {
        return strs.to_string();
    }
    if let Some(st) = style {
        sp = format!("{}{}", st, sp);
    }
    let mut temp = String::new();
    let lines = strs.split("\n");
    for (i, line) in lines.clone().enumerate() {
        temp.push_str(&format!("{}{}{}", line, &sp, &Attribute::Reset.to_string()));
        if i != lines.clone().count() - 1 {
            temp.push_str("\n");
        }
    }
    return temp;
}

pub fn pad_top(strs: &str, n: usize) -> String {
    let mut sp = "\n".repeat(n);
    sp = format!("{sp}{strs}");
    sp
}

pub fn pad_bottom(strs: &str, n: usize) -> String {
    let mut sp = "\n".repeat(n);
    sp = format!("{strs}{sp}");
    sp
}
