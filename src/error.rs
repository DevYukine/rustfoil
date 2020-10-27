use thiserror::Error;

#[derive(Debug, Error)]
pub enum RustfoilError {
    #[error("Credentials file is missing")]
    CredentialsMissing,
}
