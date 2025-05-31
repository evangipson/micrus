use core::ptr::NonNull;
use volatile::VolatilePtr;

const VGA_BUFFER_ADDRESS: usize = 0xb8000;
const VGA_WIDTH: usize = 80;
const VGA_HEIGHT: usize = 25;

#[repr(u8)]
#[derive(Clone, Copy)]
pub enum Color {
    Black = 0x0,
    Blue = 0x1,
    Green = 0x2,
    Cyan = 0x3,
    Red = 0x4,
    Magenta = 0x5,
    Brown = 0x6,
    LightGray = 0x7,
    DarkGray = 0x8,
    LightBlue = 0x9,
    LightGreen = 0xA,
    LightCyan = 0xB,
    LightRed = 0xC,
    Pink = 0xD,
    Yellow = 0xE,
    White = 0xF,
}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct ScreenChar {
    ascii_character: u8,
    color_code: u8,
}

impl ScreenChar {
    pub fn new(ascii_character: u8, foreground: Color, background: Color) -> ScreenChar {
        ScreenChar {
            ascii_character,
            color_code: (background as u8) << 4 | (foreground as u8),
        }
    }
}

pub struct VgaBuffer {
    buffer: *mut ScreenChar,
    row: usize,
    column: usize,
}

impl VgaBuffer {
    fn new() -> VgaBuffer {
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

    fn clear_screen(&mut self) {
        for row in 0..VGA_HEIGHT {
            for col in 0..VGA_WIDTH {
                let offset = row * VGA_WIDTH + col;
                unsafe {
                    let char_ptr = self.buffer.add(offset);
                    // Convert raw pointer to NonNull, then create VolatilePtr, then write
                    VolatilePtr::new(NonNull::new(char_ptr).unwrap()).write(ScreenChar::new(
                        b' ',
                        Color::Black,
                        Color::Black,
                    )); // <-- KEY CHANGE
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

use lazy_static::lazy_static;
use spin::Mutex;

lazy_static! {
    pub static ref WRITER: Mutex<VgaBuffer> = Mutex::new(VgaBuffer::new());
}

#[macro_export]
macro_rules! print {
    ($($arg:tt)*) => ($crate::graphics::vga::_print(format_args!($($arg)*)));
}

#[macro_export]
macro_rules! println {
    () => ($crate::print!("\n"));
    ($($arg:tt)*) => ($crate::print!("{}\n", format_args!($($arg)*)));
}

#[doc(hidden)]
pub fn _print(args: core::fmt::Arguments) {
    use core::fmt::Write;
    use x86_64::instructions::interrupts;

    interrupts::without_interrupts(|| {
        WRITER.lock().write_fmt(args).unwrap();
    });
}

impl core::fmt::Write for VgaBuffer {
    fn write_str(&mut self, s: &str) -> core::fmt::Result {
        self.write_string(s, Color::White, Color::Black);
        Ok(())
    }
}
