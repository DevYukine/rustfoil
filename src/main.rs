#[cfg_attr(test, macro_use)]
extern crate structopt;

use crate::index::ParsedFileInfo;
use compression::Compression;
use gdrive::GDriveService;
use index::FileEntry;
use index::Index;
use indicatif::{ProgressBar, ProgressStyle};
use logging::Logger;
use regex::Regex;
use std::path::PathBuf;
use structopt::StructOpt;

mod compression;
mod gdrive;
mod index;
mod logging;
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

    /// Path to RSA Public Key to encrypt AES-ECB-256 key with
    #[structopt(long)]
    public_key: Option<PathBuf>,

    /// Shares the index file that is uploaded to Google Drive
    #[structopt(long)]
    share_index: bool,

    /// Which compression should be used for the index file
    #[structopt(long, default_value = "zstd")]
    compression: Compression,

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

    pub fn generate_index(&mut self) -> Index {
        let mut index = Index::new();

        let files = self.scan_folder();

        for info in files {
            index.files.push(FileEntry::new(
                format!("gdrive:{}#{}", info.id, info.name_encoded),
                u64::from_str_radix(&*info.size, 10).unwrap(),
            ));
        }

        self.logger.log_info("Added files to index");

        if self.input.success.is_some() {
            index.success = self.input.success.clone().unwrap();
            self.logger.log_info("Added success message to index")
        }

        self.logger.log_info("Generated index successfully");

        index
    }

    pub fn output_index(&self, index: Index) {
        let json = serde_json::to_string(&index).unwrap();

        std::fs::write(
            &self.input.output_path,
            &self.input.compression.compress(json.as_str()),
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

    fn scan_folder(&mut self) -> Vec<ParsedFileInfo> {
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
    let mut service = RustfoilService::new(Input::from_args());

    if !service.validate_input() {
        return false;
    }

    let index = service.generate_index();

    service.output_index(index);

    true
}
