use neon_style::style::Style;
use neon_style::Color as neon_color;

fn main() {
    let strs = "    
    Lorem Ipsum is simply dummy text of the printing and typesetting industry.\nLorem Ipsum has been the industry's standard dummy text ever since the 1500s, \nwhen an unknown printer took a galley of type and scrambled it to make a type specimen book. \nIt has survived not only five centuries, but also the leap into electronic typesetting, \nremaining essentially unchanged. It was popularised in the 1960s with the release of Letraset\nsheets containing Lorem Ipsum passages, and more recently with desktop publishing software \nlike Aldus PageMaker including versions of Lorem Ipsum"
        .to_string();

    let tab_border = neon_style::Border {
        top: "─".into(),
        bottom: "─".into(),
        left: "│".into(),
        right: "│".into(),
        top_left: "╭".into(),
        top_right: "╮".into(),
        bottom_left: "┴".into(),
        bottom_right: "┴".into(),
    };

    let mut style = Style::new_style()
        .bold(true)
        .underline(true)
        .underline_spaces(false)
        .padding(&[1, 2])
        .background(neon_color::from("#874BFD"))
        .border(tab_border, &[true]);
    println!("{}", style.render(&strs));
}
