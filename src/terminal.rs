use thiserror::Error;
use std::io::{ Stdout, stdout, self, Write };
use crossterm::{ cursor as c, execute, queue, terminal as t, style as s,
    event as e, ErrorKind };

#[derive(Error, Debug)]
pub enum TermError {
    #[error("Failure to startup terminal")]
    Startup,
    #[error("Failure to cleanup terminal")]
    Cleanup,
    #[error("Failure to run command: {0:?}")]
    Command(io::Error),
}
impl From<ErrorKind> for TermError {
    fn from(err: ErrorKind) -> TermError {
        TermError::Command(err)
    }
}

pub struct Rgb {
    r: u8,
    g: u8,
    b: u8,
}
impl From<Rgb> for s::Color {
    fn from(rgb: Rgb) -> s::Color {
        let Rgb { r, g, b } = rgb;
        s::Color::Rgb { r, g, b }
    }
}

/// Event is a wrapper around crossterm's Event enum
/// Note: Resize taken out because it should be handled immediately
pub enum Event {
    Key(e::KeyEvent),
    Mouse(e::MouseEvent),
    /* Resize(crossterm::event::Resize), */
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
        let size = t::size()?;
        Ok(Self {
            size: Size { cols: size.0, rows: size.1 },
            _stdout: stdout(),
        })
    }

    pub fn flush(&mut self) -> Result<()> {
        Ok(self._stdout.flush()?)
    }


    /* Read ----------------------------------------------------------------- */

    /// Reads and returns an Event from the terminal
    /// Handles resize events automatically
    pub fn read_event(&mut self) -> Result<Event> {
        loop {
            match e::read()? {
                e::Event::Key(key) => return Ok(Event::Key(key)),
                e::Event::Mouse(mouse) => return Ok(Event::Mouse(mouse)),
                e::Event::Resize(x, y) => {
                    self.size = Size { cols: x, rows: y }
                },
                _ => todo!(),
            }
        }
    }

    /* Cursor --------------------------------------------------------------- */

    pub fn cursor_hide(&mut self) -> Result<&mut Self> {
        queue!(self._stdout,
            c::Hide
        )?;
        Ok(self)
    }
    pub fn cursor_show(&mut self) -> Result<&mut Self> {
        queue!(self._stdout,
            c::Show
        )?;
        Ok(self)
    }

    pub fn cursor_position(&mut self, col: u16, row: u16) -> Result<&mut Self> {
        queue!(self._stdout,
            c::MoveTo(col, row)
        )?;
        Ok(self)
    }


    /* Display -------------------------------------------------------------- */

    pub fn clear_screen(&mut self) -> Result<&mut Self> {
        queue!(self._stdout,
            t::Clear(t::ClearType::All)
        )?;
        Ok(self)
    }
    pub fn clear_line(&mut self) -> Result<&mut Self> {
        queue!(self._stdout,
            t::Clear(t::ClearType::CurrentLine)
        )?;
        Ok(self)
    }


    /* Coloring ------------------------------------------------------------- */

    pub fn fg_set(&mut self, rgb: Rgb) -> Result<&mut Self> {
        queue!(self._stdout,
            s::SetForegroundColor(rgb.into())
        )?;
        Ok(self)
    }
    pub fn bg_set(&mut self, rgb: Rgb) -> Result<&mut Self> {
        queue!(self._stdout,
            s::SetBackgroundColor(rgb.into())
        )?;
        Ok(self)
    }
    pub fn fg_reset(&mut self) -> Result<&mut Self> {
        queue!(self._stdout,
            s::SetForegroundColor(s::Color::Reset)
        )?;
        Ok(self)
    }
    pub fn bg_reset(&mut self) -> Result<&mut Self> {
        queue!(self._stdout,
            s::SetBackgroundColor(s::Color::Reset)
        )?;
        Ok(self)
    }


    /* Window Management ---------------------------------------------------- */

    pub fn size_get(&self) -> &Size {
        &self.size
    }
    pub fn size_set(&mut self, size: Size) -> Result<()> {
        self.size = size;
        Ok(queue!(self._stdout,
            t::SetSize(self.size.cols, self.size.rows)
        )?)
    }


    /* Init / Deinit -------------------------------------------------------- */

    pub fn startup(&mut self) -> Result<()> {
        execute!(self._stdout,
            t::EnterAlternateScreen,
            s::ResetColor,
            t::Clear(t::ClearType::All),
            c::MoveTo(0, 0),
        ).map_err(|_| TermError::Startup)?;

        t::enable_raw_mode().map_err(|_| TermError::Startup)
    }
    pub fn cleanup(&mut self) -> Result<()> {
        execute!(self._stdout,
            s::ResetColor,
            c::Show,
            t::LeaveAlternateScreen
        ).map_err(|_| TermError::Cleanup)
    }
}
