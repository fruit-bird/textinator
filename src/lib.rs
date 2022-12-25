//! # MoCkiNg
//!
//! A CLI that converts text into the SpongeBob mocking case

// (done) use rand to choose which chars to convert
// (done) have it copy the output to clipboard
// (done) maybe even input from clipboard
// (done) env var that decides wether or not to output result to clipboard or stdout
// (pain) convert String to &str in the end
// (good) make this CLI friendly
//     - if command takes no args          --> clipboard copy
//     - if commands takes arg (string)    --> output to terminal
//     + remove need for env var
//     + takes --paste flag to copy the output to the clipboard

use anyhow::Result;
use arboard::Clipboard;
use clap::{arg, command, ArgMatches};
use rand::Rng;
use std::fmt::Display;

#[derive(Default)]
pub struct Mocker(String);

impl Mocker {
    pub fn new(input: impl AsRef<str>) -> Result<Self> {
        let output = Self::mocking_spongebob_case(input);
        Ok(Self(output))
    }

    /// Takes the input from the clipboard
    pub fn new_from_clipboard() -> Result<Self> {
        let mut clipboard = Clipboard::new()?;

        let input = clipboard.get_text().unwrap_or_default();
        let output = Self::mocking_spongebob_case(input);

        Ok(Self(output))
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
            .collect()
    }
}

impl Display for Mocker {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

fn cli_builder() -> ArgMatches {
    let matches = command!()
        .about("Converts text into MoCkiNg case")
        .arg(arg!(<STRING> "Text to convert").required(false))
        .arg(arg!(-c --clipboard "Converts text from the clipboard").required(false))
        .arg(arg!(-p --paste "Pastes the conversion to the clipboard").required(false))
        // this should copy og text into some file somewhere
        // .arg(arg!(-r --reverse "Converts text back to original state").required(false))
        .get_matches();
    matches
}

pub fn run() -> Result<()> {
    let matches = cli_builder();

    let mut output = match matches.get_one::<String>("STRING") {
        Some(text) => Mocker::new(text)?,
        None => Mocker("No input given.\nUse --help for more info".to_string()), // workaround, there are better ways
    };

    if matches.get_flag("clipboard") {
        output = Mocker::new_from_clipboard()?;
    }

    println!("\n{}\n", output);

    if matches.get_flag("paste") {
        Clipboard::new()?.set_text(&output.0)?;
        println!("Your MoCkiNg text has been copied into your clipboard!");
    }

    Ok(())
}
