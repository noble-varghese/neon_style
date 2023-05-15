use std::println;

use neon_style::style::Style;
use neon_style::{
    rounded_border, with_whitespace_chars, with_whitespace_fg, Hue, Position,
};

fn main() {
    // let strs = "Lorem Ipsum is simply dummy text of the printing and typesetting industry.\nLorem Ipsum has been the industry's standard dummy text ever since the 1500s, \nwhen an unknown printer took a galley of type and scrambled it to make a type specimen book. \nIt has survived not only five centuries, but also the leap into electronic typesetting, \nremaining essentially unchanged. It was popularised in the 1960s with the release of Letraset\nsheets containing Lorem Ipsum passages, and more recently with desktop publishing software \nlike Aldus PageMaker including versions of Lorem Ipsum"
    //     .to_string();

    let _tab_border = neon_style::Border {
        top: "─".into(),
        bottom: "─".into(),
        left: "│".into(),
        right: "│".into(),
        top_left: "╭".into(),
        top_right: "╮".into(),
        bottom_left: "┴".into(),
        bottom_right: "┴".into(),
    };

    // let style = Style::new_style()
    //     .bold(true)
    //     .underline(true)
    //     .underline_spaces(false)
    //     .padding(&[1, 2])
    //     .background(Hue::from("#874BFD"))
    //     .border(tab_border, &[true])
    //     .margin(&[4])
    //     // .border_foreground(&[Hue::from("#FF0000")])
    //     .text_color(Hue::from("#FF0000"));

    // println!("{}", style.render(strs));

    let dialogue_box = Style::new_style()
        .border(rounded_border(), &[true])
        .border_foreground(&[Hue::from("#874BFD")])
        .padding(&[1, 0]);
    // println!("{}", dialogue_box.render("Noble".into()));

    let button_style = Style::new_style()
        .foreground(Hue::from("#FFF7DB"))
        .background(Hue::from("#F25D94"))
        .padding(&[0, 3])
        .margin_top(1);

    let active_button_style = button_style
        .copy()
        .foreground(Hue::from("#FFF7DB"))
        .background(Hue::from("#888B7E"))
        .margin_right(2)
        .underline(true);

    let question = Style::new_style()
        .width(50)
        .text_color(Hue::from("#FF0000"))
        .align(&[Position::Center])
        .render("Are you sure you want to exit ?".into());

    // println!("{}", noble);

    let ok_button = active_button_style.render("Yes".into());
    let cancel_button = button_style.render("Cancel".into());

    let buttons = neon_style::join_horizontally(Position::Top, &[ok_button, cancel_button]);

    let ui = neon_style::join_vertically(Position::Center, &[question, buttons]);

    let d = neon_style::place(
        96,
        9,
        Position::Center,
        Position::Center,
        dialogue_box.render(ui),
        &[
            with_whitespace_chars("猫咪".into()),
            with_whitespace_fg(Hue::from("#383838")),
        ],
    );
    println!("{}", d);
}
