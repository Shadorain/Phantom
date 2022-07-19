use thiserror::Error;
use ropey::Rope;

use std::path::Path;

#[derive(Error, Debug)]
pub enum DocError {
    #[error("Err: couldn't open document: {0}")]
    Open(String),
    #[error("Err: couldn't close document: {0}")]
    Close(String),
    #[error("Rope failure: {0:?}")]
    Rope(ropey::Error),
}
impl From<ropey::Error> for DocError {
    fn from(err: ropey::Error) -> DocError {
        DocError::Rope(err)
    }
}

type Result<T> = std::result::Result<T, DocError>;

/// ┌─ Document Structure ─────────────┐
/// │ Displays file contents, internal │
/// │   buffer handles edits.          │
/// │ TODO: Abstract out file IO       │
/// │ ┌────────┐    ┌──────┐    ┌────┐ │
/// │ │Document├───►│Buffer├───►│Rope│ │
/// │ └───┬────┘    └──────┘◄─┐ └────┘ │
/// │     │ ┌─ TODO ─┐   ┌────┴────┐   │
/// │     └►│( File )├──►│BufReader│   │ 
/// │       └────────┘   └─────────┘   │
/// └──────────────────────────────────┘
pub struct Document {
    pub file_name: Option<String>,
    text: Rope,
}

impl Document {
    pub fn default() -> Self {
        Self {
            file_name: None,
            text: Rope::new(),
        }
    }

    pub fn open(file_name: &str) -> Result<Self> {
        if !Document::is_valid_file(file_name) { return Err(DocError::Open(file_name.to_owned())); } 
        Ok(Self {
            file_name : Some(file_name.to_owned()),
            text: Rope::new(),
        })
    }

    pub fn is_valid_file(file_name: &str) -> bool {
        Path::new(file_name).is_file()
    }
}
