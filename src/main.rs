#[cfg_attr(test, macro_use)]
extern crate structopt;

use compression::CompressionFlag;
use encryption::EncryptionFlag;
use gdrive::GDriveService;
use index::FileEntry;
use index::Index;
use index::ParsedFileInfo;
use indicatif::{ProgressBar, ProgressStyle};
use logging::Logger;
use regex::Regex;
use std::path::PathBuf;
use structopt::StructOpt;
use tinfoil::convert_to_tinfoil_format;

mod compression;
mod encryption;
mod gdrive;
mod index;
mod logging;
mod tinfoil;
mod util;

/// Script that will allow you to generate an index file with Google Drive file links for use with Tinfoil
#[derive(StructOpt, Debug)]
#[structopt(rename_all = "kebab-case")]
pub struct Input {
    /// Folder IDs of Google Drive folders to scan
    folder_id: String,

    /// Path to Google Application Credentials
    #[structopt(long, parse(from_os_str), default_value = "credentials.json")]
    credentials: PathBuf,

    /// Path to Google OAuth2.0 User Token
    #[structopt(long, parse(from_os_str), default_value = "token.json")]
    token: PathBuf,

    /// Path to output index file
    #[structopt(short = "o", long, parse(from_os_str), default_value = "index.tlf")]
    output_path: PathBuf,

    /// Share all files inside the index file
    #[structopt(long)]
    share_files: bool,

    /// Scans for files only in top directory for each Folder ID entered
    #[structopt(long)]
    no_recursion: bool,

    /// Adds files without valid Title ID
    #[structopt(long)]
    add_nsw_files_without_title_id: bool,

    /// Adds files without valid NSW ROM extension(NSP/NSZ/XCI/XCZ) to index
    #[structopt(long)]
    add_non_nsw_files: bool,

    /// Adds a success message to index file to show if index is successfully read by Tinfoil
    #[structopt(long)]
    success: Option<String>,

    /// Adds a referrer to index file to prevent others from hotlinking
    #[structopt(long)]
    referrer: Option<String>,

    /// Adds a google API key to be used with all gdrive:/ requests
    #[structopt(long)]
    google_api_key: Option<String>,

    /// Adds 1Fincher API keys to be used with all 1f:/ requests, If multiple keys are provided, Tinfoil keeps trying them until it finds one that works
    #[structopt(long)]
    one_fichier_keys: Option<Vec<String>>,

    /// Adds custom HTTP headers Tinfoil should send with its requests
    #[structopt(long)]
    headers: Option<Vec<String>>,

    /// Adds a minimum Tinfoil version to load the index
    #[structopt(long)]
    min_version: Option<String>,

    /// Adds a list of themes to blacklist based on their hash
    #[structopt(long)]
    theme_blacklist: Option<Vec<String>>,

    /// Adds a list of themes to whitelist based on their hash
    #[structopt(long)]
    theme_whitelist: Option<Vec<String>>,

    /// Adds a custom theme error message to the index
    #[structopt(long)]
    theme_error: Option<String>,

    /// Path to RSA Public Key to encrypt AES-ECB-256 key with
    #[structopt(long)]
    public_key: Option<PathBuf>,

    /// Shares the index file that is uploaded to Google Drive
    #[structopt(long)]
    share_index: bool,

    /// Which compression should be used for the index file
    #[structopt(long, default_value = "zstd")]
    compression: CompressionFlag,

    /// Verbose mode (-v, -vv, -vvv, etc.)
    #[structopt(short, long, parse(from_occurrences))]
    verbose: u8,
}

pub struct RustfoilService {
    logger: Logger,
    input: Input,
    gdrive: GDriveService,
}

impl RustfoilService {
    pub fn new(input: Input) -> RustfoilService {
        let credentials = input.credentials.clone();
        let token = input.token.clone();
        RustfoilService {
            input,
            logger: Logger::new(),
            gdrive: GDriveService::new(credentials.as_path(), token.as_path()),
        }
    }

    pub fn validate_input(&self) -> bool {
        if !&self.input.credentials.exists() {
            &self.logger.log_error("Credentials file is missing!");
            false;
        }

        true
    }

