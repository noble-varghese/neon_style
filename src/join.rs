use crate::{align::get_lines, position::Position};

pub fn join_horizontal(pos: Position, strs: &[String]) -> String {
    let mut compiled_string = String::new();

    if strs.len() == 0 {
        return compiled_string;
    }

    if strs.len() == 1 {
        return strs[0].to_string();
    }

    let mut blocks = vec![vec![String::new()]; strs.len()];
    let mut max_widths: Vec<usize> = vec![0; strs.len()];
    let mut max_height = 0;

    for (i, str) in strs.iter().enumerate() {
        let (lines, width) = get_lines(str);
        max_widths[i] = width;
        blocks[i] = lines.map(|s| s.to_string()).collect();
        if blocks[i].len() > max_height {
            max_height = blocks[i].len();
        }
    }

    for (i, block) in blocks.clone().iter().enumerate() {
        if block.len() > max_height {
            continue;
        }
        let extra_lines = vec![String::new(); max_height - block.len()];
        match pos {
            Position::Top => {
                blocks[i].extend(extra_lines);
            }
            Position::Bottom => {
                // cloned returns the owned values. The rev returns the references after reversing.
                blocks[i].extend(extra_lines.iter().cloned().rev());
            }
            _ => {
                // Split in the middle and add it to the top and bottom.
                let (mut top, mut bottom) = (
                    vec![String::new(); (max_height - block.len()) / 2],
                    vec![String::new(); (max_height - block.len()) / 2],
                );
                top = extra_lines.clone()[0..top.len()].to_vec();
                bottom = extra_lines.clone()[bottom.len()..].to_vec();
                blocks[i].extend(top);
                blocks[i].extend(bottom.iter().cloned().rev());
            }
        }
    }

    // Iterate through the lines and combine each line from the block and make it a single line to display it
    // horizontally.
    for i in 0..blocks[0].len() {
        for block in &blocks {
            compiled_string.push_str(&block[i]);
        }
        if i < blocks[0].len() - 1 {
            compiled_string.push('\n');
        }
    }

    compiled_string
}
