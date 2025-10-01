use crate::filesystem::{constants, directory::Directory};

#[derive(Default)]
pub struct FileSystem<'fs> {
    pub root: Directory<'fs>,
    pub directories: &'fs [Directory<'fs>],
    pub current_dir: &'fs str,
    pub directory_count: usize,
}

impl FileSystem<'_> {
    pub fn go_to_root(&mut self) {
        self.current_dir = self.root.name
    }

    pub fn change_dir(&mut self, new_dir: &str) {
        if let Some(directory) = self.directories.iter().find(|dir| dir.name.eq(new_dir)) {
            self.current_dir = &directory.full_path;
        }
    }

    pub fn new() -> Self {
        let root = Directory::new(constants::PATH_SEPARATOR, "");
        Self {
            root,
            directories: &[],
            current_dir: "",
            directory_count: 0,
        }
    }

    pub fn add_directory(&mut self, new_dir: &'static str, parent_dir: &'static str) {
        let new_dir = Directory::new(new_dir, parent_dir);
        self.directories.get(self.directory_count).replace(&new_dir);
        self.directory_count += 1;
    }

    pub fn get_current_dir(&self) -> &str {
        self.current_dir
    }
}
