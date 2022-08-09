mod command;
mod keybind;

use thiserror::Error;

use crate::terminal::{Event, TermError, Terminal};

use self::command::{Command, CommandError};
use self::keybind::{Keybind, Keybinds};

#[derive(Error, Debug)]
pub enum InputError {
    #[error("Invalid key: {0}")]
    Key(Keybind),
    #[error("Err: couldn't open document: {0}")]
    Terminal(TermError),
    #[error("Command Err: couldn't run: {0}")]
    Command(CommandError),
}
impl From<TermError> for InputError {
    fn from(err: TermError) -> InputError {
        InputError::Terminal(err)
    }
}
impl From<CommandError> for InputError {
    fn from(err: CommandError) -> InputError {
        InputError::Command(err)
    }
}

type Result<T> = std::result::Result<T, InputError>;

pub struct Input {
    keybinds: Keybinds,
}

impl Input {
    pub fn new() -> Self {
        Self {
            keybinds: keybind::set_default_keybind(),
        }
    }

    pub fn input_handler(&mut self, term: &mut Terminal) -> Result<()> {
        /* Log Errors */
        Ok(self.read_input(term)?.run()?)
    }

    fn read_input(&mut self, term: &mut Terminal) -> Result<Command> {
        match term.read_event()? {
            Event::Key(key) => self
                .keybinds
                .get(key)
                .ok_or(InputError::Key(Keybind::from(key))),
            _Mouse => Ok(Command::default()),
        }
    }
}
