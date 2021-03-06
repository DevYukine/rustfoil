use crate::gdrive::FileInfo;
use percent_encoding::{utf8_percent_encode, AsciiSet, CONTROLS};
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

/// https://url.spec.whatwg.org/#fragment-percent-encode-set & Brackets
const FRAGMENT: &AsciiSet = &CONTROLS
    .add(b' ')
    .add(b'"')
    .add(b'<')
    .add(b'>')
    .add(b'`')
    .add(b'[')
    .add(b']');

#[skip_serializing_none]
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Index {
    pub files: Option<Vec<FileEntry>>,
    pub directories: Option<Vec<String>>,
    pub success: Option<String>,
    pub referrer: Option<String>,
    pub google_api_key: Option<String>,
    pub one_fichier_keys: Option<Vec<String>>,
    pub headers: Option<Vec<String>>,
    pub version: Option<f64>,
    pub client_cert_pub: Option<String>,
    pub client_cert_key: Option<String>,
    #[serde(rename = "themeBlackList")]
    pub theme_blacklist: Option<Vec<String>>,
    #[serde(rename = "themeWhiteList")]
    pub theme_whitelist: Option<Vec<String>>,
    pub theme_error: Option<String>,
    pub locations: Option<Vec<Location>>,
}

#[derive(Serialize, Deserialize)]
#[serde(untagged)]
pub enum Location {
    String(String),
    Object(LocationObject),
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum LocationAction {
    Disable,
    Enable,
    Add,
}

#[derive(Serialize, Deserialize)]
pub struct LocationObject {
    pub url: String,
    pub title: String,
    pub action: LocationAction,
}

#[derive(Serialize, Deserialize)]
pub struct FileEntry {
    url: String,
    size: u64,
}

impl Index {
    pub fn new() -> Index {
        Index {
            files: None,
            directories: None,
            success: None,
            referrer: None,
            google_api_key: None,
            one_fichier_keys: None,
            headers: None,
            version: None,
            client_cert_pub: None,
            client_cert_key: None,
            theme_blacklist: None,
            theme_whitelist: None,
            theme_error: None,
            locations: None,
        }
    }
}

impl FileEntry {
    pub fn new(url: String, size: u64) -> FileEntry {
        FileEntry { url, size }
    }
}

#[derive(Clone)]
pub struct ParsedFileInfo {
    pub id: String,
    pub size: String,
    pub name: String,
    pub name_encoded: String,
    pub shared: bool,
}

impl ParsedFileInfo {
    pub fn new(info: FileInfo) -> ParsedFileInfo {
        ParsedFileInfo {
            name_encoded: utf8_percent_encode(info.name.as_str(), FRAGMENT).to_string(),
            id: info.id,
            size: info.size,
            name: info.name,
            shared: info.shared,
        }
    }
}
