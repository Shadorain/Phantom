use std::io::ErrorKind;

use thiserror::Error;
use ropey::Rope;

#[derive(Error, Debug)]
pub enum DocError {
    #[error("Failure to open document")]
    Open,
    #[error("Failure to close document")]
    Close,
    #[error("Rope failure: {0:?}")]
    Rope(ropey::Error),
}
impl From<ropey::Error> for DocError {
    fn from(err: ropey::Error) -> DocError {
        DocError::Rope(err)
    }
}

type Result<T> = std::result::Result<T, DocError>;

pub struct Document {
    pub file_name: Option<String>,
    text: Rope,
}

impl Document {
    pub fn open(file_name: &str) -> Result<Self> {
        Ok(Self { file_name, text: Rope::new() })
    }
}
