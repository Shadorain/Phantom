use crate::command::EditCommand;

use std::collections::HashMap;
use crossterm::event::{KeyEvent, KeyCode, KeyModifiers};

/// A keybind is a mapping of a key to a command.
pub struct Keybind {
    key: KeyEvent,
    edit_cmds: Vec<EditCommand>,
}

pub struct Keybinds {
    pub keybinds: HashMap<KeyEvent, Keybind>,
}

impl Keybinds {
    pub fn new() -> Self {
        Self { keybinds: HashMap::new() }
    }

    /// Adds a keybind to the keybinds hash map.
    ///
    /// * `key`: Key to bind to
    /// * `edit_cmds`: Commands to bind to key
    pub fn add_keybind(&mut self, key: KeyEvent, edit_cmds: Vec<EditCommand>) {
        self.keybinds.insert(key, Keybind { key, edit_cmds });
    }

    /// Retrieves the keybind for the given key.
    /// Returns None if no keybind exists for the given key.
    ///
    /// * `key`: Key to retrieve keybind for
    pub fn get_keybind(&mut self, key: KeyEvent) -> Option<Vec<EditCommand>> {
        if let Some(c) = self.keybinds.get(&key) {
            return Some(c.edit_cmds.clone())
        };
        None
    }
}

pub fn set_default_keybind() -> Keybinds {
    use KeyCode::*;

    let mut keybinds = Keybinds::new();
    keybinds.add_keybind(KeyEvent::new(Left, KeyModifiers::NONE), vec![EditCommand::MoveLeft]);
    keybinds.add_keybind(KeyEvent::new(Down, KeyModifiers::NONE), vec![EditCommand::MoveDown]);
    keybinds.add_keybind(KeyEvent::new(Up, KeyModifiers::NONE), vec![EditCommand::MoveUp]);
    keybinds.add_keybind(KeyEvent::new(Right, KeyModifiers::NONE), vec![EditCommand::MoveRight]);

    keybinds
}
