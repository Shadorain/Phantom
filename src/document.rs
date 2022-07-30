use thiserror::Error;
use std::{io, path::Path, fs};
use crate::buffer::{ self, Buffer };

#[derive(Error, Debug)]
pub enum DocError {
    #[error("Err: couldn't open document: {0}")]
    Open(String),
    #[error("Err: couldn't close document: {0}")]
    Close(String),
    #[error("File failure: {0:?}")]
    File(io::Error),
    #[error("Buffer failure: {0:?}")]
    Buffer(buffer::BufError),
}
impl From<buffer::BufError> for DocError {
    fn from(err: buffer::BufError) -> DocError {
        DocError::Buffer(err)
    }
}

type Result<T> = std::result::Result<T, DocError>;

#[derive(Default, Clone)]
pub struct Position {
    pub row: usize,
    pub col: usize,
}

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
    pos: Position,
    buf: Buffer,
}

impl Document {
    pub fn new() -> Self {
        Self {
            file_name: None,
            pos: Position::default(),
            buf: Buffer::new(),
        }
    }

    pub fn open(file_name: &str) -> Result<Self> {
        if !Document::is_valid_file(file_name) { return Err(DocError::Open(file_name.to_owned())); } 
        
        Ok(Self {
            file_name : Some(file_name.to_owned()),
            pos: Position::default(),
            buf: Buffer::open(&Self::get_file_contents(file_name)?),
        })
    }

    fn is_valid_file(file_name: &str) -> bool {
        Path::new(file_name).is_file()
    }

    fn get_file_contents(file_name: &str) -> Result<String> {
        fs::read_to_string(file_name).map_err(DocError::File)
    }
}
