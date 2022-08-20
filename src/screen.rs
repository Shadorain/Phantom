mod pane;
mod status;
mod workspace;

use std::collections::{HashMap, VecDeque};
use thiserror::Error;

use self::workspace::Workspace;

#[derive(Error, Debug)]
pub enum ScreenError {
    #[error("Placeholder Err")]
    Component(ComponentError),
}

#[derive(Error, Debug)]
pub enum ComponentError {}

pub type CResult<T> = std::result::Result<T, ComponentError>;
type SResult<T> = std::result::Result<T, ScreenError>;

pub trait Component {
    fn draw(&self) -> CResult<()>;
    fn update(&mut self) -> CResult<()>;
}

pub enum UpdateEvent {
    StatusBar,
    Document,
}

pub struct Screen {
    event_queue: VecDeque<UpdateEvent>,
    workspaces: HashMap<u16, Workspace>,
}

impl Screen {
    pub fn new() -> Self {
        Self {
            event_queue: VecDeque::new(),
            workspaces: HashMap::new(),
        }
    }
    pub fn send_update(&mut self, event: UpdateEvent) {
        self.event_queue.push_back(event);
    }
    pub fn update(&mut self) -> SResult<()> {
        Ok(())
    }
}
