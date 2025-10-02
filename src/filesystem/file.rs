#[derive(Debug)]
pub struct File {
    pub name: &'static str,
    pub contents: Option<&'static str>,
}

impl File {
    pub fn new_file(name: &'static str) -> Self {
        Self {
            name,
            contents: None,
        }
    }
}
