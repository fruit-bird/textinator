//! # Subcommands
//! Subcommands for the Textinator CLI

mod mock;
mod morse;
mod reverse;

pub use mock::MockCommand;
pub use morse::MorseCommand;
pub use reverse::ReverseCommand;