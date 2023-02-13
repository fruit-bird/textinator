use arboard::Clipboard;
use clap::Args;
use rand::Rng;
use std::path::PathBuf;
use text_converter::TextConverter;

#[derive(Debug, Args)]
pub struct MockCommand {
    /// Text to convert
    #[arg(conflicts_with = "from_clipboard")]
    string_to_convert: Option<String>,
    /// Convert text from a file
    #[arg(short = 'f', long = "file", conflicts_with = "string_to_convert")]
    from_file: Option<PathBuf>,
    /// Convert text in clipboard
    #[arg(short = 'c', long = "clipboard", conflicts_with = "from_file")]
    from_clipboard: bool,
    /// Paste conversion to clipboard
    #[arg(short, long)]
    paste: bool,
}

impl MockCommand {
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

impl TextConverter for MockCommand {
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
