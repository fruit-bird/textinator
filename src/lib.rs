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
//     + takes -c flag to copy the output to the clipboard

use anyhow::Result;
use clap::{arg, command, ArgMatches};
use clipboard::{ClipboardContext, ClipboardProvider};
use rand::Rng;
use std::fmt::Display;

pub struct Mocker(String);

impl Mocker {
    pub fn new(input: impl AsRef<str>) -> Result<Self> {
        let output = Self::mocking_spongebob_case(input)?;
        Ok(Self(output))
    }

    /// Copies the input from the clipboard and pastes it back into the clipboard
    pub fn new_clipboard() -> Result<Self> {
        let mut ctx = ClipboardContext::new().unwrap();
        let input = ctx.get_contents().unwrap_or_default();

        let output = Self::mocking_spongebob_case(input)?;
        ctx.set_contents(output.to_string()).unwrap_or_default();

        Ok(Self(output))
    }

    fn mocking_spongebob_case(input: impl AsRef<str>) -> Result<String> {
        let mut rng = rand::thread_rng();

        let output = input
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
            .collect::<String>();

        Ok(output)
    }
}

impl Display for Mocker {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

fn cli_builder() -> ArgMatches {
    let matches = command!()
        .author("fruit-bird")
        .about("Converts text into MoCkiNg case")
        .arg(arg!(-i --input <STRING> "Text to convert. Must be in double quotes").required(true)) // this is optional when -c is used, unless i change it to do clipboard id not given text
        .arg(arg!(-c --clipboard "Copies conversion to clipboard").required(false))
        // .arg(arg!(-r --reverse "Converts text back to original state").required(false)) // this should copy og text into some file somewhere
        .get_matches();
    matches
}

pub fn run() -> Result<()> {
    let matches = cli_builder();

    let output = match matches.get_one::<String>("input") {
        Some(text) => Mocker::new(text)?,
        None => Mocker::new_clipboard()?,
    };

    if matches.get_flag("clipboard") {
        ClipboardContext::new()
            .unwrap()
            .set_contents(output.0.to_string())
            .unwrap_or_default();
    }

    println!("{}", output);

    Ok(())
}
