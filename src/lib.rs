#![no_std]
#![no_main]

pub mod graphics {
    pub mod vga;
}

pub mod input {
    pub mod keyboard;
}

pub mod interrupts {
    pub mod panic;
}
