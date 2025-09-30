use crate::filesystem::file::File;

#[derive(Default)]
pub struct Directory<'dir> {
    pub name: &'dir str,
    pub parent: &'dir str,
    pub full_path: &'dir str,
    pub files: &'dir [File],
}

impl Directory<'_> {
    pub fn new(name: &'static str, parent: &'static str) -> Self {
        Self {
            name,
            parent,
            full_path: stringify!("{}{}{}", parent, constants::PATH_SEPARATOR, name),
            files: &[],
        }
    }
}
