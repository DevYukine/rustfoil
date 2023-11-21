use crate::r#enum::compression::Compression;
use clap::{Parser, Subcommand};
use std::path::PathBuf;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand, Debug)]
pub enum Commands {
    /// Generates an Index for files hosted on Google Drive
    Gdrive(GoogleDriveCommand),

    /// Generates an Index for files hosted via a http server
    Http(HttpCommand),
}

pub trait IndexCommand {
    fn output_path(&self) -> &PathBuf;
    fn no_recursion(&self) -> bool;
    fn add_nsw_files_without_title_id(&self) -> bool;
    fn add_non_nsw_files(&self) -> bool;
    fn compression(&self) -> Option<Compression>;
    fn encrypt(&self) -> bool;
    fn public_key(&self) -> Option<PathBuf>;
    fn success(&self) -> Option<String>;
    fn referrer(&self) -> Option<String>;
    fn google_api_key(&self) -> Option<String>;
    fn headers(&self) -> Option<Vec<String>>;
    fn min_version(&self) -> Option<f32>;
    fn theme_blacklist(&self) -> Option<Vec<String>>;
    fn theme_whitelist(&self) -> Option<Vec<String>>;
    fn theme_error(&self) -> Option<String>;
}

#[derive(Parser, Debug, Clone)]
pub struct GoogleDriveCommand {
    /// Folder IDs of Google Drive folders to scan
    pub folder_ids: Vec<String>,

    /// Path to output index file
    #[arg(short = 'o', long, default_value = "index.tfl")]
    pub output_path: PathBuf,

    /// Scans for files only in top directory for each Folder entered
    #[arg(long)]
    pub no_recursion: bool,

    /// Adds files without valid Title ID
    #[arg(long)]
    pub add_nsw_files_without_title_id: bool,

    /// Adds files without valid NSW ROM extension(NSP/NSZ/XCI/XCZ) to index
    #[arg(long)]
    pub add_non_nsw_files: bool,

    /// Compression to use for index file
    #[arg(short, long, value_enum)]
    pub compression: Option<Compression>,

    /// If set, encrypts index file
    #[arg(short, long)]
    pub encrypt: bool,

    /// Path to RSA Public Key to encrypt AES-ECB-256 key with
    #[arg(long, default_value = "public.key")]
    pub public_key: Option<PathBuf>,

    /// Adds a success message to index file to show if index is successfully read by Tinfoil
    #[arg(long)]
    pub success: Option<String>,

    /// Adds a success message to index file to show if index is successfully read by Tinfoil
    #[arg(long)]
    pub referrer: Option<String>,

    /// Google API Key to use for Google Drive API requests, this is not the same as OAuth!
    #[arg(long)]
    pub google_api_key: Option<String>,

    /// specified custom HTTP headers which should be sent by tinfoil requests
    #[arg(long)]
    pub headers: Option<Vec<String>>,

    /// Adds a minimum Tinfoil version to load the index
    #[arg(long)]
    pub min_version: Option<f32>,

    /// Adds a list of themes to blacklist based on their hash
    #[arg(long)]
    pub theme_blacklist: Option<Vec<String>>,

    /// Adds a list of themes to whitelist based on their hash
    #[arg(long)]
    pub theme_whitelist: Option<Vec<String>>,

    /// Adds a custom theme error message to the index
    #[arg(long)]
    pub theme_error: Option<String>,

    /// Path to Google Application Credentials
    #[arg(long, default_value = "credentials.json")]
    pub credentials: PathBuf,

    /// Path to Google OAuth2.0 User Token
    #[arg(long, default_value = "token.json")]
    pub token: PathBuf,

    /// Path to Tinfoil authentication files
    #[arg(long, default_value = "COPY_TO_SD/switch/tinfoil")]
    pub tinfoil_auth_path: PathBuf,

    /// If OAuth should be done headless
    #[arg(long)]
    pub headless: bool,

    /// If Tinfoil authentication files should be generated
    #[arg(long)]
    pub tinfoil_auth: bool,

    /// Share all files inside the index file
    #[arg(long)]
    pub share_files: bool,

    /// Share all folders inside the provided folders
    #[arg(long)]
    pub share_folders: bool,

    /// If the index file should be uploaded to specific folder
    #[arg(long)]
    pub upload_folder_id: Option<String>,

    /// If the index file should be uploaded to My Drive
    #[arg(long)]
    pub upload_my_drive: bool,

    /// Shares the index file that is uploaded to Google Drive
    #[arg(long)]
    pub share_index: bool,
}

#[derive(Parser, Debug, Clone)]
pub struct HttpCommand {
    /// The base Url to prepend to all file paths when creating the index, this includes http:// or https://, the domain/ip & optionally a port
    pub http_base_url: String,

