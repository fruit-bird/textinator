use clap::{builder::Str, value_parser, Args, Command};
use std::path::PathBuf;

/// Morse code converter
#[derive(Debug, Args)]
pub struct DefaultCommand {
    /// Text to convert
    #[arg(conflicts_with = "from_clipboard", value_name = "STRING", value_parser = value_parser!(String))]
    string_to_convert: Option<String>,
    /// Convert text from a file
    #[arg(
        short = 'f',
        long = "file",
        conflicts_with = "string_to_convert",
        value_name = "FILE",
        value_parser = value_parser!(PathBuf)
    )]
    from_file: Option<PathBuf>,
    /// Convert text in clipboard
    #[arg(short = 'c', long = "clipboard", conflicts_with = "from_file")]
    from_clipboard: bool,
    /// Paste conversion to clipboard
    #[arg(short, long)]
    paste: bool,
}

pub fn create_command(
    name: impl Into<Str>,
    _conversion_function: impl FnOnce(&str) -> String,
) -> Command {
    let cli = Command::new(name);
    DefaultCommand::augment_args(cli)
}
