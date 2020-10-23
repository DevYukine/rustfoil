use crate::gdrive::FileInfo;
use percent_encoding::{utf8_percent_encode, NON_ALPHANUMERIC};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Index {
    pub files: Vec<FileEntry>,
    #[serde(skip_serializing_if = "String::is_empty")]
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

pub struct ParsedFileInfo {
    pub id: String,
    pub size: String,
    pub name: String,
    pub name_encoded: String,
    pub shared: bool,
}

impl ParsedFileInfo {
    pub fn new(info: FileInfo) -> ParsedFileInfo {
        let name_encoded = utf8_percent_encode(&*info.name.clone(), NON_ALPHANUMERIC).to_string();
        ParsedFileInfo {
            id: info.id,
            size: info.size,
            name: info.name,
            name_encoded,
            shared: info.shared,
        }
    }
}
