//! # Phantom Editor
//! > *A simple yet complex, shadow fast, rust text editor, to run in the shadows*
//!
//! ## Example
//!
//! ```rust,no_run
//! use phantom::{Phantom, PResult};
//!
//! let mut phantom: Phantom = Phantom::new().expect("Phantom error: {0}");
//! phantom.run().expect("Panic... phantom run failed");
//!
//! ```
mod buffer;
mod document;
mod input;
mod log;
mod screen;
mod terminal;

mod phantom;
pub use phantom::{PResult, Phantom};
