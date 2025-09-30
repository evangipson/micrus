use crate::{input::keyboard, print, println, system::modules};

/// starts a shell session.
pub fn start_shell() {
    modules::FILE_SYSTEM.lock().write_current_dir();
    print!(" > ");
    loop {
        let char = keyboard::read_char();
        match char {
            '\n' => {
                // TODO: evaluate shell command
                println!("");
                println!("unknown command.");
                modules::FILE_SYSTEM.lock().write_current_dir();
                print!(" > ");
            }
            _ => {
                print!("{char}");
            }
        }
    }
}
