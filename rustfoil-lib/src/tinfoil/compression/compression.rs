use flate2::write::ZlibEncoder;
use flate2::Compression;
use std::io::Write;

#[derive(Debug, Clone, Copy)]
pub enum TinfoilCompression {
    Off = 0x00,
    ZSTD = 0x0D,
    Zlib = 0x0E,
}

impl TinfoilCompression {
    pub fn compress(&self, data: &[u8]) -> anyhow::Result<Vec<u8>> {
        match &self {
            TinfoilCompression::Off => Ok(data.to_vec()),
            TinfoilCompression::ZSTD => Ok(zstd::bulk::compress(data, 22)?),
            TinfoilCompression::Zlib => {
                let mut encoder = ZlibEncoder::new(Vec::new(), Compression::best());
                encoder.write_all(data)?;
                Ok(encoder.finish()?)
            }
        }
    }
}
