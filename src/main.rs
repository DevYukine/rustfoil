#[cfg_attr(test, macro_use)]
extern crate structopt;

use crate::gdrive::FileInfo;
use crate::index::FileEntry;
use compression::Compression;
use console::{style, Term};
use gdrive::GDriveService;
use index::Index;
use indicatif::{ProgressBar, ProgressStyle};
use logging::LogLevel::Error;
use logging::Logger;
use percent_encoding::{utf8_percent_encode, AsciiSet, NON_ALPHANUMERIC};
use regex::Regex;
use std::cmp::min;
use std::io::{Empty, ErrorKind, Write};
use std::path::{Path, PathBuf};
use std::str::FromStr;
use std::thread;
use std::time::Duration;
use structopt::StructOpt;

mod compression;
mod gdrive;
mod index;
mod logging;

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
    index: Index,
}

impl RustfoilService {
    pub fn new(input: Input) -> RustfoilService {
        let credentials = input.credentials.clone();
        let token = input.token.clone();
        RustfoilService {
            input,
            logger: Logger::new(),
            gdrive: GDriveService::new(credentials.as_path(), token.as_path()),
            index: Index::new(),
        }
    }

    pub fn validate_input(&self) -> bool {
        if !&self.input.credentials.exists() {
            &self.logger.log_error("Credentials file is missing!");
            false;
        }

        true
    }

    pub fn generate_index(&mut self) {
        let files = self.scan_folder();

        for info in files {
            self.index.files.push(FileEntry::new(
                format!(
                    "gdrive:{}#{}",
                    info.id,
                    utf8_percent_encode(&*info.name, NON_ALPHANUMERIC)
                ),
                u64::from_str_radix(&*info.size, 10).unwrap(),
            ));
        }
    }

    fn scan_folder(&mut self) -> Vec<FileInfo> {
        let re = Regex::new("\\[[0-9A-Fa-f]{16}\\]");

        // Trigger Authentication if needed
        let files = self.gdrive.trigger_auth();

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
        let files = self
            .gdrive
            .get_all_files_in_folder(&self.input.folder_id, !self.input.no_recursion);

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

    service.generate_index();

    true
}
