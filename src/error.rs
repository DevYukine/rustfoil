use thiserror::Error;

#[derive(Debug, Error)]
pub enum RustfoilError {
    #[error("Credentials file does not exist!")]
    CredentialsFileMissing,
}
