use crate::compression::CompressionFlag;
use crate::encryption::EncryptionFlag;
use crate::result::Result;
use std::borrow::Borrow;
use std::io::Write;

pub fn convert_to_tinfoil_format(
    data: Vec<u8>,
    compression: CompressionFlag,
    encryption: EncryptionFlag,
) -> Result<Vec<u8>> {
    let mut bytes = Vec::new();

    let flag = (compression as u8) | (encryption as u8);
    let session_key = b"\x00".repeat(0x100);
    let data_length = data.len();

    bytes.write(b"TINFOIL")?;
    bytes.write(flag.to_le_bytes().borrow())?;
    bytes.write(session_key.as_slice())?;
    bytes.write(data_length.to_le_bytes().borrow())?;
    bytes.write(data.borrow())?;
    bytes.write(b"\x00".repeat(0x10 - (data_length % 0x10)).as_slice())?;

    Ok(bytes)
}
