use std::{collections::{VecDeque, HashMap}, time::Instant};
use thiserror::Error;

#[derive(Error, Debug)]
pub enum ScreenError {
    #[error("Placeholder Err")]
    Screen,
}

type Result<T> = std::result::Result<T, ScreenError>;

trait Status {
    fn show(&self) -> String;
}

/// Status bar component
///
/// * `msg`: An optional generic Status trait type to be displayed
/// * `duration`: determines duration of item being displayed
struct StatusBar {
    msg: Option<Box<dyn Status>>,
    start: Instant,
    duration: u128,
}

impl StatusBar {
    fn new() -> Self {
        Self {
            msg: None,
            start: Instant::now(),
            duration: 0,
        }
    }
    /// Determines if the Status duration has elapsed
    fn is_elapsed(&self) -> bool {
        self.start.elapsed().as_millis() > self.duration
    }
}

pub enum UpdateEvent {
    StatusBar,
    Document,
}

pub struct Screens {
    event_queue: VecDeque<UpdateEvent>,
    status_bar: Option<StatusBar>,
    screen: HashMap<u16, Screen>,
}

struct Screen {
    event_queue: VecDeque<UpdateEvent>,
}

impl Screens {
    pub fn new() -> Self {
        Self {
            event_queue: VecDeque::new(),
            status_bar: Some(StatusBar::new()),
            screen: HashMap::new(),
        }
    }
    pub fn send_update(&mut self, event: UpdateEvent) {
        self.event_queue.push_back(event);
    }
    pub fn update(&mut self) -> Result<()> {
        
        Ok(())
    }
}
