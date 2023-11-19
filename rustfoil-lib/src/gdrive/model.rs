use crate::abstraction::file::TinfoilFileLike;
use crate::tinfoil::encoding::FRAGMENT;
use percent_encoding::utf8_percent_encode;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone)]
pub struct GoogleDriveFileInfo {
    pub id: String,
    pub size: i64,
    pub name: String,
    pub shared: bool,
}

#[derive(Debug)]
pub struct GoogleDriveFolderInfo {
    pub id: String,
    pub shared: bool,
}

#[derive(Debug)]
pub struct GoogleDriveScanResult {
    pub files: Vec<GoogleDriveFileInfo>,
    pub folders: Vec<GoogleDriveFolderInfo>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GoogleDriveTokenInfo {
    pub token: GoogleDriveToken,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GoogleDriveToken {
    pub access_token: String,
    pub refresh_token: String,
}

impl GoogleDriveScanResult {
    pub fn new(
        files: Vec<GoogleDriveFileInfo>,
        folders: Vec<GoogleDriveFolderInfo>,
    ) -> GoogleDriveScanResult {
        GoogleDriveScanResult { files, folders }
    }
}

impl GoogleDriveFolderInfo {
    pub fn new(id: String, shared: bool) -> GoogleDriveFolderInfo {
        GoogleDriveFolderInfo { id, shared }
    }
}

impl GoogleDriveFileInfo {
    pub fn new(id: String, size: i64, name: String, shared: bool) -> GoogleDriveFileInfo {
        GoogleDriveFileInfo {
            id,
            size,
            name,
            shared,
        }
    }
}

impl TinfoilFileLike for GoogleDriveFileInfo {
    fn get_url(&self) -> String {
        format!("gdrive:{}#{}", &self.id, &self.get_name_encoded())
    }

    fn get_size(&self) -> i64 {
        self.size
    }

    fn get_name(&self) -> &str {
        &self.name
    }
}

impl GoogleDriveFileInfo {
    pub fn get_name_encoded(&self) -> String {
        utf8_percent_encode(&self.name, FRAGMENT).to_string()
    }
}
