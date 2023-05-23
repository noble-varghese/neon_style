# Neon-style
Style definitions for nice terminal layouts in rust applications. Built with TUIs in mind.

To render the text like rendering CSS on the front-end. This one example.

```rust
use neon_style::style::Style;
use neon_style::Hue;

fn main() {
    let strs = "Lorem Ipsum is simply dummy text of the printing and typesetting industry.\\nLorem Ipsum has been the industry's"
        .to_string();

    let style = Style::new_style()
        .bold(true)
        .underline(true)
        .underline_spaces(false)
        .padding(&[1, 2])
        .background(Hue::from("#874BFD"))
        .margin(&[4])
        .text_color(Hue::from("#FF0000"));
    println!("{}", style.render(strs));
}
```

### **Inline formatting:**

Formatting of the strings done in the same line. Things like bold, italics, strikethrough is part of inline styling.

```rust
let inline_style = Style::new_style()
        .bold(true)
        .italic(true)
        .faint(true)
        .blink(true)
        .strikethrough(true)
        .underline(true)
        .reverse(true);let padding_style = Style::new_style()
        .padding_top(3)
        .padding_bottom(3)
        .padding_left(5)
        .padding_right(5);
```

### **Block level formatting**

Things includes things like padding and margins..

```rust
// Padding
let padding_style = Style::new_style()
        .padding_top(3)
        .padding_bottom(3)
        .padding_left(5)
        .padding_right(5);

// margin
let margin_style = Style::new_style()
        .margin_top(3)
        .margin_bottom(3)
        .margin_left(5)
        .margin_right(5);
```

Shorthand form for block level formatting.

```rust
// Same padding on all sides.
let padding_style = Style::new_style().padding(&[3]);

// Padding for top and bottom
let padding_style = Style::new_style().padding(&[3, 5]);

// Padding on top, sides and bottom. Padidng is top=3, left & right = 4, bottom=5
let padding_style = Style::new_style().padding(&[3, 4, 5]);

// Padding for all sides in the order -> Top, right, bottom, left
let padding_style = Style::new_style().padding(&[3, 3, 4, 4]);

// Same margin on all sides. Margin=5
let margin_style = Style::new_style().margin(&[5]);

// Setting top and bottom margin. Top=3 & bottom=5.
let margin_style = Style::new_style().margin(&[3, 5]);

//Setting top, sides and bottom. Top=3, right & left=6, bottom=5.
let margin_style = Style::new_style().margin(&[3, 6, 5]);

// Margin for all sides in the order -> Top, right, bottom, left.
let margin_style = Style::new_style().margin(&[3, 3, 4, 4]);
```

### **Text alignment**

```rust
use neon_style::{Position};

let margin_style = Style::new_style()
        .align(&[Position::Center]) // Align horizontally center.
        .align(&[Position::Left]) // Align horizontally left.
        .align(&[Position::Right]); // Align horizontally right. This is the final selection.

// In order to set the horizontal and vertical alignment together.
// Horizontally center and and vertically left aligned.
let margin_style = Style::new_style().align(&[Position::Center, Position::Left]);
```

### **Text width and height**

User can set the min height and width using the below format.

```rust
use neon_style::Hue

let s = Style::new_style()
        .width(40) // Setting the width
        .height(5) // Setting the height
        .foreground(Hue::from("#F25D94")) // Setting color using true color.
        .set_string("Hello World!");

println!("{}", s.to_string());
```

To set the max height and width the user may use the following. NOTE: While setting the max height and width the string may be truncated if it goes beyond the specified width or height.

```rust
use neon_style::Hue

let s = Style::new_style()
        .max_width(40) // Setting the max width
        .max_height(5) // Setting the max height
        .foreground(Hue::from("#F25D94")) // Setting color using true color.
        .set_string("Hello World!");

println!("{}", s.to_string());
```

### **Setting the borders**
