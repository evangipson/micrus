use crate::{input::keyboard, print, println};

pub fn display_welcome_message() {
    println!("            _                     ");
    println!("  _ __ ___ (_) ___ _ __ _   _ ___ ");
    println!(r" | '_ ` _ \| |/ __| '__| | | / __|");
    println!(r" | | | | | | | (__| |  | |_| \__ \");
    println!(r" |_| |_| |_|_|\___|_|   \__,_|___/");
    println!("");
    println!("micrus microkernel loader version 0.0.1");
    println!("");
}

pub fn display_module_selection() {
    println!("Select a module to load:");
    println!("");
    println!("  1: Filesystem");
    println!("  2: Network Stack");
    println!("  3: Shell");
    println!("  4: Cause Breakpoint Interrupt");
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
        '4' => x86_64::instructions::interrupts::int3(),
        'Q' | 'q' => {
            println!("Shutting down...");
            // TODO: implement safe shutdown, as this will just halt the
            // CPU until the next interrupt
            x86_64::instructions::hlt();
        }
        _ => println!("Invalid choice."),
    }
}
