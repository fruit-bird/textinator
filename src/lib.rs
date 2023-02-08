// (done) use rand to choose which chars to convert
// (done) have it copy the output to clipboard
// (done) maybe even input from clipboard
// (done) env var that decides wether or not to output result to clipboard or stdout
// (pain) convert String to &str in the end
// (good) make this CLI friendly
//      x if command takes no args          --> clipboard copy
//      + if commands takes arg (string)    --> output to terminal
//      + remove need for env var
//      + takes --paste flag to copy the output to the clipboard
// (done) have error messages be more expressive and put them in an error enum
// (done) make Mocker a unit struct, as there is no need to store the conversion string

use anyhow::Result;
use arboard::Clipboard;
use clap::ArgMatches;
use std::{
    fs::{self, File},
    io::Write,
    path::Path,
};

/// Trait with all methods needed to convert text into a specific format
///
/// Meant to be used with Unit structs, that represent a specific text conversion
pub trait TextConverter {
    fn converter(input: impl AsRef<str>) -> String;

    /// Converts given input
    fn new_from_text(input: impl AsRef<str>) -> String {
        Self::converter(input)
    }

    /// Converts clipboard contents
    fn new_from_clipboard() -> String {
        let mut clipboard = Clipboard::new().expect("Could not fetch the clipboard contents");
        let input = clipboard.get_text().unwrap_or_default();

        Self::converter(input)
    }

    /// Converts file contents, and copies the conversion into a new file
    fn new_from_file(path: impl AsRef<Path>) -> String {
        let input = fs::read_to_string(path.as_ref()).expect("Failed to read file contents");
        let output = Self::converter(input);
        let new_path = path
            .as_ref()
            .to_str()
            .unwrap()
            .split('.')
            .take(1)
            .collect::<String>()
            + "_converted.md";

        File::create(new_path)
            .expect("Failed to create the output file")
            .write_all(output.as_bytes())
            .expect("Failed to write to the output file");

        output
    }
}

/// Trait with functions needed to make a CLI for a given TextConverter
pub trait TextConverterCLI: TextConverter {
    /// CLI Builder
    fn cli() -> ArgMatches;

    /// Logic of the CLI
    fn run() -> Result<()>;
}
