use lazy_static::lazy_static;
use spin::Mutex;

use crate::filesystem::file_system::FileSystem;

lazy_static! {
    pub static ref FILE_SYSTEM_ADDED: Mutex<bool> = Mutex::new(false);
    pub static ref FILE_SYSTEM: Mutex<FileSystem<'static>> = Mutex::new(FileSystem::default());
}
