use thiserror::Error;
use std::io::{ Stdout, stdout, self, Write };
use crossterm::{ cursor as c, execute, queue, terminal as t, style as s };

pub struct Rgb {
    r: u8,
    g: u8,
    b: u8,
}

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
        let size = t::size().map_err(|_| TermError::Size)?;
        Ok(Self {
            size: Size { cols: size.0, rows: size.1 },
            _stdout: stdout(),
        })
    }

    pub fn flush(&mut self) -> Result<()> {
        self._stdout.flush().map_err(TermError::Command)
    }
    /* Cursor */
    pub fn cursor_hide(&mut self) -> Result<()> {
        queue!(self._stdout,
            c::Hide
        ).map_err(TermError::Command)
    }
    pub fn cursor_show(&mut self) -> Result<()> {
        queue!(self._stdout,
            c::Show
        ).map_err(TermError::Command)
    }

    pub fn cursor_position(&mut self, col: u16, row: u16) -> Result<()> {
        queue!(self._stdout,
            c::MoveTo(col, row)
        ).map_err(TermError::Command)
    }

    /* Display */
    pub fn clear_screen(&mut self) -> Result<()> {
        queue!(self._stdout,
            t::Clear(t::ClearType::All)
        ).map_err(TermError::Command)
    }
    pub fn clear_line(&mut self) -> Result<()> {
        queue!(self._stdout,
            t::Clear(t::ClearType::CurrentLine)
        ).map_err(TermError::Command)
    }

    /* Coloring */
    pub fn fg_set(&mut self, rgb: Rgb) -> Result<()> {
        queue!(self._stdout,
            s::SetForegroundColor(s::Color::Rgb { r: rgb.r, g: rgb.g, b: rgb.b })
        ).map_err(TermError::Command)
    }
    pub fn bg_set(&mut self, rgb: Rgb) -> Result<()> {
        queue!(self._stdout,
            s::SetBackgroundColor(s::Color::Rgb { r: rgb.r, g: rgb.g, b: rgb.b })
        ).map_err(TermError::Command)
    }
    pub fn fg_reset(&mut self) -> Result<()> {
        queue!(self._stdout,
            s::SetForegroundColor(s::Color::Reset)
        ).map_err(TermError::Command)
    }
    pub fn bg_reset(&mut self) -> Result<()> {
        queue!(self._stdout,
            s::SetBackgroundColor(s::Color::Reset)
        ).map_err(TermError::Command)
    }

    /* Window Management */
    pub fn size_get(&self) -> &Size {
        &self.size
    }
    pub fn size_set(&mut self, size: Size) -> Result<()> {
        self.size = size;
        queue!(self._stdout,
            t::SetSize(self.size.cols, self.size.rows)
        ).map_err(|_| TermError::Size)
    }

    /* Init / Deinit */
    pub fn startup(&mut self) -> Result<()> {
        queue!(self._stdout,
            c::MoveTo(0, 0),
        ).map_err(|_| TermError::Startup)
    }
    pub fn cleanup(&mut self) -> Result<()> {
        execute!(self._stdout,
            s::ResetColor,
            c::Show,
            t::LeaveAlternateScreen
        ).map_err(|_| TermError::Cleanup)
    }
}