    /// local folder paths to scan
    pub folder_paths: Vec<PathBuf>,

    /// Path to output index file
    #[arg(short = 'o', long, default_value = "index.tfl")]
    pub output_path: PathBuf,

    /// Scans for files only in top directory for each Folder entered
    #[arg(long)]
    pub no_recursion: bool,

    /// Adds files without valid Title ID
    #[arg(long)]
    pub add_nsw_files_without_title_id: bool,

    /// Adds files without valid NSW ROM extension(NSP/NSZ/XCI/XCZ) to index
    #[arg(long)]
    pub add_non_nsw_files: bool,

    /// Compression to use for index file
    #[arg(short, long, value_enum)]
    pub compression: Option<Compression>,

    /// If set, encrypts index file
    #[arg(short, long)]
    pub encrypt: bool,

    /// Path to RSA Public Key to encrypt AES-ECB-256 key with
    #[arg(long, default_value = "public.key")]
    pub public_key: Option<PathBuf>,

    /// Adds a success message to index file to show if index is successfully read by Tinfoil
    #[arg(long)]
    pub success: Option<String>,

    /// Adds a success message to index file to show if index is successfully read by Tinfoil
    #[arg(long)]
    pub referrer: Option<String>,

    /// specified custom HTTP headers which should be sent by tinfoil requests
    #[arg(long)]
    pub headers: Option<Vec<String>>,

    /// Adds a minimum Tinfoil version to load the index
    #[arg(long)]
    pub min_version: Option<f32>,

    /// Adds a list of themes to blacklist based on their hash
    #[arg(long)]
    pub theme_blacklist: Option<Vec<String>>,

    /// Adds a list of themes to whitelist based on their hash
    #[arg(long)]
    pub theme_whitelist: Option<Vec<String>>,

    /// Adds a custom theme error message to the index
    #[arg(long)]
    pub theme_error: Option<String>,
}

impl IndexCommand for GoogleDriveCommand {
    fn output_path(&self) -> &PathBuf {
        &self.output_path
    }

    fn no_recursion(&self) -> bool {
        self.no_recursion
    }

    fn add_nsw_files_without_title_id(&self) -> bool {
        self.add_nsw_files_without_title_id
    }

    fn add_non_nsw_files(&self) -> bool {
        self.add_non_nsw_files
    }

    fn compression(&self) -> Option<Compression> {
        self.compression
    }

    fn encrypt(&self) -> bool {
        self.encrypt
    }

    fn public_key(&self) -> Option<PathBuf> {
        self.public_key.clone()
    }

    fn success(&self) -> Option<String> {
        self.success.clone()
    }

    fn referrer(&self) -> Option<String> {
        self.referrer.clone()
    }

    fn google_api_key(&self) -> Option<String> {
        self.google_api_key.clone()
    }

    fn headers(&self) -> Option<Vec<String>> {
        self.headers.clone()
    }

    fn min_version(&self) -> Option<f32> {
        self.min_version
    }

    fn theme_blacklist(&self) -> Option<Vec<String>> {
        self.theme_blacklist.clone()
    }

    fn theme_whitelist(&self) -> Option<Vec<String>> {
        self.theme_whitelist.clone()
    }

    fn theme_error(&self) -> Option<String> {
        self.theme_error.clone()
    }
}

impl IndexCommand for HttpCommand {
    fn output_path(&self) -> &PathBuf {
        &self.output_path
    }

    fn no_recursion(&self) -> bool {
        self.no_recursion
    }

    fn add_nsw_files_without_title_id(&self) -> bool {
        self.add_nsw_files_without_title_id
    }

    fn add_non_nsw_files(&self) -> bool {
        self.add_non_nsw_files
    }

    fn compression(&self) -> Option<Compression> {
        self.compression
    }

    fn encrypt(&self) -> bool {
        self.encrypt
    }

    fn public_key(&self) -> Option<PathBuf> {
        self.public_key.clone()
    }

    fn success(&self) -> Option<String> {
        self.success.clone()
    }

    fn referrer(&self) -> Option<String> {
        self.referrer.clone()
    }

    fn google_api_key(&self) -> Option<String> {
        None
    }

    fn headers(&self) -> Option<Vec<String>> {
        self.headers.clone()
    }

    fn min_version(&self) -> Option<f32> {
        self.min_version
    }

    fn theme_blacklist(&self) -> Option<Vec<String>> {
        self.theme_blacklist.clone()
    }

    fn theme_whitelist(&self) -> Option<Vec<String>> {
        self.theme_whitelist.clone()
    }

    fn theme_error(&self) -> Option<String> {
        self.theme_error.clone()
    }
}
