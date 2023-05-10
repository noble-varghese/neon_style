use crossterm::style::Color;

use crate::renderer::Renderer;

// pub trait TerminalColor {
//     fn color(&self, _renderer: &Renderer) -> Color;
//     fn rgb(&self) -> (u32, u32, u32, u32);
// }

#[derive(Clone, Copy)]
pub struct NoColor;

#[derive(Default)]
pub struct TerminalColor {
    color: Color,
}

// impl TerminalColor for NoColor {
//     fn color(&self, _renderer: &Renderer) -> Color {
//         Color::Reset
//     }

//     fn rgb(&self) -> (u32, u32, u32, u32) {
//         (0x0, 0x0, 0x0, 0xFFFF)
//     }
// }
