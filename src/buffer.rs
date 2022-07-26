use crate::document as doc;

use ropey::Rope;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum BufError {
    #[error("Rope failure: {0:?}")]
    Rope(ropey::Error),
}
impl From<ropey::Error> for BufError {
    fn from(err: ropey::Error) -> BufError {
        BufError::Rope(err)
    }
}

type Result<T> = std::result::Result<T, BufError>;

enum InsertType {
    Char,
    String,
}

/// ┌─ Buffer Structure ───────────────────┐
/// │ Handles the internal text management │
/// │   of the Document structure.         │
/// │ ┌──────┐    ┌────┐                   │
/// │ │Buffer├───►│Rope│                   │
/// │ └──────┘    └────┘                   │
/// └──────────────────────────────────────┘
pub struct Buffer {
    text: Rope,
}

impl Buffer {
    pub fn new() -> Self {
        Self { text: Rope::new() }
    }

    pub fn open(content: &str) -> Self {
        let text: Rope = Rope::from_str(content);
        Self {
            text,
        }
    }

    pub fn line_count(&self) -> usize {
        self.text.len_lines()
    }

    pub fn insert(&mut self, c: InsertType, pos: &doc::CursorPosition) {

    }
}
