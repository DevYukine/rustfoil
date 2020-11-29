use crate::result::Result;
use flate2::write::ZlibEncoder;
use flate2::Compression;
use std::io::Write;
use structopt::clap::arg_enum;
use structopt::StructOpt;

arg_enum! {
    #[derive(StructOpt, Debug, Clone, Copy)]
    pub enum CompressionFlag {
        Off = 0x00,
        ZSTD = 0x0D,
        Zlib = 0x0E,
    }
}

impl CompressionFlag {
    pub fn compress(&self, data: &str) -> Result<Vec<u8>> {
        match &self {
            CompressionFlag::Off => Ok(data.as_bytes().to_vec()),
            CompressionFlag::ZSTD => Ok(zstd::block::compress(data.as_bytes(), 22)?),
            CompressionFlag::Zlib => {
                let mut encoder = ZlibEncoder::new(Vec::new(), Compression::best());
                encoder.write_all(data.as_ref())?;
                Ok(encoder.finish()?)
            }
        }
    }
}
