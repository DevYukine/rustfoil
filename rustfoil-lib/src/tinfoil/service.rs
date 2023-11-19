use std::io::Write;
use std::path::PathBuf;

use crate::abstraction::file::TinfoilFileLike;
use crate::tinfoil::compression::compression::TinfoilCompression;
use crate::tinfoil::encryption::encryption::TinfoilEncryption;
use crate::tinfoil::model::{TinfoilFile, TinfoilIndex};

pub struct TinfoilService {}

impl TinfoilService {
    pub fn new() -> Self {
        Self {}
    }

    pub fn generate_index<F>(
        &self,
        files: &Vec<F>,
        success: Option<String>,
        referrer: Option<String>,
        min_version: Option<f32>,
        theme_blacklist: Option<Vec<String>>,
        theme_whitelist: Option<Vec<String>>,
        theme_error: Option<String>,
    ) -> anyhow::Result<TinfoilIndex>
    where
        F: TinfoilFileLike,
    {
        let mut index = TinfoilIndex::new();

        index.files = Some(Vec::new());

        for file in files {
            let tinfoil_file = TinfoilFile {
                url: file.get_url(),
                size: file.get_size(),
            };

            index.add_file(tinfoil_file);
        }

        index.success = success;
        index.referrer = referrer;
        index.version = min_version;
        index.theme_black_list = theme_blacklist;
        index.theme_white_list = theme_whitelist;
        index.theme_error = theme_error;

        Ok(index)
    }

    pub async fn generate_index_file(
        &self,
        index_file: TinfoilIndex,
        compression: TinfoilCompression,
        encryption: TinfoilEncryption,
        encryption_key: Option<PathBuf>,
    ) -> anyhow::Result<Vec<u8>> {
        let data = serde_json::to_vec(&index_file)?;

        let mut data = compression.compress(data.as_slice())?;
        let data_length = data.len();

        let session_key = match encryption {
            TinfoilEncryption::NoEncrypt => b"\x00".repeat(0x100),
            TinfoilEncryption::Encrypt => {
                let encryption_key_path = match encryption_key {
                    None => {
                        return Err(anyhow::anyhow!("Encryption Key not provided"));
                    }
                    Some(key_path) => key_path,
                };

                let (encrypted_data, encryption_key) =
                    encryption.encrypt(data, &encryption_key_path).await?;

                data = encrypted_data;
                encryption_key
            }
        };

        let mut bytes = Vec::new();

        let flag = (compression as u8) | (encryption as u8);

        bytes.write(b"TINFOIL")?;
        bytes.write(&flag.to_le_bytes())?;
        bytes.write(&session_key)?;
        bytes.write(&data_length.to_le_bytes())?;
        bytes.write(&data)?;

        Ok(bytes)
    }
}
