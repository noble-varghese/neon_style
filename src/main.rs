use neon_style::style::Style;

fn main() {
    let mut strs = "Hello World".to_string();
    let style = Style::new_style().bold(true).render(&mut strs);
    println!("Yes this is the output - {}", style);
}
