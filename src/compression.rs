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

    fn from_str(compression: &str) -> Result<Self, Self::Err> {
        match compression.to_lowercase().as_ref() {
            "off" => Ok(CompressionFlag::Off),
            "zstd" => Ok(CompressionFlag::ZSTD),
            "zlib" => Ok(CompressionFlag::Zlib),
            _ => Err(format!("Invalid compression: {}", compression)),
        }
    }
}

impl CompressionFlag {
    pub fn compress(&self, data: &str) -> Vec<u8> {
        match &self {
            CompressionFlag::Off => data.as_bytes().to_vec(),
            CompressionFlag::ZSTD => zstd::block::compress(data.as_bytes(), 22).unwrap().clone(),
            CompressionFlag::Zlib => {
                let mut encoder = ZlibEncoder::new(Vec::new(), Compression::best());
                encoder.write_all(data.as_ref());
                encoder.finish().unwrap().clone()
            }
        }
    }
}
