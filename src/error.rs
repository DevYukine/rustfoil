use thiserror::Error;

#[derive(Debug, Error)]
pub enum RustfoilError {
    #[error("Credentials file does not exist!")]
    CredentialsFileMissing,

    #[error("Public key file does not exist!")]
    PubkeyFileMissing,
}
