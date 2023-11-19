mod cli;
mod r#enum;

use crate::cli::model::IndexCommand;
use crate::r#enum::compression::Compression;
use clap::Parser;
use cli::model::{Cli, Commands, GoogleDriveCommand, HttpCommand};
use env_logger::Env;
use hhmmss::Hhmmss;
use log::{debug, info, warn};
use rustfoil_lib::abstraction::file::TinfoilFileLike;
use rustfoil_lib::filter::file::filter_files;
use rustfoil_lib::fs::file::read_files_recursive;
use rustfoil_lib::gdrive::gdrive_api::GoogleDriveApiService;
use rustfoil_lib::gdrive::service::GoogleDriveService;
use rustfoil_lib::http::model::HttpFile;
use rustfoil_lib::tinfoil::auth::copy_tinfoil_auth_files;
use rustfoil_lib::tinfoil::encryption::encryption::TinfoilEncryption;
use rustfoil_lib::tinfoil::service::TinfoilService;
use tokio::time::Instant;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    env_logger::Builder::from_env(Env::default().default_filter_or("info")).init();
    let timer = Instant::now();
    let cli = Cli::parse();

    match cli.command {
        Commands::Gdrive(cmd) => gdrive(cmd).await?,
        Commands::Http(cmd) => http(cmd).await?,
    }

    info!("Execution took {}", timer.elapsed().hhmmss());

    Ok(())
}

async fn gdrive(command: GoogleDriveCommand) -> anyhow::Result<()> {
    info!(
        "Generating Index for {} Google Drive Folders",
        command.folder_ids.len()
    );

    debug!("Folder IDs: {:?}", command.folder_ids);

    let tinfoil_service = TinfoilService::new();
    let gdrive_api_service = GoogleDriveApiService::new(
        command.credentials.clone(),
        command.token.clone(),
        command.headless,
    )
    .await?;

    let gdrive_service = GoogleDriveService::new(gdrive_api_service);

    info!("Scanning Google Drive... this may take a while");

    let scan = gdrive_service
        .scan_folders(command.folder_ids.clone(), command.no_recursion.clone())
        .await?;

    let scan_files_length = scan.files.len();

    info!(
        "Scanned a total of {} file(s) & {} folder(s)",
        scan_files_length,
        scan.folders.len()
    );

    build_and_write_index(
        command.clone(),
        &tinfoil_service,
        scan.files.clone(),
        scan_files_length,
    )
    .await?;

    if command.share_files {
        for file in &scan.files {
            gdrive_service.share_file(&file.id, file.shared).await?;
        }

        warn!("Consider switching to share-folders if you want faster sharing");

        info!("Shared {} files", scan.files.len());
    }

    if command.share_folders {
        for folder in &scan.folders {
            gdrive_service
                .share_folder(&folder.id, folder.shared)
                .await?;
        }

        info!("Shared {} folders", scan.folders.len());
    }

    if command.upload_my_drive || command.upload_folder_id.is_some() {
        let folder_name = match &command.upload_folder_id {
            None => "My Drive".to_string(),
            Some(id) => id.clone(),
        };

        let file = gdrive_service
            .upload_index(command.output_path, command.upload_folder_id)
            .await?;

        info!("Uploaded Index to {}", folder_name);

        if command.share_index {
            gdrive_service.share_index(&file.id).await?;

            info!(
                "Shared Index File, accessible at https://drive.google.com/uc?id={}",
                &file.id
            );
        }
    }

    if command.tinfoil_auth {
        copy_tinfoil_auth_files(
            command.tinfoil_auth_path.clone(),
            command.token,
            command.credentials,
        )
        .await?;

        info!(
            "Copied tinfoil OAuth files to {:?}",
            command.tinfoil_auth_path
        );
    }

    Ok(())
}

async fn http(command: HttpCommand) -> anyhow::Result<()> {
    info!(
        "Generating Index for {} Local Folders",
        command.folder_paths.len()
    );

    debug!("Folder Paths: {:?}", command.folder_paths);

    let tinfoil_service = TinfoilService::new();

    info!("Scanning Local Folders... this may take a while");

    let mut files = Vec::new();

    for folder_path in &command.folder_paths {
        let folder_files = read_files_recursive(folder_path).await?;

        let mut http_files: Vec<HttpFile> = folder_files
            .iter()
            .map(|local| {
                HttpFile::from_local_with_base_url(
                    command.http_base_url.as_str(),
                    folder_path,
                    local,
                )
                .unwrap()
            })
            .collect();

        files.append(&mut http_files)
    }

    info!(
        "Scanned a total of {} file(s) in {} folders",
        files.len(),
        command.folder_paths.len()
    );

    let file_length = files.len();

    build_and_write_index(command, &tinfoil_service, files, file_length).await?;

    Ok(())
}

async fn build_and_write_index<F, C>(
    command: C,
    tinfoil_service: &TinfoilService,
    files: Vec<F>,
    file_count: usize,
) -> anyhow::Result<()>
where
    F: TinfoilFileLike,
    C: IndexCommand,
{
    let filtered = filter_files(
        files,
        command.add_non_nsw_files(),
        command.add_nsw_files_without_title_id(),
    );

    debug!("{} of {} passed filters", filtered.len(), file_count);

    let index = tinfoil_service.generate_index(
        &filtered,
        command.success(),
        command.referrer(),
        command.min_version(),
        command.theme_blacklist(),
        command.theme_whitelist(),
        command.theme_error(),
    )?;

    let encryption = match command.encrypt() {
        true => TinfoilEncryption::Encrypt,
        false => TinfoilEncryption::NoEncrypt,
    };

    let compression = command.compression().unwrap_or(Compression::OFF);

    let tinfoil_compression = compression.into();

    let file_data = tinfoil_service
        .generate_index_file(index, tinfoil_compression, encryption, command.public_key())
        .await?;

    let output_path = command.output_path();

    let output_dir = match output_path.parent() {
        None => {
            return Err(anyhow::Error::msg("Output path has no parent"));
        }
        Some(dir) => dir,
    };

    tokio::fs::create_dir_all(output_dir).await?;
    tokio::fs::write(&output_path, file_data).await?;

    info!(
        "Index file generated at {:?}, using {} compression & {}encryption",
        &output_path,
        match compression {
            Compression::OFF => "no".to_string(),
            Compression::ZSTD | Compression::ZLIB => {
                compression.to_string()
            }
        },
        match encryption {
            TinfoilEncryption::NoEncrypt => "no ",
            TinfoilEncryption::Encrypt => "",
        }
    );

    Ok(())
}
