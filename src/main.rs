use neon_style::style::Style;

fn main() {
    let mut strs = "Hello World".to_string();
    let mut style = Style::new_style().bold(true);
    println!("Yes this is the output - {}", style.render(&mut strs));
}
