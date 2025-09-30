use crate::{
    clear,
    input::keyboard,
    interrupts::shutdown,
    print, println,
    system::{self, modules},
};

pub fn display_welcome_message() {
    println!("");
    println!(r"            _                     ");
    println!(r"  _ __ ___ (_) ___ _ __ _   _ ___ ");
    println!(r" | '_ ` _ \| |/ __| '__| | | / __|");
    println!(r" | | | | | | | (__| |  | |_| \__ \");
    println!(r" |_| |_| |_|_|\___|_|   \__,_|___/");
    println!("");
    println!("micrus microkernel loader version 0.0.1");
    println!("");
    println!("select module(s) to install:");
    println!("[1]: filesystem");
    println!("[2]: network");
    println!("[3]: fire interrupt");
    println!("[4]: boot kernel");
    println!("[Q]: quit");
    println!("");
    print!("> ");
}

pub fn display_module_selection() {
    let char = keyboard::read_char();
    match char {
        '1' => {
            println!("1");
            if modules::FILE_SYSTEM_ADDED.lock().eq(&true) {
                println!("already added file system.");
            } else {
                println!("adding file system...");
                *modules::FILE_SYSTEM_ADDED.lock() = true;
                modules::FILE_SYSTEM.lock().new();
                println!("file system added.");
            }
        }
        '2' => {
            println!("2");
            println!("network stack not yet implemented.")
        }
        '3' => {
            x86_64::instructions::interrupts::int3();
        }
        '4' => {
            println!("4");
            println!("booting micrus...");
            clear!();
            modules::FILE_SYSTEM.lock().go_to_root();
            system::shell::start_shell();
        }
        'Q' | 'q' => {
            if char == 'Q' {
                println!("Q");
            } else {
                println!("q");
            }
            println!("shutting down...");
            println!("");
            shutdown::shutdown();
        }
        _ => {
            println!("{char}");
            println!("invalid option.");
        }
    }

    println!("");
    print!("> ");
}
