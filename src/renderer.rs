pub struct Renderer {
    pub color: Option<crossterm::style::Color>,
    pub attribute: Option<crossterm::style::Attribute>,
    pub has_dark_background: bool,
}

impl Renderer {
    pub fn new() -> Self {
        Self {
            has_dark_background: true,
            color: None,
            attribute: None,
        }
    }
}
