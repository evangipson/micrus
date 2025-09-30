use crate::filesystem::{constants, directory::Directory};

#[derive(Default)]
pub struct FileSystem<'fs> {
    pub root: Directory<'fs>,
    pub directories: &'fs [Directory<'fs>],
    pub current_dir: &'fs str,
}

impl FileSystem<'_> {
    pub fn go_to_root(&mut self) {
        self.current_dir = self.root.full_path
    }

    pub fn change_dir(&'static mut self, new_dir: &'static str) {
        if let Some(directory) = self.directories.iter().find(|dir| dir.name.eq(new_dir)) {
            self.current_dir = directory.full_path;
        }
    }

    pub fn new(&self) -> Self {
        let root = Directory::new(constants::PATH_SEPARATOR, "");
        Self {
            root,
            directories: &[],
            current_dir: "",
        }
    }

    pub fn write_current_dir(&self) -> &str {
        self.current_dir
    }
}
