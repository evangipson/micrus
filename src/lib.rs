#![no_std]
#![feature(abi_x86_interrupt)]

pub mod filesystem {
    pub mod constants;
    pub mod directory;
    pub mod file;
    pub mod file_system;
}

pub mod graphics {
    pub mod color;
    pub mod constants;
    pub mod screen_char;
    pub mod vga_buffer;
}

pub mod input {
    pub mod keyboard;
}

pub mod interrupts {
    pub mod constants;
    pub mod descriptor_table;
    pub mod fault_handlers;
    pub mod panic;
    pub mod shutdown;
}

pub mod messaging {
    pub mod boot;
    pub mod print;
}

pub mod system {
    pub mod modules;
    pub mod shell;
}
