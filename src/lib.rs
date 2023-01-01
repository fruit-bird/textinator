//! # MoCkiNg
//!
//! A CLI that converts text into the SpongeBob mocking case

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
// (have error messages be more expressive and put them in an error enum)
// Make Mocker a unit struct, as there is no need to store the conversion string

use anyhow::Result;
use arboard::Clipboard;
use clap::{arg, command, ArgMatches};
use rand::Rng;
use std::{
    fs::{self, File},
    io::Write,
    path::Path,
};
use thiserror::Error;

#[derive(Error, Debug)]
pub enum MockerError {
    #[error("No input given")]
    NoInput,
    #[error("Error reading the file")]
    FileReading,
}

pub struct Mocker;

impl Mocker {
    /// Converts passed input into MoCkiNg case
    pub fn new(input: impl AsRef<str>) -> String {
        Self::mocking_spongebob_case(input)
    }

    /// Converts clipboard contents into MoCkiNg case
    pub fn new_from_clipboard() -> String {
        let mut clipboard = Clipboard::new().expect("Could not fetch the clipboard contents");
        let input = clipboard.get_text().unwrap_or_default();

        Self::mocking_spongebob_case(input)
    }

    /// Converts file contents into MoCkiNg case, and copies the conversion into a new file
    pub fn new_from_file(path: impl AsRef<Path>) -> Result<String> {
        let input = fs::read_to_string(&path)?;
        let output = Self::mocking_spongebob_case(input);
        let new_path = path
            .as_ref()
            .to_str()
            .unwrap()
            .split('.')
            .take(1)
            .collect::<String>()
            + "_mock.md";
        File::create(new_path)?.write_all(output.as_bytes())?;
        Ok(output)
    }

    fn mocking_spongebob_case(input: impl AsRef<str>) -> String {
        let mut rng = rand::thread_rng();

        input
            .as_ref()
            .chars()
            .map(|c| {
                if rng.gen_bool(0.55) {
                    c.to_uppercase()
                        .to_string()
                        .parse::<char>()
                        .expect("Parsing error")
                } else {
                    c
                }
            })
            .collect::<String>()
    }
}

fn cli() -> ArgMatches {
    let matches = command!()
        .about("Converts text into MoCkiNg case")
        .arg(arg!([STRING] "Text to convert").conflicts_with("clipboard"))
        .arg(arg!(-c --clipboard "Converts text from the clipboard").conflicts_with("file"))
        .arg(arg!(-p --paste "Pastes the conversion to the clipboard"))
        .arg(arg!(-f --file <path> "Converts text from a file into a new file"))
        .get_matches();
    matches
}

pub fn run() -> Result<()> {
    let matches = cli();

    let output = if matches.get_flag("clipboard") {
        Mocker::new_from_clipboard()
    }
    // get_flag doesn't work here, but contains_id does for some reason
    // will do for now
    else if matches.contains_id("file") {
        match matches.get_one::<String>("file") {
            Some(path) => Mocker::new_from_file(path)?,
            None => return Err(MockerError::FileReading.into()),
        }
    } else {
        match matches.get_one::<String>("STRING") {
            Some(text) => Mocker::new(text),
            None => return Err(MockerError::NoInput.into()),
        }
    };

    println!("\n{}", output);

    if matches.get_flag("paste") {
        Clipboard::new()?.set_text(output)?;
    }

    Ok(())
}
