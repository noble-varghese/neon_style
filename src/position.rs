use std::cmp;

use textwrap::core::display_width;

use crate::{
    align::{get_lines, get_strs_height},
    whitespace::{WhiteSpace, WhiteSpaceType},
};

#[derive(Clone, Copy, Debug)]
pub enum Position {
    Top,
    Bottom,
    Center,
    Left,
    Right,
}

pub fn place(
    width: i32,
    height: i32,
    h_pos: Position,
    v_pos: Position,
    strs: String,
    opts: &[WhiteSpaceType],
) -> String {
    return place_vertical(
        height,
        v_pos,
        place_horizontal(width, h_pos, strs, opts),
        opts,
    );
}

pub fn place_horizontal(
    width: i32,
    pos: Position,
    strs: String,
    opts: &[WhiteSpaceType],
) -> String {
    let binding = strs.to_string();
    let (lines, content_width) = get_lines(&binding);
    let gap = width as usize - content_width;
    if gap <= 0 {
        return strs;
    }

    let mut ws = WhiteSpace::new(opts);

    let mut b = String::new();

    for (i, line) in lines.clone().enumerate() {
        let short = cmp::max(0, content_width - display_width(line));
        match pos {
            Position::Left => {
                b.push_str(&line);
                b.push_str(&ws.render(gap + short));
            }
            Position::Right => {
                b.push_str(&ws.render(gap + short));
                b.push_str(&line);
            }
            _ => {
                let left = (gap + short) / 2;
                let right = (gap + short) / 2;
                b.push_str(&ws.render(left));
                b.push_str(&line);
                b.push_str(&ws.render(right));
            }
        }

        if i < lines.clone().count() - 1 {
            b.push('\n');
        }
    }
    b
}

pub fn place_vertical(height: i32, pos: Position, strs: String, opts: &[WhiteSpaceType]) -> String {
    let content_height = get_strs_height(&strs);
    let gap = height as usize - content_height;
    if gap <= 0 {
        return strs;
    }

    let mut ws = WhiteSpace::new(opts);

    let (_, width) = get_lines(&strs);
    let empty_line = format!("{}\n", ws.render(width));

    let mut b = String::new();
    match pos {
        Position::Bottom => {
            b.push_str(&empty_line.repeat(gap));
            b.push_str(&strs);
        }
        Position::Top => {
            b.push_str(&strs);
            for _ in 0..gap {
                b.push('\n');
                b.push_str(&empty_line);
            }
        }
        _ => {
            let top = gap / 2;
            let bottom = gap / 2;
            b.push_str(&empty_line.repeat(top));
            b.push_str(&strs);
            for _ in 0..bottom {
                b.push('\n');
                b.push_str(&empty_line);
            }
        }
    }
    b
}
