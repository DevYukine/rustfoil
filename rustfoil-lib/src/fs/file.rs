use async_recursion::async_recursion;

use std::path::PathBuf;
use tokio::fs;

#[derive(Debug, Clone)]
pub struct LocalFile {
    pub path: PathBuf,
    pub size: u64,
    pub name: String,
}

#[async_recursion]
pub async fn read_files_recursive(folder_path: &PathBuf) -> anyhow::Result<Vec<LocalFile>> {
    let mut dir = fs::read_dir(&folder_path).await?;
    let mut files = Vec::new();

    while let Some(entry) = dir.next_entry().await? {
        let path = entry.path();

        if path.is_dir() {
            files.append(&mut read_files_recursive(&path).await?);
        } else {
            let metadata = entry.metadata().await?;

            let file = LocalFile {
                path: path.clone(),
                size: metadata.len(),
                name: path.file_name().unwrap().to_str().unwrap().to_string(),
            };

            files.push(file);
        }
    }

    return Ok(files);
}
