use neon_style::style::Style;

fn main() {
    let mut strs = "
Lorem Ipsum is simply dummy text of the printing and typesetting industry.
Lorem Ipsum has been the industry's standard dummy text ever since the 1500s, 
when an unknown printer took a galley of type and scrambled it to make a type specimen book. 
It has survived not only five centuries, but also the leap into electronic typesetting, 
remaining essentially unchanged. It was popularised in the 1960s with the release of Letraset
sheets containing Lorem Ipsum passages, and more recently with desktop publishing software 
like Aldus PageMaker including versions of Lorem Ipsum"
        .to_string();
    let mut style = Style::new_style()
        .bold(true)
        .underline(true)
        .underline_spaces(false)
        .padding(&[1, 1])
        .faint(true)
        .border(neon_style::normal_border(), &[true, true, true, true]);
    println!("Yes this is the output - \n{}", style.render(&mut strs));
}
