use std::env;

use crossterm::event::KeyEvent;
use thiserror::Error;

use crate::terminal::{Terminal, TermError, Event};
use crate::document::{Document, DocError};
use crate::command::EditCommand;

#[derive(Error, Debug)]
pub enum EditorError {
    #[error("Err: couldn't open document: {0}")]
    Terminal(TermError), /* -> ScreenError when implemented */
    #[error("Err: couldn't close document: {0}")]
    Document(DocError),
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

type Result<T> = std::result::Result<T, EditorError>;

pub struct Editor {
    term: Terminal,
    documents: Vec<Document>,
}

impl Editor {
    /// Creates a new Editor instance.
    pub fn new() -> Self {
        let args: Vec<String> = env::args().collect();

        let mut doc_vec: Vec<Document> = Vec::new();
        doc_vec.push(if let Some(file_name) = args.get(1) {
            let doc = Document::open(file_name);
            if let Ok(doc) = doc {
                doc
            } else {
                println!("{0}", doc.is_err());
                Document::new()
            }
        } else { Document::new() });
        
        Self {
            term: Terminal::new().expect("Failed to initialize terminal"),
            documents: doc_vec,
        }
    }

    /// Main loop of the editor.
    pub fn run(&mut self) -> Result<()> {
        loop {
            // self.term.clear();
            self.refresh()?;
            self.read_input()?;
        }
    }

    fn read_input(&mut self) -> Result<()> {
        let e = self.term.read_event()?;
        match e {
            Event::Key(key) => {},
            Event::Mouse(_) => {},
        }
        Ok(())
    }

    /// Will move to screen.refresh
    fn refresh(&mut self) -> Result<()> {
        unimplemented!();
    }
}
