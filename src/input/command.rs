use std::fmt;

use thiserror::Error;

#[derive(Error, Debug)]
pub enum CommandError {
    #[error("Edit Command Err: {0}")]
    Error(EditCommand),
}

type Result<T> = std::result::Result<T, CommandError>;

#[derive(Debug, Clone)]
pub enum EditCommand {
    MoveLeft,
    MoveDown,
    MoveUp,
    MoveRight,
}
impl fmt::Display for EditCommand {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "<{}>", match self {
            EditCommand::MoveLeft => "MoveLeft",
            EditCommand::MoveDown => "MoveDown",
            EditCommand::MoveUp => "MoveUp",
            EditCommand::MoveRight => "MoveRight",
        })
    }
}

/// Can store and execute a list of commands
///
/// * `edit_cmds`: Vec of edit commands
#[derive(Default, Debug, Clone)]
pub struct Command {
    edit_cmds: Vec<EditCommand>,
}

impl Command {
    pub fn new(edit_cmds: Vec<EditCommand>) -> Self {
        Self { edit_cmds }
    }

    /// Runs list of commands held in structure
    pub fn run(self) -> Result<()> {
        for cmd in self.edit_cmds {
            match cmd {
                EditCommand::MoveLeft => {}
                EditCommand::MoveDown => {}
                EditCommand::MoveUp => {}
                EditCommand::MoveRight => {}
            }
        }
        Ok(())
    }
}
