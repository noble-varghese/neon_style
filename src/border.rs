#[derive(Clone, Default, PartialEq)]
pub struct Border {
    pub top: String,
    pub bottom: String,
    pub left: String,
    pub right: String,
    pub top_left: String,
    pub top_right: String,
    pub bottom_right: String,
    pub bottom_left: String,
}

impl Border {
    pub fn new() -> Self {
        Self {
            top: String::new(),
            bottom: String::new(),
            left: String::new(),
            right: String::new(),
            top_left: String::new(),
            top_right: String::new(),
            bottom_right: String::new(),
            bottom_left: String::new(),
        }
    }
}
