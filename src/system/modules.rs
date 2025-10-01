use lazy_static::lazy_static;
use spin::Mutex;

use crate::filesystem::{constants, file_system::FileSystem};

lazy_static! {
    pub static ref FILE_SYSTEM_ADDED: Mutex<bool> = Mutex::new(false);
    pub static ref FILE_SYSTEM: Mutex<FileSystem<'static>> = Mutex::new(FileSystem::new());
}

/// checks if the [`FileSystem`] module is added into [`micrus`](crate).
pub fn file_system_added() -> bool {
    FILE_SYSTEM_ADDED.lock().eq(&true)
}

/// adds the [`FileSystem`] module to [`micrus`](crate), creates a "/user"
/// directory, and changes the current directory to root.
pub fn add_file_system() {
    if !file_system_added() {
        *FILE_SYSTEM_ADDED.lock() = true;

        // add the /user directory under root
        FILE_SYSTEM
            .lock()
            .add_directory("user", constants::PATH_SEPARATOR);

        // navigate to the root directory
        FILE_SYSTEM.lock().go_to_root();
    }
}
