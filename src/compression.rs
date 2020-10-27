use crate::result::Result;
use core::fmt;
use flate2::write::ZlibEncoder;
use flate2::Compression;
use std::io::Write;
use std::str::FromStr;
use structopt::StructOpt;

#[derive(StructOpt, Debug, Clone)]
pub enum CompressionFlag {
    Off = 0x00,
    ZSTD = 0x0D,
    Zlib = 0x0E,
}

impl FromStr for CompressionFlag {
    type Err = String;

    fn from_str(compression: &str) -> std::result::Result<Self, Self::Err> {
        match compression.to_lowercase().as_ref() {
            "off" => Ok(CompressionFlag::Off),
            "zstd" => Ok(CompressionFlag::ZSTD),
            "zlib" => Ok(CompressionFlag::Zlib),
            _ => Err(format!("Invalid compression: {}", compression)),
        }
    }
}

impl fmt::Display for CompressionFlag {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}",
            match self {
                CompressionFlag::Off => "off".to_string(),
                CompressionFlag::ZSTD => "zstd".to_string(),
                CompressionFlag::Zlib => "zlib".to_string(),
            }
        )
    }
}

impl CompressionFlag {
    pub fn compress(&self, data: &str) -> Result<Vec<u8>> {
        match &self {
            CompressionFlag::Off => Ok(data.as_bytes().to_vec()),
            CompressionFlag::ZSTD => Ok(zstd::block::compress(data.as_bytes(), 22)?.clone()),
            CompressionFlag::Zlib => {
                let mut encoder = ZlibEncoder::new(Vec::new(), Compression::best());
                encoder.write_all(data.as_ref())?;
                Ok(encoder.finish()?.clone())
            }
        }
    }
}
