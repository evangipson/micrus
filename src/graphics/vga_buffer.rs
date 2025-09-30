use crate::graphics::{
    color::Color,
    constants::{VGA_BUFFER_ADDRESS, VGA_HEIGHT, VGA_WIDTH},
    screen_char::ScreenChar,
};
use core::ptr::NonNull;
use volatile::VolatilePtr;

#[derive(Default)]
pub struct VgaBuffer {
    buffer: *mut ScreenChar,
    row: usize,
    column: usize,
}

impl VgaBuffer {
    pub fn new() -> VgaBuffer {
        VgaBuffer {
            buffer: VGA_BUFFER_ADDRESS as *mut ScreenChar,
            row: 0,
            column: 0,
        }
    }

    unsafe fn write_byte(&mut self, byte: u8, foreground: Color, background: Color) {
        let color_code = (background as u8) << 4 | (foreground as u8);
        let char_to_write = ScreenChar {
            ascii_character: byte,
            color_code,
        };

        match byte {
            b'\n' => self.new_line(),
            _ => {
                if self.column >= VGA_WIDTH {
                    self.new_line();
                }
                let offset = self.row * VGA_WIDTH + self.column;
                unsafe {
                    let char_ptr = self.buffer.add(offset);
                    // Convert raw pointer to NonNull, then create VolatilePtr, then write
                    VolatilePtr::new(NonNull::new(char_ptr).unwrap()).write(char_to_write); // <-- KEY CHANGE
                }
                self.column += 1;
            }
        }
    }

    fn new_line(&mut self) {
        self.column = 0;
        self.row += 1;
        if self.row >= VGA_HEIGHT {
            self.row = 0;
            self.clear_screen();
        }
    }

    pub fn clear_screen(&mut self) {
        for row in 0..VGA_HEIGHT {
            for col in 0..VGA_WIDTH {
                let offset = row * VGA_WIDTH + col;
                unsafe {
                    let char_ptr = self.buffer.add(offset);
                    // convert raw pointer to NonNull, then create VolatilePtr, then write
                    VolatilePtr::new(NonNull::new(char_ptr).unwrap()).write(ScreenChar::new(
                        b' ',
                        Color::Black,
                        Color::Black,
                    ));
                }
            }
        }
    }

    pub fn write_string(&mut self, s: &str, foreground: Color, background: Color) {
        for byte in s.bytes() {
            match byte {
                0x20..=0x7e | b'\n' => unsafe {
                    self.write_byte(byte, foreground, background);
                },
                _ => unsafe {
                    self.write_byte(0xfe, foreground, background);
                },
            }
        }
    }
}

unsafe impl Send for VgaBuffer {}
unsafe impl Sync for VgaBuffer {}

impl core::fmt::Write for VgaBuffer {
    fn write_str(&mut self, s: &str) -> core::fmt::Result {
        self.write_string(s, Color::White, Color::Black);
        Ok(())
    }
}
