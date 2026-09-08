//! Use-case layer shared by CLI and TUI.

mod context;
mod services;

pub use context::{AppContext, AppContextBuilder};
pub use ptero_wings::{ConsoleEvent, WingsConsole};
pub use services::*;
