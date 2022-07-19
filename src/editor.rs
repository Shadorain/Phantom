use std::env;

use crate::terminal::Terminal;
use crate::document::Document;

pub struct Editor {
    term: Terminal,
    documents: Vec<Document>,
}

impl Editor {
    pub fn new() -> Self {
        let args: Vec<String> = env::args().collect();

        let mut doc_vec: Vec<Document> = Vec::new();
        doc_vec.push(if let Some(file_name) = args.get(1) {
            let doc = Document::open(file_name);
            if let Ok(doc) = doc {
                doc
            } else {
                println!("{0}", doc.is_err());
                Document::default()
            }
        } else { Document::default() });
        
        Self {
            term: Terminal::new().expect("Failed to initialize terminal"),
            documents: doc_vec,
        }
    }
}
