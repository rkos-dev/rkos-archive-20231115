use thiserror::Error;

#[derive(Error, Debug)]
pub enum FileError {
    #[error("File not found")]
    NotFound(),
}
