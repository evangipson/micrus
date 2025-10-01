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
        let new_dir = command
            .split(" ")
            .nth(1)
            .unwrap_or(constants::PATH_SEPARATOR);
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
    let mut char_count: u32 = 0;
    write_prompt();
    loop {
        let char = keyboard::read_char();
        match char {
            '\n' => {
                evaluate_command(command.as_str());
                write_prompt();
            }
            '\x08' => {
                print!("\x08");
                command.remove(char_count as usize);
                char_count -= 1;
            }
            ' '..='~' => {
                print!("{char}");
                command.push(char).expect("argument too long for shell.");
                char_count += 1;
            }
            _ => {}
        }
    }
}
