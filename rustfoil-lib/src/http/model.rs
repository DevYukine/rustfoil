use crate::abstraction::file::TinfoilFileLike;
use crate::fs::file::LocalFile;
use std::path::PathBuf;

pub struct HttpFile {
    pub url: String,
    pub size: i64,
    pub name: String,
}

impl TinfoilFileLike for HttpFile {
    fn get_url(&self) -> String {
        self.url.clone()
    }

    fn get_size(&self) -> i64 {
        self.size
    }

    fn get_name(&self) -> &str {
        self.name.as_str()
    }
}

impl HttpFile {
    pub fn new(url: String, size: i64, name: String) -> HttpFile {
        HttpFile { url, size, name }
    }

    pub fn from_local_with_base_url(
        base_url: &str,
        base_path: &PathBuf,
        local_file: &LocalFile,
    ) -> anyhow::Result<HttpFile> {
        let path = local_file.path.strip_prefix(base_path)?;

        Ok(HttpFile {
            url: format!(
                "{}{}",
                base_url,
                urlencoding::encode(path.to_str().unwrap())
            ),
            size: local_file.size as i64,
            name: local_file.name.clone(),
        })
    }
}
