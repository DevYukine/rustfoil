extern crate google_drive3 as drive3;

use crate::gdrive::model::{GoogleDriveFileInfo, GoogleDriveFolderInfo, GoogleDriveScanResult};
use async_recursion::async_recursion;
use drive3::{hyper, hyper_rustls, oauth2, DriveHub};
use google_drive3::api::Scope::Full;
use google_drive3::api::{File, Permission};
use google_drive3::hyper::client::HttpConnector;
use google_drive3::hyper::{Body, Response};
use hyper_rustls::HttpsConnector;
use oauth2::{InstalledFlowAuthenticator, InstalledFlowReturnMethod};
use std::fs;
use std::path::PathBuf;

pub struct GoogleDriveApiService {
    drive_hub: DriveHub<HttpsConnector<HttpConnector>>,
}

impl GoogleDriveApiService {
    pub async fn new(
        secret_path: PathBuf,
        token_path: PathBuf,
        headless: bool,
    ) -> anyhow::Result<GoogleDriveApiService> {
        let auth = InstalledFlowAuthenticator::builder(
            oauth2::read_application_secret(secret_path).await?,
            match headless {
                true => InstalledFlowReturnMethod::Interactive,
                false => InstalledFlowReturnMethod::HTTPRedirect,
            },
        )
        .persist_tokens_to_disk(token_path)
        .build()
        .await?;

        let hub = DriveHub::new(
            hyper::Client::builder().build(
                hyper_rustls::HttpsConnectorBuilder::new()
                    .with_native_roots()
                    .https_or_http()
                    .enable_http1()
                    .build(),
            ),
            auth,
        );

        Ok(GoogleDriveApiService { drive_hub: hub })
    }

    pub async fn trigger_auth(&self) -> anyhow::Result<()> {
        self.drive_hub
            .about()
            .get()
            .add_scope(Full)
            .param("fields", "*")
            .doit()
            .await?;
        Ok(())
    }

    pub async fn get_file(&self, file_id: &str) -> anyhow::Result<File> {
        let (_, file) = self
            .drive_hub
            .files()
            .get(file_id)
            .supports_all_drives(true)
            .add_scope(Full)
            .param("fields", "id,name,size,permissionIds")
            .doit()
            .await?;
        Ok(file)
    }

    pub async fn ls(
        &self,
        folder_id: &str,
        search_terms: Option<&str>,
    ) -> anyhow::Result<Vec<File>> {
        let mut files = Vec::new();

        let mut page_token: Option<String> = None;

        let q: String = vec![
            format!("\"{}\" in parents", folder_id),
            search_terms.unwrap_or("").to_string(),
            "trashed = false".to_string(),
        ]
        .join(" and ");

        loop {
            let req = self
                .drive_hub
                .files()
                .list()
                .q(q.as_str())
                .page_size(1000)
                .supports_all_drives(true)
                .include_items_from_all_drives(true)
                .param(
                    "fields",
                    "files(id,name,size,permissionIds,shortcutDetails),nextPageToken",
                );

            let resp = match page_token {
                None => req.add_scope(Full).doit().await?,
                Some(token) => {
                    req.page_token(token.as_str())
                        .add_scope(Full)
                        .doit()
                        .await?
                }
            };

            if let Some(response_files) = resp.1.files {
                for file in response_files {
                    files.push(file);
                }
            }

            page_token = resp.1.next_page_token;

            if page_token.is_none() {
                break;
            }
        }
        Ok(files)
    }

    pub async fn lsd(&self, folder_id: &str) -> anyhow::Result<Vec<File>> {
        self.ls(
            folder_id,
            Option::from("mimeType contains \"application/vnd.google-apps.folder\""),
        )
        .await
    }

    pub async fn lsf(&self, folder_id: &str) -> anyhow::Result<Vec<File>> {
        self.ls(
            folder_id,
            Option::from("not mimeType contains \"application/vnd.google-apps.folder\""),
        )
        .await
    }

    pub async fn lsd_my_drive(&self) -> anyhow::Result<Vec<File>> {
        self.ls(
            "root",
            Option::from("mimeType contains \"application/vnd.google-apps.folder\""),
        )
        .await
    }

    pub async fn lsf_my_drive(&self) -> anyhow::Result<Vec<File>> {
        self.ls(
            "root",
            Option::from("not mimeType contains \"application/vnd.google-apps.folder\""),
        )
        .await
    }

    pub async fn lss(&self, folder_id: &str) -> anyhow::Result<Vec<File>> {
        self.ls(
            folder_id,
            Option::from("mimeType contains \"application/vnd.google-apps.shortcut\""),
        )
        .await
    }

    pub async fn is_file_shared_by_id(&self, file_id: &str) -> anyhow::Result<bool> {
        let file = self.get_file(file_id).await?;
        self.is_file_shared(&file).await
    }

