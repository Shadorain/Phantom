use thiserror::Error;
use std::io::{ Stdout, stdout, self, Write };
use crossterm::{ cursor, execute, queue, terminal::{self, SetSize} };

#[derive(Error, Debug)]
pub enum TermError {
    #[error("Terminal size failure")]
    Size,
    #[error("Failure to startup terminal")]
    Startup,
    #[error("Failure to cleanup terminal")]
    Cleanup,
    #[error("Failure to run command")]
    Command(io::Error),
}

#[derive(Clone, Copy)]
pub struct Size {
    cols: u16,
    rows: u16,
}

pub struct Terminal {
    size: Size,
    _stdout: Stdout,
}

type Result<T> = std::result::Result<T, TermError>;

impl Terminal {
    pub fn new() -> Result<Self> {
        let size = terminal::size().map_err(|_e| TermError::Size).unwrap();
        Ok(Self {
            size: Size { cols: size.0, rows: size.1 },
            _stdout: stdout(),
        })
    }

    pub fn flush(&mut self) -> Result<()> {
        self._stdout.flush().map_err(TermError::Command)
    }

    /* Cursor */

    /* Display */

    /* Window Management */
    pub fn size_get(&self) -> &Size {
        &self.size
    }
    pub fn size_set(&mut self, size: Size) -> Result<()> {
        self.size = size;
        queue!(self._stdout, SetSize(self.size.cols, self.size.rows)).map_err(|_e| TermError::Size)
    }

    /* Init / Deinit */
    pub fn startup(&mut self) -> Result<()> {
        queue!(self._stdout, cursor::MoveTo(0, 0), ).map_err(|_e| TermError::Startup)
    }
    pub fn cleanup(&mut self) -> Result<()> {
        execute!(self._stdout, SetSize(self.size.cols, self.size.rows)).map_err(|_e| TermError::Cleanup)
    }
}
