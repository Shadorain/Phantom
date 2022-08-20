use std::time::Instant;

use super::{CResult, Component};

trait Status {
    fn show(&self) -> String;
}

/// Status bar component
///
/// * `msg`: An optional generic Status trait type to be displayed
/// * `duration`: determines duration of item being displayed
pub struct StatusBar {
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

impl Component for StatusBar {
    fn draw(&self) -> CResult<()> {
        Ok(())
    }
    fn update(&mut self) -> CResult<()> {
        Ok(())
    }
}
