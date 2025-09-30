use lazy_static::lazy_static;
use spin::Mutex;

use crate::filesystem::file_system::FileSystem;

lazy_static! {
    pub static ref FILE_SYSTEM_ADDED: Mutex<bool> = Mutex::new(false);
    pub static ref FILE_SYSTEM: Mutex<FileSystem<'static>> = Mutex::new(FileSystem::default());
}

/// checks if the [`FileSystem`] module is added into [`micrus`](crate).
pub fn file_system_added() -> bool {
    FILE_SYSTEM_ADDED.lock().eq(&true)
}

/// adds the [`FileSystem`] module to [`micrus`](crate), and changes directory
/// to root.
pub fn add_file_system() {
    if !file_system_added() {
        *FILE_SYSTEM_ADDED.lock() = true;
        FILE_SYSTEM.lock().new();
        FILE_SYSTEM.lock().go_to_root();
    }
}
