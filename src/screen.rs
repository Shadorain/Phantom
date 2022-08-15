use thiserror::Error;
use std::{collections::VecDeque, time::Instant};

#[derive(Error, Debug)]
pub enum ScreenError {
    #[error("Placeholder Err")]
    Screen,
}

trait Status {
    fn show(&self) -> String;
}

/// Status bar component 
///
/// * `msg`: An optional generic Status trait type to be displayed
/// * `duration`: determines duration of item being displayed
struct StatusBar {
    msg: Option<Box<dyn Status>>,
    duration: Instant,
}

impl StatusBar {
    fn new() -> Self {
        Self {
            msg: None,
            duration: Instant::now(),
        }
    }
    /// Determines if the Status duration has elapsed
    ///
    /// * `ms`: time in milliseconds since
    fn is_elapsed(&self, ms: u128) -> bool {
        self.duration.elapsed().as_millis() > ms
    }
}

pub enum UpdateEvent {
    StatusBar,
    Document,
}

pub struct Screen {
    event_queue: VecDeque<UpdateEvent>,
    status_bar: Option<StatusBar>,
}

impl Screen {
    pub fn new() -> Self {
        Self {
            event_queue: VecDeque::new(),
            status_bar: Some(StatusBar::new()),
        }
    }
    pub fn update(&self, event: UpdateEvent) -> Result<()> {
        Ok(())
    }
} 
