use crate::gdrive::gdrive_api::GoogleDriveApiService;
use crate::gdrive::model::{GoogleDriveFileInfo, GoogleDriveScanResult};
use log::debug;
use std::path::PathBuf;

pub struct GoogleDriveService {
    pub api: GoogleDriveApiService,
}

impl GoogleDriveService {
    pub fn new(api: GoogleDriveApiService) -> Self {
        Self { api }
    }

    pub async fn scan_folders(
        &self,
        folder_ids: Vec<String>,
        no_recursion: bool,
    ) -> anyhow::Result<GoogleDriveScanResult> {
        // Trigger Authentication if needed
        self.api.trigger_auth().await?;

        let mut folders = vec![];

        for id in &folder_ids {
            let scan = self
                .api
                .get_all_files_in_folder(id.as_str(), !no_recursion)
                .await?;

            debug!(
                "Scanned Folder {} containing {} files & {} subfolders",
                id,
                scan.files.len(),
                scan.folders.len()
            );

            folders.push(scan);
        }

        let scan = folders.into_iter().fold(
            GoogleDriveScanResult::new(Vec::new(), Vec::new()),
            |mut old, mut new| {
                old.files.append(&mut new.files);
                old.folders.append(&mut new.folders);
                old
            },
        );

        Ok(scan)
    }

    pub async fn upload_index(
        &self,
        index_file_path: PathBuf,
        folder_id: Option<String>,
    ) -> anyhow::Result<GoogleDriveFileInfo> {
        let file = self.api.upload_file(index_file_path, &folder_id).await?;

        Ok(file)
    }

    pub async fn share_index(&self, file_id: &str) -> anyhow::Result<()> {
        let is_shared = self.api.is_file_shared_by_id(file_id).await?;

        self.share_file(file_id, is_shared).await?;

        Ok(())
    }

    pub async fn share_file(&self, file_id: &str, is_shared: bool) -> anyhow::Result<()> {
        if !is_shared {
            self.api.share_file(file_id).await?;
        }

        Ok(())
    }

    pub async fn share_folder(&self, folder_id: &str, is_shared: bool) -> anyhow::Result<()> {
        self.share_file(folder_id, is_shared).await?;

        Ok(())
    }
}
