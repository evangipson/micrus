#![no_std]
#![no_main]

use micrus::{interrupts::descriptor_table, messaging::boot};

/// `_start` is what the bootloader uses to load `micrus`. This function is
/// the entry point, since the linker looks for a function named `_start` by
/// default.
#[unsafe(no_mangle)]
pub extern "C" fn _start() -> ! {
    descriptor_table::init_idt();

    boot::display_welcome_message();
    loop {
        boot::display_module_selection();
    }
}
