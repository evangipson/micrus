#![no_std]
#![no_main]
#![feature(abi_x86_interrupt)]

pub mod graphics {
    pub mod vga;
}

pub mod input {
    pub mod keyboard;
}

pub mod interrupts {
    pub mod descriptor_table;
    pub mod panic;
}

pub mod messaging {
    pub mod boot;
}
