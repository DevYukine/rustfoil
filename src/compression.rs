use std::str::FromStr;
use structopt::StructOpt;

#[derive(StructOpt, Debug)]
pub enum Compression {
    Off,
    ZSTD,
    Zlib,
}

impl FromStr for Compression {
    type Err = String;

    fn from_str(compression: &str) -> Result<Self, Self::Err> {
        match compression.to_lowercase().as_ref() {
            "off" => Ok(Compression::Off),
            "zstd" => Ok(Compression::ZSTD),
            "zlib" => Ok(Compression::Zlib),
            _ => Err(format!("Invalid compression: {}", compression)),
        }
    }
}

impl Compression {
    pub fn compress(&self, data: &str) -> Vec<u8> {
        match &self {
            Compression::Off => data.as_bytes().to_vec(),
            Compression::ZSTD => zstd::block::compress(data.as_bytes(), 22).unwrap().clone(),
            Compression::Zlib => {
                let mut encoder = ZlibEncoder::new(Vec::new(), flate2::Compression::best());
                encoder.write_all(data.as_ref());
                encoder.finish().unwrap().clone()
            }
        }
    }
}
