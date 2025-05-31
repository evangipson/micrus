#![no_std]
#![no_main]

use micrus::input::keyboard;
use micrus::{print, println};

/// `_start` is what the bootloader uses to load `micrus`. This function is
/// the entry point, since the linker looks for a function named `_start` by
/// default.
#[unsafe(no_mangle)]
pub extern "C" fn _start() -> ! {
    println!("            _                     ");
    println!("  _ __ ___ (_) ___ _ __ _   _ ___ ");
    println!(r" | '_ ` _ \| |/ __| '__| | | / __|");
    println!(r" | | | | | | | (__| |  | |_| \__ \");
    println!(r" |_| |_| |_|_|\___|_|   \__,_|___/");
    println!("");
    println!("micrus microkernel loader version 0.0.1");
    println!("");
    loop {
        println!("Select a module to load:");
        println!("");
        println!("  1: Filesystem");
        println!("  2: Network Stack");
        println!("  3: Shell");
        println!("  q: Quit");
        print!("> ");

        // TODO: implement read_char_from_keyboard
        let choice = keyboard::read_char();
        match choice {
            '1' => {
                println!("Loading Basic Filesystem...");
                // load and initialize the filesystem module
                // e.g.: `load_module("filesystem");`
            }
            '2' => println!("Network Stack not yet implemented!"),
            '3' => println!("Shell not yet implemented!"),
            'Q' | 'q' => {
                println!("Shutting down...");
                // TODO: implement safe shutdown, as this will just halt the
                // CPU until the next interrupt
                x86_64::instructions::hlt();
            }
            _ => println!("Invalid choice."),
        }
    }
}
