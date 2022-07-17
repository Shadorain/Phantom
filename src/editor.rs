use std::env;

use crate::terminal::Terminal;
use crate::document::Document;

pub struct Editor {
    term: Terminal,
    buffers: Vec<Document>,
}

impl Editor {
    pub fn new() -> Self {
        // let args: Vec<String> = env::args().collect();
        // let document = if let Some(file_name) = args.get(1) {
        //     let doc = Document::open(file_name);
        // }
        
        Self {
            term: Terminal::new().expect("Failed to initialize terminal"),
            buffers: Vec::new().push(Document::new(file_name)),
        }
    }
}
