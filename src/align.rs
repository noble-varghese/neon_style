use crate::position::Position;

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
