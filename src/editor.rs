use crate::terminal::Terminal;

pub struct Editor {
    term: Terminal,
}

impl Editor {
    pub fn new() -> Self {
        let term = Terminal::new().expect("Failed to initialize terminal");
        Self { term }
    }
}
