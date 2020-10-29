use crate::compression::CompressionFlag;
use crate::encryption::EncryptionFlag;
use crate::result;
use std::borrow::Borrow;
use std::io::Write;
use std::path::{Path, PathBuf};

pub fn convert_to_tinfoil_format(
    json: &str,
    compression: CompressionFlag,
    encryption: EncryptionFlag,
    encryption_file_path_buf: Option<PathBuf>,
) -> result::Result<Vec<u8>> {
    let mut data = compression.compress(json)?;

    let session_key = match encryption {
        EncryptionFlag::NoEncrypt => Some(b"\x00".repeat(0x100)),
        EncryptionFlag::Encrypt => {
            let (encrypted_data, encryption_key) =
                encryption.encrypt(data, encryption_file_path_buf.unwrap().as_path())?;

            data = encrypted_data;
            Some(encryption_key)
        }
    };

    let mut bytes = Vec::new();

    let flag = (compression as u8) | (encryption as u8);
    let data_length = data.len();

    bytes.write(b"TINFOIL")?;
    bytes.write(flag.to_le_bytes().borrow())?;
    bytes.write(session_key.unwrap().as_slice())?;
    bytes.write(data_length.to_le_bytes().borrow())?;
    bytes.write(data.borrow())?;
    bytes.write(b"\x00".repeat(0x10 - (data_length % 0x10)).as_slice())?;

    Ok(bytes)
}
