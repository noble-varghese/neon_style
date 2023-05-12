use neon_style::style::Style;

fn main() {
    let strs = "    
    Lorem Ipsum is simply dummy text of the printing and typesetting industry.\nLorem Ipsum has been the industry's standard dummy text ever since the 1500s, \nwhen an unknown printer took a galley of type and scrambled it to make a type specimen book. \nIt has survived not only five centuries, but also the leap into electronic typesetting, \nremaining essentially unchanged. It was popularised in the 1960s with the release of Letraset\nsheets containing Lorem Ipsum passages, and more recently with desktop publishing software \nlike Aldus PageMaker including versions of Lorem Ipsum"
        .to_string();

    let active_border = neon_style::Border {
        top: String::from("─"),
        bottom: String::from(" "),
        left: String::from("│"),
        right: String::from("│"),
        top_left: String::from("╭"),
        top_right: String::from("╮"),
        bottom_left: String::from("┘"),
        bottom_right: String::from("└"),
    };

    let mut style = Style::new_style()
        .bold(true)
        .underline(true)
        .underline_spaces(false)
        .padding(&[1])
        .faint(true)
        .border(active_border, &[true, true, true, true]);
    println!("Yes this is the output - \n{}", style.render(&strs));
}
