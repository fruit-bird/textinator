//! # MoCkiNg
//!
//! A CLI that converts text into the SpongeBob mocking case

use anyhow::Result;
use arboard::Clipboard;
use clap::{arg, command, ArgMatches};
use rand::Rng;
use text_converter::{TextConverter, TextConverterCLI};
use thiserror::Error;

#[derive(Error, Debug)]
enum MockerError {
    #[error("No input given")]
    NoInput,
    #[error("Error reading the file")]
    FileReading,
}

pub struct Mocker;

impl TextConverter for Mocker {
    fn converter(input: impl AsRef<str>) -> String {
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
            .collect()
    }
}

impl TextConverterCLI for Mocker {
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

    fn run() -> Result<()> {
        let matches = Self::cli();

        let output = if matches.get_flag("clipboard") {
            Mocker::new_from_clipboard()
        }
        // get_flag doesn't work here, but contains_id does for some reason
        // will do for now
        else if matches.contains_id("file") {
            match matches.get_one::<String>("file") {
                Some(path) => Self::new_from_file(path),
                None => return Err(MockerError::FileReading.into()),
            }
        } else {
            match matches.get_one::<String>("STRING") {
                Some(text) => Mocker::new_from_text(text),
                None => return Err(MockerError::NoInput.into()),
            }
        };

        println!("\n{}", output);

        if matches.get_flag("paste") {
            Clipboard::new()?.set_text(output)?;
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn copy() {
        let mock = Mocker::new_from_clipboard();
        dbg!(mock);
    }

    #[test]
    fn text() {
        let mock = Mocker::new_from_text("ching cheng hanji");
        dbg!(mock);
    }

    #[test]
    fn file() {
        let mock = Mocker::new_from_file("input.md");
        dbg!(&mock);
    }
}
