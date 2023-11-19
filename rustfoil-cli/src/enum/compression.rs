use clap::ValueEnum;
use rustfoil_lib::tinfoil::compression::compression::TinfoilCompression;
use strum::Display;

#[derive(Debug, Display, Clone, Copy, ValueEnum)]
pub enum Compression {
    OFF,
    ZSTD,
    ZLIB,
}

impl Into<TinfoilCompression> for Compression {
    fn into(self) -> TinfoilCompression {
        match &self {
            Compression::OFF => TinfoilCompression::Off,
            Compression::ZSTD => TinfoilCompression::ZSTD,
            Compression::ZLIB => TinfoilCompression::Zlib,
        }
    }
}
