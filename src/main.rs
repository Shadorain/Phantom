mod terminal;
mod editor;

use editor::Editor;

pub const NAME: &str =  env!("CARGO_PKG_NAME");
pub const AUTHOR: &str = env!("CARGO_PKG_AUTHORS");
pub const VERSION: &str = env!("CARGO_PKG_VERSION");
pub const DESCRIPTION: &str = env!("CARGO_PKG_DESCRIPTION");
pub const REPOSITORY: &str = env!("CARGO_PKG_REPOSITORY");

fn main() {
    let editor = Editor::new(); //.expect("Failed to initialize editor");
}
