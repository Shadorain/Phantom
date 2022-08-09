use std::env;
use thiserror::Error;

use crate::input::{Input, InputError};
use crate::terminal::{Terminal, TermError};
use crate::document::{Document, DocError};

#[derive(Error, Debug)]
pub enum EditorError {
    #[error("Terminal Err: {0}")]
    Terminal(TermError), /* -> ScreenError when implemented */
    #[error("Document Err: {0}")]
    Document(DocError),
    #[error("Input Err: {0}")]
    Input(InputError),
}
impl From<TermError> for EditorError {
    fn from(err: TermError) -> EditorError {
        EditorError::Terminal(err)
    }
}
impl From<DocError> for EditorError {
    fn from(err: DocError) -> EditorError {
        EditorError::Document(err)
    }
}
impl From<InputError> for EditorError {
    fn from(err: InputError) -> EditorError {
        EditorError::Input(err)
    }
}

type Result<T> = std::result::Result<T, EditorError>;

pub struct Editor {
    term: Terminal,
    documents: Vec<Document>,
    input: Input,
}

impl Editor {
    /// Creates a new Editor instance.
    pub fn new() -> Result<Self> {
        let args: Vec<String> = env::args().collect();

        let mut doc_vec: Vec<Document> = Vec::new();
        doc_vec.push(if let Some(file_name) = args.get(1) {
            Document::open(file_name)?
        } else { Document::new() });

        Ok(Self {
            term: Terminal::new()?,
            documents: doc_vec,
            input: Input::new(),
        })
    }

    /// Main loop of the editor.
    pub fn run(&mut self) -> Result<()> {
        loop {
            // self.term.clear();
            self.refresh()?;
            self.input.input_handler(&mut self.term)?;
        }
    }

    /// Will move to `screen.refresh`
    fn refresh(&mut self) -> Result<()> {
        self.term.cursor_hide()?;

        Ok(())
    }

    fn clear_screen(&mut self) -> Result<()> {
        Ok(self.term.clear_screen()?.cursor_position(0, 0)?.flush()?)
    }
}
