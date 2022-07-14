mod terminal;
use crossterm::Result;
use terminal::Terminal;

pub const NAME: &str =  env!("CARGO_PKG_NAME");
pub const AUTHOR: &str = env!("CARGO_PKG_AUTHORS");
pub const VERSION: &str = env!("CARGO_PKG_VERSION");
pub const DESCRIPTION: &str = env!("CARGO_PKG_DESCRIPTION");
pub const REPOSITORY: &str = env!("CARGO_PKG_REPOSITORY");

fn main() {
    let mut terminal = Terminal::new().expect("Failed to initialize terminal");
    terminal.startup();
    terminal.cleanup();
}
