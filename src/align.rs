use crate::position::Position;
use std::{cmp, str::Split};
use textwrap::core::display_width;

fn get_lines(s: &str) -> (Split<char>, usize) {
    let lines = s.split('\n');
    let mut widest = 0;
    for (i, line) in lines.clone().enumerate() {
        // This gives us the printable width of string.
        // NOTE: printable width is not the same as the number of chars or bytes in a string. When working with Non-ASCII chars it may take up
        // more than one cell when printed.
        let w = display_width(line);
        if w > widest {
            widest = w;
        }
    }
    return (lines, widest);
}

pub fn align_text_vertical(strs: &mut String, pos: Position, height: usize) -> String {
    let str_height = strs
        .chars()
        .map(|ch| ch == '\n')
        .fold(0, |acc, x| if x { acc + 1 } else { acc });
    if height < str_height {
        return strs.to_string();
    }
    match pos {
        Position::Bottom => {
            let mut temp = "\n".repeat(height - str_height);
            temp.push_str(&strs);
            return temp;
        }
        Position::Center => {
            let mut top_height = (height - str_height) / 2;
            let mut bottom_height = (height - str_height) / 2;
            if (str_height + bottom_height + top_height) > height {
                top_height -= 1;
            } else if (str_height + bottom_height + top_height) < height {
                bottom_height += 1;
            }
            let mut temp = "\n".repeat(top_height);
            temp.push_str(&strs);
            temp.push_str(&"\n".repeat(bottom_height));
            return temp;
        }
        Position::Top => {
            let temp = "\n".repeat(height - str_height);
            strs.push_str(&temp);
            return strs.to_string();
        }
        _ => {}
    }
    return strs.to_string();
}

pub fn align_text_horizontal(strs: &String, pos: Position, width: usize) -> String {
    let (lines, widest_line) = get_lines(strs);
    let mut temp = String::new();
    for (i, line) in lines.clone().enumerate() {
        let line_width = display_width(line);
        let mut short_amount = widest_line - line_width;
        short_amount = cmp::max(0, width - (short_amount + line_width));

        if short_amount > 0 {
            match pos {
                Position::Center => {
                    let left = short_amount / 2;
                    let right = left + short_amount % 2;
                }
                Position::Right => {
                    let sp = " ".repeat(short_amount);
                    temp.push_str(&sp);
                    temp.push_str(strs);
                }
                _ => {
                    // Default case is left orientation.
                    let sp = " ".repeat(short_amount);
                    temp.push_str(strs);
                    temp.push_str(&sp);
                }
            }
        }
        if i < lines.clone().count() {
            temp.push('\n');
        }
    }
    return temp;
}
