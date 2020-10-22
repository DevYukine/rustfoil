use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Index {
    pub files: Vec<FileEntry>,
    pub success: String,
}

#[derive(Serialize, Deserialize)]
pub struct FileEntry {
    url: String,
    size: u64,
}

impl Index {
    pub fn new() -> Index {
        Index {
            files: Vec::new(),
            success: String::new(),
        }
    }
}

impl FileEntry {
    pub fn new(url: String, size: u64) -> FileEntry {
        FileEntry { url, size }
    }
}
