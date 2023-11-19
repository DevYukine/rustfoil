use crate::gdrive::model::GoogleDriveTokenInfo;
use crate::tinfoil::model::TinfoilToken;
use std::path::PathBuf;
use tokio::fs;

pub async fn copy_tinfoil_auth_files(
    output_path: PathBuf,
    token_path: PathBuf,
    credentials_path: PathBuf,
) -> anyhow::Result<()> {
    let token_file_raw = fs::read(token_path).await?;

    let mut token_file: Vec<GoogleDriveTokenInfo> =
        serde_json::from_slice(token_file_raw.as_slice())?;

    let token = token_file.remove(0).token;

    let tinfoil_auth = TinfoilToken {
        access_token: token.access_token,
        refresh_token: token.refresh_token,
    };

    fs::create_dir_all(&output_path).await?;

    let mut tinfoil_token_path = output_path.clone();

    tinfoil_token_path.push("gdrive.token");

    fs::write(tinfoil_token_path, serde_json::to_string(&tinfoil_auth)?).await?;

    let mut tinfoil_credentials_path = output_path.clone();

    tinfoil_credentials_path.push(&credentials_path);

    fs::copy(credentials_path, tinfoil_credentials_path).await?;

    Ok(())
}
