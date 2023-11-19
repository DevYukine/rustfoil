use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

#[skip_serializing_none]
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TinfoilIndex {
    pub files: Option<Vec<TinfoilFile>>,
    pub directories: Option<Vec<String>>,
    pub success: Option<String>,
    pub referrer: Option<String>,
    pub google_api_key: Option<String>,
    pub one_fichier_keys: Option<Vec<String>>,
    pub headers: Option<Vec<String>>,
    pub version: Option<f32>,
    pub client_cert_pub: Option<String>,
    pub client_cert_key: Option<String>,
    pub theme_black_list: Option<Vec<String>>,
    pub theme_white_list: Option<Vec<String>>,
    pub theme_error: Option<String>,
    pub locations: Option<Vec<TinfoilLocation>>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TinfoilFile {
    pub url: String,
    pub size: i64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TinfoilLocation {
    pub url: String,
    pub title: String,
    pub action: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TinfoilToken {
    pub access_token: String,
    pub refresh_token: String,
}

impl TinfoilIndex {
    pub fn new() -> Self {
        Self {
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
            theme_black_list: None,
            theme_white_list: None,
            theme_error: None,
            locations: None,
        }
    }

    pub fn add_file(&mut self, file: TinfoilFile) -> &Self {
        if let Some(files) = &mut self.files {
            files.push(file)
        }

        self
    }
}