    pub async fn is_file_shared(&self, file: &File) -> anyhow::Result<bool> {
        let mut shared = false;

        let file_id = match &file.id {
            None => return Err(anyhow::Error::msg("File ID not found in file object")),
            Some(id) => id,
        };

        if let Some(ids) = &file.permission_ids {
            for id in ids {
                let mut vec: Vec<char> = id.chars().collect();

                let last = vec.remove(vec.len() - 1);

                let mut all_numeric = true;

                for char in vec {
                    all_numeric = char.is_numeric();
                    if !all_numeric {
                        break;
                    }
                }

                if last == 'k' && all_numeric {
                    self.delete_file_permissions(file_id.as_str(), id.as_str())
                        .await?;
                }

                if id == "anyoneWithLink" {
                    shared = true;
                }
            }
        }

        Ok(shared)
    }

    pub async fn delete_file_permissions(
        &self,
        file_id: &str,
        permission_id: &str,
    ) -> anyhow::Result<Response<Body>> {
        Ok(self
            .drive_hub
            .permissions()
            .delete(file_id, permission_id)
            .supports_all_drives(true)
            .add_scope(Full)
            .doit()
            .await?)
    }

    #[async_recursion]
    pub async fn get_all_files_in_folder(
        &self,
        folder_id: &str,
        recursion: bool,
    ) -> anyhow::Result<GoogleDriveScanResult> {
        let mut files = Vec::new();
        let mut folders = Vec::new();

        for file in self.lsf(folder_id).await? {
            if let Some(_) = &file.size {
                let is_shared = self.is_file_shared(&file).await?;

                files.push(GoogleDriveFileInfo::new(
                    file.id.unwrap(),
                    file.size.unwrap(),
                    file.name.unwrap(),
                    is_shared,
                ));
            }
        }

        if recursion {
            for shortcut in self.lss(folder_id).await? {
                let info = shortcut.shortcut_details.unwrap();

                if let Some(mime_type) = &info.target_mime_type {
                    if mime_type == "application/vnd.google-apps.folder" {
                        if let Some(id) = &info.target_id {
                            let folder = self.get_file(id).await?;

                            folders.push(GoogleDriveFolderInfo::new(
                                folder.id.to_owned().unwrap(),
                                self.is_file_shared(&folder).await?,
                            ));

                            for file_info in
                                self.get_all_files_in_folder(id, recursion).await?.files
                            {
                                files.push(file_info);
                            }
                        };
                    };
                };
            }

            for folder in self.lsd(folder_id).await? {
                let folder_id = folder.id.to_owned().unwrap();
                for file_info in self
                    .get_all_files_in_folder(folder_id.as_str(), recursion)
                    .await?
                    .files
                {
                    files.push(file_info);
                }
                folders.push(GoogleDriveFolderInfo::new(
                    folder_id,
                    self.is_file_shared(&folder).await?,
                ));
            }
        }

        Ok(GoogleDriveScanResult::new(files, folders))
    }

    pub async fn share_file(&self, file_id: &str) -> anyhow::Result<(Response<Body>, Permission)> {
        let mut perms = Permission::default();
        perms.role = Option::from("reader".to_string());
        perms.type_ = Option::from("anyone".to_string());
        Ok(self
            .drive_hub
            .permissions()
            .create(perms, file_id)
            .supports_all_drives(true)
            .add_scope(Full)
            .doit()
            .await?)
    }

    pub async fn upload_file(
        &self,
        file_path: PathBuf,
        dest_folder_id: &Option<String>,
    ) -> anyhow::Result<GoogleDriveFileInfo> {
        let root_files = if let Some(folder_id) = dest_folder_id {
            self.lsf(folder_id.as_str()).await
        } else {
            self.lsf_my_drive().await
        }?;

        let file_path_name = file_path.file_name().unwrap().to_str().unwrap();

        let mut existing_file: Option<File> = None;

        for file in root_files {
            if let Some(name) = file.name.as_ref() {
                if name == file_path_name {
                    existing_file = Some(file);
                }
            }
        }

        let res = match existing_file {
            Some(api_file) => {
                let mut req = File::default();

                req.name = api_file.name;

                let file = tokio::task::spawn_blocking(move || fs::File::open(file_path)).await??;

                self.drive_hub
                    .files()
                    .update(req, api_file.id.unwrap().as_str())
                    .supports_all_drives(true)
                    .add_scope(Full)
                    .upload_resumable(file, "application/octet-stream".parse()?)
                    .await?
                    .1
            }
            None => {
                let mut api_file = File::default();

                api_file.name = Some(file_path_name.to_string());

                if let Some(id) = dest_folder_id {
                    let mut vec = Vec::new();
                    vec.push(id.to_owned());
                    api_file.parents = Some(vec);
                }

                let file = tokio::task::spawn_blocking(move || fs::File::open(file_path)).await??;

                self.drive_hub
                    .files()
                    .create(api_file)
                    .supports_all_drives(true)
                    .add_scope(Full)
                    .upload_resumable(file, "application/octet-stream".parse()?)
                    .await?
                    .1
            }
        };

        let is_shared = self.is_file_shared(&res).await?;

        Ok(GoogleDriveFileInfo::new(
            res.id.unwrap(),
            res.size.unwrap(),
            res.name.unwrap(),
            is_shared,
        ))
    }
}
