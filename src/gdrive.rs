use async_recursion::async_recursion;
use google_drive3::api::Scope::Full;
use google_drive3::api::{About, File, Permission};
use google_drive3::DriveHub;
use hyper::{Body, Response};
use std::fs;
use std::path::Path;
use yup_oauth2::{InstalledFlowAuthenticator, InstalledFlowReturnMethod};

pub struct GDriveService {
    drive_hub: DriveHub,
}

#[derive(Clone)]
pub struct FileInfo {
    pub id: String,
    pub size: String,
    pub name: String,
    pub shared: bool,
}

pub struct FolderInfo {
    pub id: String,
    pub shared: bool,
}

pub struct ScanResult {
    pub files: Vec<FileInfo>,
    pub folders: Vec<FolderInfo>,
}

impl ScanResult {
    pub fn new(files: Vec<FileInfo>, folders: Vec<FolderInfo>) -> ScanResult {
        ScanResult { files, folders }
    }
}

impl FolderInfo {
    pub fn new(id: String, shared: bool) -> FolderInfo {
        FolderInfo { id, shared }
    }
}

impl FileInfo {
    pub fn new(id: String, size: String, name: String, shared: bool) -> FileInfo {
        FileInfo {
            id,
            size,
            name,
            shared,
        }
    }
}

impl GDriveService {
    pub async fn new(
        secret_path: &Path,
        token_path: &Path,
        headless: bool,
    ) -> std::io::Result<GDriveService> {
        let auth = InstalledFlowAuthenticator::builder(
            yup_oauth2::read_application_secret(secret_path).await?,
            match headless {
                true => InstalledFlowReturnMethod::Interactive,
                false => InstalledFlowReturnMethod::HTTPRedirect,
            },
        )
        .persist_tokens_to_disk(token_path)
        .build()
        .await?;

        let hub = DriveHub::new(
            hyper::Client::builder().build(hyper_rustls::HttpsConnector::with_native_roots()),
            auth,
        );

        Ok(GDriveService { drive_hub: hub })
    }

    pub async fn trigger_auth(
        &self,
    ) -> google_drive3::Result<(hyper::Response<hyper::body::Body>, About)> {
        self.drive_hub
            .about()
            .get()
            .add_scope(Full)
            .param("fields", "*")
            .doit()
            .await
    }

    pub async fn get_file(
        &self,
        file_id: &str,
    ) -> google_drive3::Result<(hyper::Response<hyper::body::Body>, File)> {
        self.drive_hub
            .files()
            .get(file_id)
            .supports_all_drives(true)
            .add_scope(Full)
            .doit()
            .await
    }

    pub async fn ls(
        &self,
        folder_id: &str,
        search_terms: Option<&str>,
    ) -> google_drive3::Result<Vec<File>> {
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
                Some(_) => {
                    req.page_token(page_token.unwrap().as_str())
                        .add_scope(Full)
                        .doit()
                        .await?
                }
            };

            for file in resp.1.files.unwrap() {
                files.push(file);
            }

            page_token = resp.1.next_page_token;

