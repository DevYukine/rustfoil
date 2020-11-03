use google_drive3::Scope::Full;
use google_drive3::{About, DriveHub, File, Permission};
use hyper::client::Response;
use hyper::Client;
use std::fs;
use std::path::Path;
use yup_oauth2::{Authenticator, DefaultAuthenticatorDelegate, DiskTokenStorage, FlowType};

trait ScopedRequest<'a, A, B> {
    fn add_scope<T, S>(&self, scope: T) -> Box<Self>
    where
        T: Into<Option<S>>,
        S: AsRef<str>;
}

pub struct GDriveService {
    drive_hub:
        DriveHub<Client, Authenticator<DefaultAuthenticatorDelegate, DiskTokenStorage, Client>>,
}

pub struct FileInfo {
    pub id: String,
    pub size: String,
    pub name: String,
    pub shared: bool,
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
    pub fn new(secret_path: &Path, token_path: &Path, headless: bool) -> GDriveService {
        let secret = yup_oauth2::read_application_secret(secret_path)
            .expect("failed to read \"credentials.json\" file");
        let auth = Authenticator::new(
            &secret,
            DefaultAuthenticatorDelegate,
            hyper::Client::with_connector(hyper::net::HttpsConnector::new(
                hyper_rustls::TlsClient::new(),
            )),
            DiskTokenStorage::new(&token_path.to_str().unwrap().to_string()).unwrap(),
            Option::from(match headless {
                true => FlowType::InstalledInteractive,
                false => FlowType::InstalledRedirect(3333),
            }),
        );
        let hub = DriveHub::new(
            hyper::Client::with_connector(hyper::net::HttpsConnector::new(
                hyper_rustls::TlsClient::new(),
            )),
            auth,
        );

        GDriveService { drive_hub: hub }
    }

    pub fn trigger_auth(&self) -> google_drive3::Result<(Response, About)> {
        self.drive_hub.about().get().add_scope(Full).doit()
    }

    pub fn ls(
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
                .param("fields", "files(id,name,size,permissionIds),nextPageToken");

            let resp = match page_token {
                None => req.add_scope(Full).doit()?,
                Some(_) => req
                    .page_token(page_token.unwrap().as_str())
                    .add_scope(Full)
                    .doit()?,
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

    pub fn lsd(&self, folder_id: &str) -> google_drive3::Result<Vec<File>> {
        self.ls(
            folder_id,
            Option::from("mimeType contains \"application/vnd.google-apps.folder\""),
        )
    }

    pub fn lsf(&self, folder_id: &str) -> google_drive3::Result<Vec<File>> {
        self.ls(
            folder_id,
            Option::from("not mimeType contains \"application/vnd.google-apps.folder\""),
        )
    }

    pub fn lsd_my_drive(&self) -> google_drive3::Result<Vec<File>> {
        self.ls(
            "root",
            Option::from("mimeType contains \"application/vnd.google-apps.folder\""),
        )
    }

    pub fn lsf_my_drive(&self) -> google_drive3::Result<Vec<File>> {
        self.ls(
            "root",
            Option::from("not mimeType contains \"application/vnd.google-apps.folder\""),
        )
    }

    pub fn is_file_shared(&self, file: File) -> google_drive3::Result<bool> {
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
                    self.delete_file_permissions(file_id.as_str(), id.as_str())?;
                }

                if id == "anyoneWithLink" {
                    shared = true
                }
            }
        }

        Ok(shared)
    }

    pub fn delete_file_permissions(
        &self,
        file_id: &str,
        permission_id: &str,
    ) -> google_drive3::Result<Response> {
        self.drive_hub
            .permissions()
            .delete(file_id, permission_id)
            .add_scope(Full)
            .doit()
    }

    pub fn get_all_files_in_folder(
        &self,
        folder_id: &str,
        recursion: bool,
    ) -> google_drive3::Result<Vec<FileInfo>> {
        let mut files = Vec::new();

        for file in self.lsf(folder_id)? {
            if let Some(_) = &file.size {
                files.push(FileInfo::new(
                    file.id.to_owned().unwrap(),
                    file.size.to_owned().unwrap(),
                    file.name.to_owned().unwrap(),
                    self.is_file_shared(file)?,
                ));
            }
        }

        if recursion {
            for folder in self.lsd(folder_id).unwrap() {
                for file_info in
                    self.get_all_files_in_folder(folder.id.unwrap().as_str(), recursion)?
                {
                    files.push(file_info);
                }
            }
        }

        Ok(files)
    }

    pub fn share_file(&self, file_id: &str) -> google_drive3::Result<(Response, Permission)> {
        let mut perms = Permission::default();
        perms.role = Option::from("reader".to_string());
        perms.type_ = Option::from("anyone".to_string());
        self.drive_hub
            .permissions()
            .create(perms, file_id)
            .supports_all_drives(true)
            .add_scope(Full)
            .doit()
    }

    pub fn upload_file(
        &self,
        file_path: &Path,
        dest_folder_id: Option<String>,
    ) -> google_drive3::Result<(String, bool)> {
        let root_files = if dest_folder_id.is_some() {
            self.lsf(dest_folder_id.unwrap().as_str())
        } else {
            self.lsf_my_drive()
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
                    .unwrap()
                    .1
            }
            None => {
                let mut file = File::default();

                file.name = Some(file_path_name.to_string());

                self.drive_hub
                    .files()
                    .create(file)
                    .supports_all_drives(true)
                    .add_scope(Full)
                    .upload_resumable(
                        fs::File::open(file_path).unwrap(),
                        "application/octet-stream".parse().unwrap(),
                    )
                    .unwrap()
                    .1
            }
        };

        let id = res.id.clone().unwrap();

        Ok((id, self.is_file_shared(res).unwrap()))
    }
}
