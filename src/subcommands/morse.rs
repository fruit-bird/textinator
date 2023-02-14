use arboard::Clipboard;
use clap::Args;
use std::{collections::HashMap, path::PathBuf};
use text_converter::TextConverter;

/// Morse code converter
#[derive(Debug, Args)]
pub struct MorseCommand {
    /// Text to convert
    #[arg(conflicts_with = "from_clipboard", value_name = "STRING")]
    string_to_convert: Option<String>,
    /// Convert text from a file
    #[arg(
        short = 'f',
        long = "file",
        conflicts_with = "string_to_convert",
        value_name = "FILE"
    )]
    from_file: Option<PathBuf>,
    /// Convert text in clipboard
    #[arg(short = 'c', long = "clipboard", conflicts_with = "from_file")]
    from_clipboard: bool,
    /// Paste conversion to clipboard
    #[arg(short, long)]
    paste: bool,
}

impl MorseCommand {
    pub fn parse_args(&self) -> String {
        let output = if let Some(ref input) = self.string_to_convert {
            Self::new_from_text(input)
        } else if let Some(ref file) = self.from_file {
            Self::new_from_file(file)
        } else if self.from_clipboard {
            Self::new_from_clipboard()
        } else {
            unreachable!()
        };

        if self.paste {
            Clipboard::new()
                .expect("Error while fetching clipboard")
                .set_text(&output)
                .expect("Error while pasting to clipboard");
        }
        output
    }
}

impl TextConverter for MorseCommand {
    fn converter(input: impl AsRef<str>) -> String {
        let char_to_morse = HashMap::from([
            ('a', ".-"),
            ('b', "-..."),
            ('c', "-.-."),
            ('d', "-.."),
            ('e', "."),
            ('f', "..-."),
            ('g', "--."),
            ('h', "...."),
            ('i', ".."),
            ('j', ".---"),
            ('k', "-.-"),
            ('l', ".-.."),
            ('m', "--"),
            ('n', "-."),
            ('o', "---"),
            ('p', ".--."),
            ('q', "--.-"),
            ('r', ".-."),
            ('s', "..."),
            ('t', "-"),
            ('u', "..-"),
            ('v', "...-"),
            ('w', ".--"),
            ('x', "-..-"),
            ('y', "-.--"),
            ('z', "--.."),
            (' ', "/"),
            ('0', "-----"),
            ('1', ".----"),
            ('2', "..---"),
            ('3', "...--"),
            ('4', "....-"),
            ('5', "....."),
            ('6', "-...."),
            ('7', "--..."),
            ('8', "---.."),
            ('9', "----."),
        ]);

        input
            .as_ref()
            .to_lowercase()
            .chars()
            .map(|c| {
                char_to_morse
                    .get(&c)
                    .expect("Error converting the character to morse")
                    .to_string()
                    + " "
            })
            .collect()
    }
}
