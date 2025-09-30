use crate::{
    input::keyboard,
    print, println,
    system::modules::{self, file_system_added},
};

/// writes the directory for the shell with user prompt.
fn write_prompt() {
    if file_system_added() {
        print!(" ");
        modules::FILE_SYSTEM.lock().write_current_dir();
    }
    print!("> ");
}

/// starts a shell session.
pub fn start_shell() {
    write_prompt();
    loop {
        let char = keyboard::read_char();
        match char {
            '\n' => {
                // TODO: evaluate shell command
                println!("");
                println!("unknown command.");
                write_prompt();
            }
            _ => {
                print!("{char}");
            }
        }
    }
}
