use google_drive3::{DriveHub, File, FileListCall, Permission};
use hyper::Client;
use std::borrow::Borrow;
use std::error::Error;
use std::path::Path;
use std::{error, fmt};
use yup_oauth2::{Authenticator, DefaultAuthenticatorDelegate, DiskTokenStorage, FlowType};

pub struct GDriveService {
    drive_hub:
        DriveHub<Client, Authenticator<DefaultAuthenticatorDelegate, DiskTokenStorage, Client>>,
}

#[derive(Debug, Clone)]
pub struct DriveError {
    message: String,
}

pub struct FileInfo {
    pub id: String,
    pub size: String,
    pub name: String,
    pub shared: bool,
}

impl fmt::Display for DriveError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.message)
    }
}

impl DriveError {
    pub fn new(message: String) -> DriveError {
        return DriveError { message };
    }
}

impl error::Error for DriveError {}

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
    pub fn new(secret_path: &Path, token_path: &Path) -> GDriveService {
        let secret = yup_oauth2::read_application_secret(secret_path)
            .expect("failed to read \"credentials.json\" file");
        let auth = Authenticator::new(
            &secret,
            DefaultAuthenticatorDelegate,
            hyper::Client::with_connector(hyper::net::HttpsConnector::new(
                hyper_rustls::TlsClient::new(),
            )),
            DiskTokenStorage::new(&token_path.to_str().unwrap().to_string()).unwrap(),
            Option::from(FlowType::InstalledRedirect(3333)),
        );
        let hub = DriveHub::new(
            hyper::Client::with_connector(hyper::net::HttpsConnector::new(
                hyper_rustls::TlsClient::new(),
            )),
            auth,
        );

        GDriveService { drive_hub: hub }
    }

    pub fn trigger_auth(&self) {
        self.drive_hub.about().get().doit();
    }

    pub fn ls(&self, folder_id: &str, search_terms: Option<&str>) -> Result<Vec<File>, DriveError> {
        let mut files: Vec<File> = Vec::new();

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
                .param("fields", "files(id,name,size,permissionIds),nextPageToken");

            let resp = match page_token {
                None => req.doit().unwrap(),
                Some(_) => req.page_token(page_token.unwrap().as_str()).doit().unwrap(),
            };

            if !resp.0.status.is_success() {
                return Err(DriveError::new(format!(
                    "Non Success Status Code {}",
                    &resp.0.status
                )));
            }

            for file in resp.1.files.unwrap() {
                files.push(file);
            }

            page_token = resp.1.next_page_token.clone();

            if page_token.is_none() {
                break;
            }
        }
        Ok(files)
    }

    pub fn lsd(&self, folder_id: &str) -> Result<Vec<File>, DriveError> {
        self.ls(
            folder_id,
            Option::from("mimeType contains \"application/vnd.google-apps.folder\""),
        )
    }

    pub fn lsf(&self, folder_id: &str) -> Result<Vec<File>, DriveError> {
        self.ls(
            folder_id,
            Option::from("not mimeType contains \"application/vnd.google-apps.folder\""),
        )
    }

    pub fn lsd_my_drive(&self) -> Result<Vec<File>, DriveError> {
        self.ls(
            "root",
            Option::from("mimeType contains \"application/vnd.google-apps.folder\""),
        )
    }

    pub fn lsf_my_drive(&self) -> Result<Vec<File>, DriveError> {
        self.ls(
            "root",
            Option::from("not mimeType contains \"application/vnd.google-apps.folder\""),
        )
    }

    pub fn is_file_shared(&self, file: File) -> bool {
        let mut shared = false;

        let file_id = file.id.unwrap();

        if file.permission_ids.is_some() {
            for id in file.permission_ids.unwrap() {
                let original_vector: Vec<char> = id.chars().collect();

                let mut vector = original_vector.to_vec();

                vector.remove(vector.len() - 1);

                let mut all_numeric = true;

                for char in vector {
                    all_numeric = char.is_numeric();
                    if !all_numeric {
                        break;
                    }
                }

                if *original_vector.last().unwrap() == 'k' && all_numeric {
                    self.delete_file_permissions(file_id.as_str(), id.as_str())
                }

                if id == "anyoneWithLink" {
                    shared = true
                }
            }
        }

        shared
    }

    pub fn delete_file_permissions(&self, file_id: &str, permission_id: &str) {
        self.drive_hub
            .permissions()
            .delete(file_id, permission_id)
            .doit();
    }

    pub fn get_all_files_in_folder(&self, folder_id: &str, recursion: bool) -> Vec<FileInfo> {
        let mut files: Vec<FileInfo> = Vec::new();

        for file in self.lsf(folder_id).unwrap() {
            if file.size.is_some() {
                let cl = file.clone();
                files.push(FileInfo::new(
                    file.id.unwrap(),
                    file.size.unwrap(),
                    file.name.unwrap(),
                    self.is_file_shared(cl),
                ));
            }
        }

        if recursion {
            for folder in self.lsd(folder_id).unwrap() {
                for file_info in
                    self.get_all_files_in_folder(folder.id.unwrap().as_str(), recursion)
                {
                    files.push(file_info);
                }
            }
        }

        files
    }

    pub fn share_files(&self, file_id: &str) {
        let mut perms = Permission::default();
        perms.role = Option::from("reader".to_string());
        perms.type_ = Option::from("anyone".to_string());
        self.drive_hub.permissions().create(perms, file_id).doit();
    }
}
