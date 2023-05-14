use neon_style::style::Style;
use neon_style::{rounded_border, Color as neon_color, Position};

fn main() {
    let strs = "Lorem Ipsum is simply dummy text of the printing and typesetting industry.\nLorem Ipsum has been the industry's standard dummy text ever since the 1500s, \nwhen an unknown printer took a galley of type and scrambled it to make a type specimen book. \nIt has survived not only five centuries, but also the leap into electronic typesetting, \nremaining essentially unchanged. It was popularised in the 1960s with the release of Letraset\nsheets containing Lorem Ipsum passages, and more recently with desktop publishing software \nlike Aldus PageMaker including versions of Lorem Ipsum"
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

    let style = Style::new_style()
        .bold(true)
        .underline(true)
        .underline_spaces(false)
        .padding(&[1, 2])
        .background(neon_color::from("#874BFD"))
        .border(tab_border, &[])
        .border_left(true)
        .border_bottom(true)
        .margin(&[4])
        // .border_foreground(&[neon_color::from("#FF0000")])
        .text_color(neon_color::from("#FF0000"));

    println!("{}", style.render(strs));
    let dialogue_box = Style::new_style()
        .border(rounded_border(), &[true])
        .border_foreground(&[neon_color::from("#874BFD")])
        .padding(&[1, 0]);

    let button_style = Style::new_style()
        .foreground(neon_color::from("#FFF7DB"))
        .background(neon_color::from("#888B7E"))
        .margin_right(2)
        .underline(true);

    let active_button_style = Style::new_style()
        .foreground(neon_color::from("#FFF7DB"))
        .background(neon_color::from("#F25D94"))
        .margin_right(2)
        .underline(true);

    let question = Style::new_style()
        .width(50)
        .text_color(neon_color::from("#FF0000"))
        .align(&[Position::Center])
        .render("Are you sure you want to eat marmalade?".into());

    // let ok_button = active_button_style.render("OK".into());
    // let cancel_button = button_style.render("Cancel".into());

    // let buttons = neon_style::join_horizontally(Position::Top, &[ok_button, cancel_button]);
    // // println!("{buttons}");

    // let ui = neon_style::join_vertically(Position::Center, &[question, buttons]);
    // println!("{}", ui);
    let questionair = Style::new_style()
        .border(rounded_border(), &[true])
        .border_foreground(&[neon_color::from("#874BFD")])
        .padding(&[1, 0])
        .render(question);
    println!("{}", questionair.to_string());
}
