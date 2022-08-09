use super::command::{Command, EditCommand};

use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};
use std::{collections::HashMap, fmt};

/// A keybind is a mapping of a key to a command.
#[derive(Debug)]
pub struct Keybind {
    key: KeyEvent,
    command: Command,
}

impl From<KeyEvent> for Keybind {
    fn from(key: KeyEvent) -> Keybind {
        Keybind {
            key,
            command: Command::default(),
        }
    }
}

impl fmt::Display for Keybind {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let m = match self.key.modifiers {
            KeyModifiers::CONTROL => 'C',
            KeyModifiers::SHIFT => 'S',
            KeyModifiers::ALT => 'M',
            _ => '\0',
        };
        let s;

        write!(f, "<{}{}{}>", m, if m != '\0' { '-' } else { '\0' },
            match self.key.code {
                KeyCode::Backspace => "Backspace",
                KeyCode::Enter => "Enter",
                KeyCode::Left => "Left",
                KeyCode::Right => "Right",
                KeyCode::Up => "Up",
                KeyCode::Down => "Down",
                KeyCode::Home => "Home",
                KeyCode::End => "End",
                KeyCode::PageUp => "PageUp",
                KeyCode::PageDown => "PageDown",
                KeyCode::Tab => "Tab",
                KeyCode::BackTab => "BackTab",
                KeyCode::Delete => "Delete",
                KeyCode::Insert => "Insert",
                KeyCode::Null => return Err(fmt::Error),
                KeyCode::F(n) => { s = format!("F{}", n); &s },
                KeyCode::Char(c) => { s = format!("{}", c); &s },
                KeyCode::Esc => "Esc",
            }
        )
    }
}

pub struct Keybinds {
    pub keybinds: HashMap<KeyEvent, Keybind>,
}

impl Keybinds {
    pub fn new() -> Self {
        Self {
            keybinds: HashMap::new(),
        }
    }

    /// Adds a keybind to the keybinds hashmap.
    ///
    /// * `key`: Key to bind to
    /// * `command`: Command to bind to key
    pub fn add(&mut self, key: KeyEvent, command: Command) {
        self.keybinds.insert(key, Keybind { key, command });
    }

    /// Retrieves the keybind for the given key.
    /// Returns None if no keybind exists for the given key.
    ///
    /// * `key`: Key to retrieve keybind for
    pub fn get(&mut self, key: KeyEvent) -> Option<Command> {
        if let Some(k) = self.keybinds.get(&key) {
            return Some(k.command.clone());
        };
        None
    }
}

pub fn set_default_keybind() -> Keybinds {
    use KeyCode::*;

    let mut keybinds = Keybinds::new();
    keybinds.add(
        KeyEvent::new(Left, KeyModifiers::NONE),
        Command::new(vec![EditCommand::MoveLeft]),
    );
    keybinds.add(
        KeyEvent::new(Down, KeyModifiers::NONE),
        Command::new(vec![EditCommand::MoveDown]),
    );
    keybinds.add(
        KeyEvent::new(Up, KeyModifiers::NONE),
        Command::new(vec![EditCommand::MoveUp]),
    );
    keybinds.add(
        KeyEvent::new(Right, KeyModifiers::NONE),
        Command::new(vec![EditCommand::MoveRight]),
    );

    keybinds
}