    pub fn generate_index(&mut self, files: Vec<ParsedFileInfo>) -> Box<Index> {
        let mut index = Box::new(Index::new());

        let mut index_files: Vec<FileEntry> = Vec::new();

        for info in files {
            index_files.push(FileEntry::new(
                format!("gdrive:{}#{}", info.id, info.name_encoded),
                u64::from_str_radix(&*info.size, 10).unwrap(),
            ));
        }

        index.files = Some(index_files);

        self.logger.log_info("Added files to index");

        if self.input.success.is_some() {
            index.success = Some(self.input.success.clone().unwrap());
            self.logger.log_info("Added success message to index");
        }

        if self.input.referrer.is_some() {
            index.referrer = Some(self.input.referrer.clone().unwrap());
            self.logger.log_info("Added referrer to index");
        }

        if self.input.google_api_key.is_some() {
            index.google_api_key = Some(self.input.google_api_key.clone().unwrap());
            self.logger.log_info("Added google api key to index");
        }

        if self.input.one_fichier_keys.is_some() {
            index.one_fichier_keys = Some(self.input.one_fichier_keys.clone().unwrap());
            self.logger.log_info("Added 1Fichier keys to index");
        }

        if self.input.headers.is_some() {
            index.headers = Some(self.input.headers.clone().unwrap());
            self.logger.log_info("Added headers to index");
        }

        if self.input.min_version.is_some() {
            index.version = Some(self.input.min_version.clone().unwrap());
            self.logger.log_info("Added minimum version to index");
        }

        if self.input.theme_blacklist.is_some() {
            index.theme_blacklist = Some(self.input.theme_blacklist.clone().unwrap());
            self.logger.log_info("Added theme blacklist to index");
        }

        if self.input.theme_whitelist.is_some() {
            index.theme_whitelist = Some(self.input.theme_whitelist.clone().unwrap());
            self.logger.log_info("Added theme whitelist to index");
        }

        if self.input.theme_error.is_some() {
            index.theme_error = Some(self.input.theme_error.clone().unwrap());
            self.logger.log_info("Added theme error message to index");
        }

        self.logger.log_info("Generated index successfully");

        index
    }

    pub fn output_index(&self, index: Index) {
        let json = serde_json::to_string(&index).unwrap();
        let compression = &self.input.compression;

        std::fs::write(
            &self.input.output_path,
            convert_to_tinfoil_format(
                compression.compress(json.as_str()),
                compression.clone(),
                EncryptionFlag::NoEncrypt,
            ),
        )
        .expect("Couldn't write output file to Path");

        self.logger.log_info(
            format!(
                "Finished writing {} to disk",
                self.input
                    .output_path
                    .file_name()
                    .unwrap()
                    .to_str()
                    .unwrap()
            )
            .as_str(),
        )
    }

    pub fn share_files(&self, files: Vec<ParsedFileInfo>) {
        let pb = ProgressBar::new(files.len() as u64);

        pb.set_style(
            ProgressStyle::default_bar()
                .template("[{elapsed_precise}] [{bar:40.cyan/blue}] {bytes}/{total_bytes} ({eta})")
                .progress_chars("#>-"),
        );

        for file in &files {
            if !file.shared {
                self.gdrive.share_file(file.id.as_str())
            }
            pb.inc(1);
        }

        pb.finish_with_message(format!("Shared {} files", files.len()).as_str());
    }

    pub fn scan_folder(&mut self) -> Vec<ParsedFileInfo> {
        let re = Regex::new("%5B[0-9A-Fa-f]{16}%5D").unwrap();

        // Trigger Authentication if needed
        self.gdrive.trigger_auth();

        let pb = ProgressBar::new(!0);
        pb.enable_steady_tick(130);
        pb.set_style(
            ProgressStyle::default_spinner()
                // For more spinners check out the cli-spinners project:
                // https://github.com/sindresorhus/cli-spinners/blob/master/spinners.json
                .tick_strings(&["-", "\\", "|", "/"])
                .template("{spinner:.blue} {msg}"),
        );
        pb.set_message("Scanning...");

        let files: Vec<ParsedFileInfo> = self
            .gdrive
            .get_all_files_in_folder(&self.input.folder_id, !self.input.no_recursion)
            .into_iter()
            .map(|file_info| ParsedFileInfo::new(file_info))
            .filter(|file| {
                let mut keep = true;

                if !self.input.add_non_nsw_files {
                    let extension: String = file
                        .name
                        .chars()
                        .skip(file.name.len() - 4)
                        .take(4)
                        .collect();

                    keep = vec![".nsp", ".nsz", ".xci", ".xcz"].contains(&&*extension);
                }

                if !self.input.add_nsw_files_without_title_id {
                    keep = re.is_match(file.name_encoded.as_str());
                }

                keep
            })
            .collect();

        pb.finish_with_message(&*format!("Scanned {} files", files.len()));

        files
    }
}

pub fn main() {
    match real_main() {
        true => std::process::exit(0),
        false => std::process::exit(1),
    }
}

fn real_main() -> bool {
    // TODO: do validate checks before or move gdrive hub construction to later point so it doesn't trigger panics when credentials are missing
    let mut service = RustfoilService::new(Input::from_args());

    if !service.validate_input() {
        return false;
    }

    let files = service.scan_folder();

    let index = service.generate_index(files.to_owned());

    service.output_index(*index);

    if service.input.share_files {
        service.share_files(files);
    }

    true
}
