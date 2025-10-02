use crate::{
    filesystem::constants,
    input::keyboard,
    print, println,
    system::modules::{self, file_system_added},
};
use heapless::String;

/// writes the directory for the shell with user prompt.
fn write_prompt() {
    if file_system_added() {
        print!("{} ", modules::FILE_SYSTEM.lock().get_current_dir());
    }
    print!("> ");
}

fn evaluate_command(command: &str) {
    if command.starts_with("cd") {
        let mut new_dir_raw = command.split_whitespace();
        let new_dir = new_dir_raw.nth(1).unwrap_or(constants::PATH_SEPARATOR);
        println!("");
        println!("should change directory to {new_dir}");
        modules::FILE_SYSTEM.lock().change_dir(new_dir);
        println!("");
        println!(
            "changed directory to {}",
            modules::FILE_SYSTEM.lock().get_current_dir()
        );
    } else {
        println!("");
        println!("unknown command.");
    }
}

/// starts a shell session.
pub fn start_shell() {
    let mut command: String<256> = String::new();
    write_prompt();
    loop {
        let char = keyboard::read_char();
        match char {
            // handle enter
            '\n' => {
                // evaluate, then clear, then re-prompt
                evaluate_command(command.as_str());
                command.clear();
                write_prompt();
            }
            // handle backspace
            '\x08' => {
                // don't allow removal of the prompt
                if !command.is_empty() {
                    print!("\x08 \x08");
                    command.pop();
                }
            }
            // handle all valid characters
            ' '..='~' => {
                print!("{char}");
                command.push(char).expect("argument too long for shell.");
            }
            _ => {}
        }
    }
}
