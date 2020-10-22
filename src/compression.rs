use std::str::FromStr;
use structopt::StructOpt;

#[derive(StructOpt, Debug)]
pub enum Compression {
    Off,
    ZStandart,
    ZSTD,
    Zlib,
}

impl FromStr for Compression {
    type Err = String;

    fn from_str(compression: &str) -> Result<Self, Self::Err> {
        match compression.to_lowercase().as_ref() {
            "off" => Ok(Compression::Off),
            "zstandart" => Ok(Compression::ZStandart),
            "zstd" => Ok(Compression::ZSTD),
            "zlib" => Ok(Compression::Zlib),
            _ => Err(format!("Invalid compression: {}", compression)),
        }
    }
}
