use std::path::PathBuf;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum ZackstrapError {
    #[error("Directory not found: {0}")]
    DirectoryNotFound(PathBuf),

    #[error("Path is not a directory: {0}")]
    NotADirectory(PathBuf),

    #[error("Failed to read directory: {0}")]
    ReadDirError(#[from] std::io::Error),

    #[error("Failed to write file {0}: {1}")]
    WriteFileError(PathBuf, #[source] std::io::Error),

    #[error("File already exists and force flag not set: {0}")]
    FileExists(PathBuf),

    #[error("Failed to serialize configuration: {0}")]
    SerializationError(#[from] serde_json::Error),

    #[error("Failed to determine current directory")]
    CurrentDirError(#[from] std::env::VarError),
}
