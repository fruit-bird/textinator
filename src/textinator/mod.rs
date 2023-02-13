//! # Textinator
//! A text conversion CLI

// text converters
mod mock;
mod morse;
mod reverse;

// cli stuff
mod builder_macro;
mod cli;

pub use cli::Textinator;
