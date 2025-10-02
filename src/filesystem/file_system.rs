use crate::filesystem::{constants, directory::Directory};
use heapless::{String, Vec};

#[derive(Default, Debug)]
pub struct FileSystem<'fs> {
    pub root: Directory<'fs>,
    pub directories: Vec<Directory<'fs>, { constants::MAX_DIRECTORIES }>,
    pub current_dir: String<{ constants::MAX_PATH_LENGTH }>,
}

impl FileSystem<'_> {
    pub fn go_to_root(&mut self) {
        self.current_dir = self.root.full_path.clone();
    }

    pub fn change_dir(&mut self, new_dir: &str) {
        if let Some(directory) = self.directories.iter().find(|dir| dir.name.eq(new_dir)) {
            self.current_dir = directory.full_path.clone();
        }
    }

    pub fn new() -> Self {
        // create the root directory
        let root = Directory::new(constants::PATH_SEPARATOR, "");

        // add the foundational directories off of root
        let mut directories = Vec::new();
        let user_dir = Directory::new("user", constants::PATH_SEPARATOR);
        directories
            .push(user_dir)
            .expect("Failed to initialize user directory");

        Self {
            root,
            directories,
            current_dir: String::try_from(constants::PATH_SEPARATOR)
                .expect("Could not create root as current directory"),
        }
    }

    pub fn add_directory(&mut self, new_dir_name: &'static str, parent_dir_name: &'static str) {
        let new_dir = Directory::new(new_dir_name, parent_dir_name);
        self.directories
            .push(new_dir)
            .expect("Max directory limit reached! Cannot add more directories.");
    }

    pub fn get_current_dir(&self) -> &str {
        &self.current_dir
    }
}
