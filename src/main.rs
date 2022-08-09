mod terminal;
mod editor;
mod document;
mod buffer;
mod input;

use editor::Editor;

pub const NAME: &str =  env!("CARGO_PKG_NAME");
pub const AUTHOR: &str = env!("CARGO_PKG_AUTHORS");
pub const VERSION: &str = env!("CARGO_PKG_VERSION");
pub const DESCRIPTION: &str = env!("CARGO_PKG_DESCRIPTION");
pub const REPOSITORY: &str = env!("CARGO_PKG_REPOSITORY");

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut editor = Editor::new()?; //.expect("Failed to initialize editor");
    editor.run()?;
    Ok(())
}