            if page_token.is_none() {
                break;
            }
        }
        Ok(files)
    }

    pub async fn lsd(&self, folder_id: &str) -> google_drive3::Result<Vec<File>> {
        self.ls(
            folder_id,
            Option::from("mimeType contains \"application/vnd.google-apps.folder\""),
        )
        .await
    }

    pub async fn lsf(&self, folder_id: &str) -> google_drive3::Result<Vec<File>> {
        self.ls(
            folder_id,
            Option::from("not mimeType contains \"application/vnd.google-apps.folder\""),
        )
        .await
    }

    pub async fn lsd_my_drive(&self) -> google_drive3::Result<Vec<File>> {
        self.ls(
            "root",
            Option::from("mimeType contains \"application/vnd.google-apps.folder\""),
        )
        .await
    }

    pub async fn lsf_my_drive(&self) -> google_drive3::Result<Vec<File>> {
        self.ls(
            "root",
            Option::from("not mimeType contains \"application/vnd.google-apps.folder\""),
        )
        .await
    }

    pub async fn lss(&self, folder_id: &str) -> google_drive3::Result<Vec<File>> {
        self.ls(
            folder_id,
            Option::from("mimeType contains \"application/vnd.google-apps.shortcut\""),
        )
        .await
    }

    pub async fn is_file_shared(&self, file: File) -> google_drive3::Result<bool> {
        let mut shared = false;

        let file_id = file.id.unwrap();

        if let Some(ids) = file.permission_ids {
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
                    shared = true
                }
            }
        }

        Ok(shared)
    }

    pub async fn delete_file_permissions(
        &self,
        file_id: &str,
        permission_id: &str,
    ) -> google_drive3::Result<Response<Body>> {
        self.drive_hub
            .permissions()
            .delete(file_id, permission_id)
            .supports_all_drives(true)
            .add_scope(Full)
            .doit()
            .await
    }

    #[async_recursion]
    pub async fn get_all_files_in_folder(
        &self,
        folder_id: &str,
        recursion: bool,
    ) -> google_drive3::Result<ScanResult> {
        let mut files = Vec::new();
        let mut folders = Vec::new();

        for file in self.lsf(folder_id).await? {
            if let Some(_) = &file.size {
                files.push(FileInfo::new(
                    file.id.to_owned().unwrap(),
                    file.size.to_owned().unwrap(),
                    file.name.to_owned().unwrap(),
                    self.is_file_shared(file).await?,
                ));
            }
        }

        if recursion {
            for shortcut in self.lss(folder_id).await? {
                let info = shortcut.shortcut_details.unwrap();

                if let Some(mime_type) = &info.target_mime_type {
                    if mime_type == "application/vnd.google-apps.folder" {
                        if let Some(id) = &info.target_id {
                            let folder = self.get_file(id).await?.1;

                            folders.push(FolderInfo::new(
                                folder.id.to_owned().unwrap(),
                                self.is_file_shared(folder).await?,
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
                folders.push(FolderInfo::new(
                    folder_id,
                    self.is_file_shared(folder).await?,
                ));
            }
        }

        Ok(ScanResult::new(files, folders))
    }

    pub async fn share_file(
        &self,
        file_id: &str,
    ) -> google_drive3::Result<(Response<Body>, Permission)> {
        let mut perms = Permission::default();
        perms.role = Option::from("reader".to_string());
        perms.type_ = Option::from("anyone".to_string());
        self.drive_hub
            .permissions()
            .create(perms, file_id)
            .supports_all_drives(true)
            .add_scope(Full)
            .doit()
            .await
    }

    pub async fn upload_file(
        &self,
        file_path: &Path,
        dest_folder_id: &Option<String>,
    ) -> google_drive3::Result<(String, bool)> {
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
            Some(file) => {
                let mut req = File::default();

                req.name = file.name;

                self.drive_hub
                    .files()
                    .update(req, file.id.unwrap().as_str())
                    .supports_all_drives(true)
                    .add_scope(Full)
                    .upload_resumable(
                        fs::File::open(file_path).unwrap(),
                        "application/octet-stream".parse().unwrap(),
                    )
                    .await?
                    .1
            }
            None => {
                let mut file = File::default();

                file.name = Some(file_path_name.to_string());

                if let Some(id) = dest_folder_id {
                    let mut vec = Vec::new();
                    vec.push(id.to_owned());
                    file.parents = Some(vec);
                }

                self.drive_hub
                    .files()
                    .create(file)
                    .supports_all_drives(true)
                    .add_scope(Full)
                    .upload_resumable(
                        fs::File::open(file_path).unwrap(),
                        "application/octet-stream".parse().unwrap(),
                    )
                    .await?
                    .1
            }
        };

        Ok((res.id.to_owned().unwrap(), self.is_file_shared(res).await?))
    }
}
