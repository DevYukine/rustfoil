#[derive(Debug, Clone)]
pub enum EncryptionFlag {
    NoEncrypt = 0x00,
    Encrypt = 0xF0,
}
