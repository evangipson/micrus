use crate::filesystem::constants;
use core::str;
use heapless::String;

#[derive(Default, Debug)]
pub struct Directory<'dir> {
    pub name: &'dir str,
    pub parent: &'dir str,
    pub full_path: String<{ constants::MAX_PATH_LENGTH }>,
}

impl Directory<'_> {
    fn build_path(segments: &[&str]) -> String<{ constants::MAX_PATH_LENGTH }> {
        // create a fixed-capacity string on the stack
        let mut path: String<{ constants::MAX_PATH_LENGTH }> = String::new();

        for (i, segment) in segments.iter().enumerate() {
            // append the directory segment
            if path.push_str(segment).is_err() {
                // return the path truncated up to the limit
                return path;
            }

            // append the separator (unless it's the last segment)
            if i < segments.len() - 1 && path.push('/').is_err() {
                // handle error: capacity exceeded
                return path;
            }
        }
        path
    }

    pub fn new(name: &'static str, parent: &'static str) -> Self {
        Self {
            name,
            parent,
            full_path: if name.eq(constants::PATH_SEPARATOR) {
                Self::build_path(&[constants::PATH_SEPARATOR])
            } else if parent.eq(constants::PATH_SEPARATOR) {
                Self::build_path(&[constants::PATH_SEPARATOR, name])
            } else {
                Self::build_path(&[parent, constants::PATH_SEPARATOR, name])
            },
        }
    }
}
