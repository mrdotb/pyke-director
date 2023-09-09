use reqwest;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum RecordingError {
    #[error("network error: {0}")]
    NetworkError(#[from] reqwest::Error),

    #[error("IO error occurred: {0}")]
    Io(#[from] std::io::Error),

    #[error("failed to unwrap Arc")]
    ArcUnwrapError,
}
