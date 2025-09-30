use crate::graphics::color::Color;

#[repr(C)]
#[derive(Clone, Copy)]
pub struct ScreenChar {
    pub ascii_character: u8,
    pub color_code: u8,
}

impl ScreenChar {
    pub fn new(ascii_character: u8, foreground: Color, background: Color) -> ScreenChar {
        ScreenChar {
            ascii_character,
            color_code: (background as u8) << 4 | (foreground as u8),
        }
    }
}
