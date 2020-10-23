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
