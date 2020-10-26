use crate::gdrive::FileInfo;
use percent_encoding::{utf8_percent_encode, NON_ALPHANUMERIC};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Index {
    pub files: Option<Vec<FileEntry>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub directories: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub success: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub referrer: Option<String>,
    #[serde(
        rename(deserialize = "googleApiKey"),
        skip_serializing_if = "Option::is_none"
    )]
    pub google_api_key: Option<String>,
    #[serde(
        rename(deserialize = "oneFichierKeys"),
        skip_serializing_if = "Option::is_none"
    )]
    pub one_fichier_keys: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub headers: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version: Option<String>,
    #[serde(
        rename(deserialize = "clientCertPub"),
        skip_serializing_if = "Option::is_none"
    )]
    pub client_cert_pub: Option<String>,
    #[serde(
        rename(deserialize = "clientCertKey"),
        skip_serializing_if = "Option::is_none"
    )]
    pub client_cert_key: Option<String>,
    #[serde(
        rename(deserialize = "themeBlackList"),
        skip_serializing_if = "Option::is_none"
    )]
    pub theme_blacklist: Option<Vec<String>>,
    #[serde(
        rename(deserialize = "themeWhiteList"),
        skip_serializing_if = "Option::is_none"
    )]
    pub theme_whitelist: Option<Vec<String>>,
    #[serde(
        rename(deserialize = "themeError"),
        skip_serializing_if = "Option::is_none"
    )]
    pub theme_error: Option<String>,
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
